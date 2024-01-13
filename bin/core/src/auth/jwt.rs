use std::{collections::HashMap, sync::Arc};

use anyhow::{anyhow, Context};
use async_timing_util::{
  get_timelength_in_ms, unix_timestamp_ms, Timelength,
};
use axum::{http::HeaderMap, Extension};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use monitor_client::entities::{
  config::CoreConfig, monitor_timestamp,
};
use mungos::mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tokio::sync::Mutex;

use crate::state::State;

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

impl State {
  pub async fn authenticate_check_enabled(
    &self,
    headers: &HeaderMap,
  ) -> anyhow::Result<RequestUser> {
    let user_id = match (
      headers.get("authorization"),
      headers.get("x-api-key"),
      headers.get("x-api-secret"),
    ) {
      (Some(jwt), _, _) => {
        // USE JWT
        let jwt = jwt
          .to_str()
          .context("jwt is not str")?
          .replace("Bearer ", "")
          .replace("bearer ", "");
        self
          .auth_jwt_get_user_id(&jwt)
          .await
          .context("failed to authenticate jwt")?
      }
      (None, Some(key), Some(secret)) => {
        // USE API KEY / SECRET
        let key = key.to_str().context("key is not str")?;
        let secret = secret.to_str().context("secret is not str")?;
        self
          .auth_api_key_get_user_id(key, secret)
          .await
          .context("failed to authenticate api key")?
      }
      _ => {
        // AUTH FAIL
        return Err(anyhow!("must attach either AUTHORIZATION header with jwt OR pass X-API-KEY and X-API-SECRET"));
      }
    };
    let user = self.get_user(&user_id).await?;
    if user.enabled {
      let user = InnerRequestUser {
        id: user_id,
        username: user.username,
        is_admin: user.admin,
        create_server_permissions: user.create_server_permissions,
        create_build_permissions: user.create_build_permissions,
      };
      Ok(user.into())
    } else {
      Err(anyhow!("user not enabled"))
    }
  }

  pub async fn auth_jwt_get_user_id(
    &self,
    jwt: &str,
  ) -> anyhow::Result<String> {
    let claims: JwtClaims = jwt
      .verify_with_key(&self.jwt.key)
      .context("failed to verify claims")?;
    if claims.exp > unix_timestamp_ms() {
      Ok(claims.id)
    } else {
      Err(anyhow!("token has expired"))
    }
  }

  pub async fn auth_jwt_check_enabled(
    &self,
    jwt: &str,
  ) -> anyhow::Result<RequestUser> {
    let user_id = self.auth_jwt_get_user_id(jwt).await?;
    self.check_enabled(user_id).await
  }

  pub async fn auth_api_key_get_user_id(
    &self,
    key: &str,
    secret: &str,
  ) -> anyhow::Result<String> {
    let key = self
      .db
      .api_keys
      .find_one(doc! { "key": key }, None)
      .await
      .context("failed to query db")?
      .context("no api key matching key")?;
    if key.expires != 0 && key.expires < monitor_timestamp() {
      return Err(anyhow!("api key expired"));
    }
    if bcrypt::verify(secret, &key.secret)
      .context("failed to verify secret hash")?
    {
      // secret matches
      Ok(key.user_id)
    } else {
      // secret mismatch
      Err(anyhow!("invalid api secret"))
    }
  }

  pub async fn auth_api_key_check_enabled(
    &self,
    key: &str,
    secret: &str,
  ) -> anyhow::Result<RequestUser> {
    let user_id = self.auth_api_key_get_user_id(key, secret).await?;
    self.check_enabled(user_id).await
  }

  async fn check_enabled(
    &self,
    user_id: String,
  ) -> anyhow::Result<RequestUser> {
    let user = self.get_user(&user_id).await?;
    if user.enabled {
      let user = InnerRequestUser {
        id: user_id,
        username: user.username,
        is_admin: user.admin,
        create_server_permissions: user.create_server_permissions,
        create_build_permissions: user.create_build_permissions,
      };
      Ok(user.into())
    } else {
      Err(anyhow!("user not enabled"))
    }
  }
}
