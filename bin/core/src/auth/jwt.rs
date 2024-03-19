use std::{
  collections::HashMap,
  sync::{Arc, OnceLock},
};

use anyhow::{anyhow, Context};
use async_timing_util::{
  get_timelength_in_ms, unix_timestamp_ms, Timelength,
};
use axum::Extension;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use monitor_client::entities::config::CoreConfig;
use mungos::mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tokio::sync::Mutex;

use crate::config::core_config;

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

impl InnerRequestUser {
  pub fn procedure() -> InnerRequestUser {
    InnerRequestUser {
      id: String::from("procedure"),
      username: String::from("procedure"),
      is_admin: true,
      create_build_permissions: true,
      create_server_permissions: true,
    }
  }
}

pub fn jwt_client() -> &'static JwtClient {
  static JWT_CLIENT: OnceLock<JwtClient> = OnceLock::new();
  JWT_CLIENT.get_or_init(|| JwtClient::new(core_config()))
}

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
  pub id: String,
  pub iat: u128,
  pub exp: u128,
}

pub struct JwtClient {
  pub key: Hmac<Sha256>,
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
