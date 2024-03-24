use std::time::Instant;

use ::jwt::VerifyWithKey;
use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{
  extract::Request, http::HeaderMap, middleware::Next,
  response::Response, routing::post, Json, Router,
};
use axum_extra::{headers::ContentType, TypedHeader};
use monitor_client::entities::{monitor_timestamp, user::User};
use mungos::mongodb::bson::doc;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use resolver_api::Resolver;
use serror::{AppError, AuthError};
use uuid::Uuid;

pub mod jwt;

mod github;
mod google;
mod local;

use crate::{
  api::auth::AuthRequest, db::db_client, helpers::get_user,
  state::State,
};

use self::{
  github::client::github_oauth_client,
  google::client::google_oauth_client, jwt::jwt_client,
  jwt::JwtClaims,
};

pub async fn auth_request(
  headers: HeaderMap,
  mut req: Request,
  next: Next,
) -> Result<Response, AuthError> {
  let user = authenticate_check_enabled(&headers).await?;
  req.extensions_mut().insert(user);
  Ok(next.run(req).await)
}

pub fn router() -> Router {
  let mut router = Router::new().route(
    "/",
    post(|Json(request): Json<AuthRequest>| async move {
      let timer = Instant::now();
      let req_id = Uuid::new_v4();
      info!(
        "/auth request {req_id} | METHOD: {}",
        request.req_type()
      );
      let res = State.resolve_request(request, ()).await;
      if let Err(e) = &res {
        info!("/auth request {req_id} | ERROR: {e:?}");
      }
      let res = res?;
      let elapsed = timer.elapsed();
      info!("/auth request {req_id} | resolve time: {elapsed:?}");
      debug!("/auth request {req_id} | RESPONSE: {res}");
      Result::<_, AppError>::Ok((
        TypedHeader(ContentType::json()),
        res,
      ))
    }),
  );

  if github_oauth_client().is_some() {
    router = router.nest("/github", github::router())
  }

  if google_oauth_client().is_some() {
    router = router.nest("/google", google::router())
  }

  router
}

pub fn random_string(length: usize) -> String {
  thread_rng()
    .sample_iter(&Alphanumeric)
    .take(length)
    .map(char::from)
    .collect()
}

pub async fn authenticate_check_enabled(
  headers: &HeaderMap,
) -> anyhow::Result<User> {
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
      auth_jwt_get_user_id(&jwt)
        .await
        .context("failed to authenticate jwt")?
    }
    (None, Some(key), Some(secret)) => {
      // USE API KEY / SECRET
      let key = key.to_str().context("key is not str")?;
      let secret = secret.to_str().context("secret is not str")?;
      auth_api_key_get_user_id(key, secret)
        .await
        .context("failed to authenticate api key")?
    }
    _ => {
      // AUTH FAIL
      return Err(anyhow!("must attach either AUTHORIZATION header with jwt OR pass X-API-KEY and X-API-SECRET"));
    }
  };
  let user = get_user(&user_id).await?;
  if user.enabled {
    Ok(user)
  } else {
    Err(anyhow!("user not enabled"))
  }
}

pub async fn auth_jwt_get_user_id(
  jwt: &str,
) -> anyhow::Result<String> {
  let claims: JwtClaims = jwt
    .verify_with_key(&jwt_client().key)
    .context("failed to verify claims")?;
  if claims.exp > unix_timestamp_ms() {
    Ok(claims.id)
  } else {
    Err(anyhow!("token has expired"))
  }
}

pub async fn auth_jwt_check_enabled(
  jwt: &str,
) -> anyhow::Result<User> {
  let user_id = auth_jwt_get_user_id(jwt).await?;
  check_enabled(user_id).await
}

pub async fn auth_api_key_get_user_id(
  key: &str,
  secret: &str,
) -> anyhow::Result<String> {
  let key = db_client()
    .await
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
  key: &str,
  secret: &str,
) -> anyhow::Result<User> {
  let user_id = auth_api_key_get_user_id(key, secret).await?;
  check_enabled(user_id).await
}

async fn check_enabled(user_id: String) -> anyhow::Result<User> {
  let user = get_user(&user_id).await?;
  if user.enabled {
    Ok(user)
  } else {
    Err(anyhow!("user not enabled"))
  }
}
