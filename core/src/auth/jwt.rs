use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Context};
use async_timing_util::{get_timelength_in_ms, unix_timestamp_ms, Timelength};
use axum::{body::Body, http::Request, Extension};
use axum_oauth2::random_string;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use mungos::{Deserialize, Serialize};
use sha2::Sha256;
use types::{CoreConfig, User};

use crate::state::State;

pub type JwtExtension = Extension<Arc<JwtClient>>;
pub type RequestUserExtension = Extension<Arc<RequestUser>>;

type ExchangeTokenMap = Mutex<HashMap<String, (String, u128)>>;

pub struct RequestUser {
    pub id: String,
    pub is_admin: bool,
    pub create_server_permissions: bool,
}

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub id: String,
    pub iat: u128,
    pub exp: u128,
}

pub struct JwtClient {
    key: Hmac<Sha256>,
    valid_for_ms: u128,
    exchange_tokens: ExchangeTokenMap,
}

impl JwtClient {
    pub fn extension(config: &CoreConfig) -> JwtExtension {
        let key = Hmac::new_from_slice(config.jwt_secret.as_bytes())
            .expect("failed at taking HmacSha256 of jwt secret");
        let client = JwtClient {
            key,
            valid_for_ms: get_timelength_in_ms(config.jwt_valid_for.to_string().parse().unwrap()),
            exchange_tokens: Default::default(),
        };
        Extension(Arc::new(client))
    }

    pub fn generate(&self, user_id: String) -> anyhow::Result<String> {
        let iat = unix_timestamp_ms();
        let exp = iat + self.valid_for_ms;
        let claims = JwtClaims {
            id: user_id,
            iat,
            exp,
        };
        let jwt = claims
            .sign_with_key(&self.key)
            .context("failed at signing claim")?;
        Ok(jwt)
    }

    pub async fn authenticate_check_enabled(
        &self,
        req: &Request<Body>,
    ) -> anyhow::Result<Arc<RequestUser>> {
        let jwt = req
            .headers()
            .get("authorization")
            .ok_or(anyhow!(
                "no authorization header provided. must be Bearer <jwt_token>"
            ))?
            .to_str()?
            .replace("Bearer ", "")
            .replace("bearer ", "");
        let state = req
            .extensions()
            .get::<Arc<State>>()
            .ok_or(anyhow!("failed at getting state handle"))?;
        let user = self
            .auth_jwt_check_enabled(&jwt, &state)
            .await
            .context("failed to authenticate jwt")?;
        Ok(Arc::new(user))
    }

    pub async fn auth_jwt_check_enabled(
        &self,
        jwt: &str,
        state: &State,
    ) -> anyhow::Result<RequestUser> {
        let claims: JwtClaims = jwt
            .verify_with_key(&self.key)
            .context("failed to verify claims")?;
        if claims.exp > unix_timestamp_ms() {
            let user = state
                .db
                .users
                .find_one_by_id(&claims.id)
                .await?
                .ok_or(anyhow!("did not find user with id {}", claims.id))?;
            if user.enabled {
                let user = RequestUser {
                    id: claims.id,
                    is_admin: user.admin,
                    create_server_permissions: user.create_server_permissions,
                };
                Ok(user)
            } else {
                Err(anyhow!("user not enabled"))
            }
        } else {
            Err(anyhow!("token has expired"))
        }
    }

    pub async fn authenticate(&self, req: &Request<Body>) -> anyhow::Result<User> {
        let jwt = req
            .headers()
            .get("authorization")
            .ok_or(anyhow!(
                "no authorization header provided. must be Bearer <jwt_token>"
            ))?
            .to_str()?
            .replace("Bearer ", "")
            .replace("bearer ", "");
        let state = req
            .extensions()
            .get::<Arc<State>>()
            .ok_or(anyhow!("failed at getting state handle"))?;
        let user = self
            .auth_jwt(&jwt, &state)
            .await
            .context("failed to authenticate jwt")?;
        Ok(user)
    }

    pub async fn auth_jwt(&self, jwt: &str, state: &State) -> anyhow::Result<User> {
        let claims: JwtClaims = jwt
            .verify_with_key(&self.key)
            .context("failed to verify claims")?;
        if claims.exp > unix_timestamp_ms() {
            let user = state
                .db
                .users
                .find_one_by_id(&claims.id)
                .await?
                .ok_or(anyhow!("did not find user with id {}", claims.id))?;
            Ok(user)
        } else {
            Err(anyhow!("token has expired"))
        }
    }

    pub fn create_exchange_token(&self, jwt: String) -> String {
        let exchange_token = random_string(40);
        self.exchange_tokens.lock().unwrap().insert(
            exchange_token.clone(),
            (
                jwt,
                unix_timestamp_ms() + get_timelength_in_ms(Timelength::OneMinute),
            ),
        );
        exchange_token
    }

    pub fn redeem_exchange_token(&self, exchange_token: &str) -> anyhow::Result<String> {
        let (jwt, valid_until) = self
            .exchange_tokens
            .lock()
            .unwrap()
            .remove(exchange_token)
            .ok_or(anyhow!("invalid exchange token: unrecognized"))?;
        if unix_timestamp_ms() < valid_until {
            Ok(jwt)
        } else {
            Err(anyhow!("invalid exchange token: expired"))
        }
    }
}
