use std::sync::Arc;

use anyhow::{anyhow, Context};
use async_timing_util::{get_timelength_in_ms, unix_timestamp_ms, Timelength};
use axum::Extension;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use mungos::{Deserialize, Serialize};
use sha2::Sha256;
use types::CoreConfig;

pub type JwtExtension = Extension<Arc<JwtClient>>;

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub id: String,
    pub iat: u128,
    pub exp: u128,
}

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

    pub fn validate(&self, jwt: &str) -> anyhow::Result<JwtClaims> {
        let claims: JwtClaims = jwt
            .verify_with_key(&self.key)
            .context("failed to verify claims")?;
        if claims.exp < unix_timestamp_ms() {
            Ok(claims)
        } else {
            Err(anyhow!("token has expired"))
        }
    }
}
