use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Context};
use async_timing_util::{
    get_timelength_in_ms, unix_timestamp_ms, Timelength,
};
use axum::{body::Body, http::Request, Extension};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tokio::sync::Mutex;

use crate::{config::CoreConfig, state::State};

use super::random_string;

type ExchangeTokenMap = Mutex<HashMap<String, (String, u128)>>;

pub type RequestUser = Arc<InnerRequestUser>;
pub type RequestUserExtension = Extension<RequestUser>;

#[derive(Default)]
pub struct InnerRequestUser {
    pub id: String,
    pub username: String,
    pub is_admin: bool,
    pub create_server_permissions: bool,
    pub create_build_permissions: bool,
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
    pub fn new(config: &CoreConfig) -> JwtClient {
        let key = Hmac::new_from_slice(random_string(40).as_bytes())
            .expect("failed at taking HmacSha256 of jwt secret");
        JwtClient {
            key,
            valid_for_ms: get_timelength_in_ms(
                config.jwt_valid_for.to_string().parse().unwrap(),
            ),
            exchange_tokens: Default::default(),
        }
    }

    pub fn generate(
        &self,
        user_id: String,
    ) -> anyhow::Result<String> {
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

    pub async fn create_exchange_token(&self, jwt: String) -> String {
        let exchange_token = random_string(40);
        self.exchange_tokens.lock().await.insert(
            exchange_token.clone(),
            (
                jwt,
                unix_timestamp_ms()
                    + get_timelength_in_ms(Timelength::OneMinute),
            ),
        );
        exchange_token
    }

    pub async fn redeem_exchange_token(
        &self,
        exchange_token: &str,
    ) -> anyhow::Result<String> {
        let (jwt, valid_until) = self
            .exchange_tokens
            .lock()
            .await
            .remove(exchange_token)
            .ok_or(anyhow!("invalid exchange token: unrecognized"))?;
        if unix_timestamp_ms() < valid_until {
            Ok(jwt)
        } else {
            Err(anyhow!("invalid exchange token: expired"))
        }
    }
}

impl State {
    pub async fn authenticate_check_enabled(
        &self,
        req: &Request<Body>,
    ) -> anyhow::Result<RequestUser> {
        let jwt = req
            .headers()
            .get("authorization")
            .ok_or(anyhow!(
                "no authorization header provided. must be Bearer <jwt_token>"
            ))?
            .to_str()?
            .replace("Bearer ", "")
            .replace("bearer ", "");
        let user = self
            .auth_jwt_check_enabled(&jwt)
            .await
            .context("failed to authenticate jwt")?;
        Ok(user)
    }

    pub async fn auth_jwt_check_enabled(
        &self,
        jwt: &str,
    ) -> anyhow::Result<RequestUser> {
        let claims: JwtClaims = jwt
            .verify_with_key(&self.jwt.key)
            .context("failed to verify claims")?;
        if claims.exp > unix_timestamp_ms() {
            let user = self
                .db
                .users
                .find_one_by_id(&claims.id)
                .await?
                .ok_or(anyhow!(
                    "did not find user with id {}",
                    claims.id
                ))?;
            if user.enabled {
                let user = InnerRequestUser {
                    id: claims.id,
                    username: user.username,
                    is_admin: user.admin,
                    create_server_permissions: user
                        .create_server_permissions,
                    create_build_permissions: user
                        .create_build_permissions,
                };
                Ok(user.into())
            } else {
                Err(anyhow!("user not enabled"))
            }
        } else {
            Err(anyhow!("token has expired"))
        }
    }
}
