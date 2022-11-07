use std::sync::Arc;

use anyhow::{anyhow, Context};
use async_timing_util::{get_timelength_in_ms, unix_timestamp_ms, Timelength};
use axum::{body::Body, http::Request, Extension};
use db::DbClient;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use mungos::{Deserialize, Serialize};
use sha2::Sha256;
use types::{CoreConfig, User, UserId};

pub type JwtExtension = Extension<Arc<JwtClient>>;

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub id: UserId,
    pub iat: u128,
    pub exp: u128,
}

#[derive(Clone)]
pub struct JwtClient {
    key: Hmac<Sha256>,
    valid_for_ms: u128,
}

impl JwtClient {
    pub fn extension(config: &CoreConfig) -> JwtExtension {
        let key = Hmac::new_from_slice(config.jwt_secret.as_bytes()).unwrap();
        let client = JwtClient {
            key,
            valid_for_ms: get_timelength_in_ms(config.jwt_valid_for),
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

    pub async fn authenticate(&self, req: &Request<Body>) -> anyhow::Result<UserId> {
        let jwt = req
            .headers()
            .get("authorization")
            .ok_or(anyhow!(
                "no authorization header provided. must be Bearer <jwt_token>"
            ))?
            .to_str()?
            .replace("Bearer ", "")
            .replace("bearer ", "");
        let claims: JwtClaims = jwt
            .verify_with_key(&self.key)
            .context("failed to verify claims")?;
        if claims.exp > unix_timestamp_ms() {
            let users_collection = &req
                .extensions()
                .get::<Arc<DbClient>>()
                .ok_or(anyhow!("failed at getting db handle"))?
                .users;
            let user = users_collection
                .find_one_by_id(&claims.id)
                .await?
                .ok_or(anyhow!("did not find user with id {}", claims.id))?;
            if user.enabled {
                Ok(claims.id)
            } else {
                Err(anyhow!("user not enabled"))
            }
        } else {
            Err(anyhow!("token has expired"))
        }
    }
}
