use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{
  extract::Query, response::Redirect, routing::get, Router,
};
use monitor_client::entities::user::User;
use mungos::mongodb::bson::doc;
use serde::Deserialize;
use serror::AppError;

use crate::state::StateExtension;

pub mod client;

pub fn router() -> Router {
  Router::new()
    .route(
      "/login",
      get(|state: StateExtension| async move {
        Redirect::to(
          &state
            .google_auth
            .as_ref()
            .unwrap()
            .get_login_redirect_url()
            .await,
        )
      }),
    )
    .route(
      "/callback",
      get(|state, query| async {
        let redirect = callback(state, query).await?;
        Result::<_, AppError>::Ok(redirect)
      }),
    )
}

#[derive(Deserialize)]
struct CallbackQuery {
  state: Option<String>,
  code: Option<String>,
  error: Option<String>,
}

async fn callback(
  state: StateExtension,
  Query(query): Query<CallbackQuery>,
) -> anyhow::Result<Redirect> {
  let client = state.google_auth.as_ref().unwrap();
  if let Some(error) = query.error {
    return Err(anyhow!("auth error from google: {error}"));
  }
  if !client
    .check_state(
      &query
        .state
        .ok_or(anyhow!("callback query does not contain state"))?,
    )
    .await
  {
    return Err(anyhow!("state mismatch"));
  }
  let token = client
    .get_access_token(
      &query
        .code
        .ok_or(anyhow!("callback query does not contain code"))?,
    )
    .await?;
  let google_user = client.get_google_user(&token.id_token)?;
  let google_id = google_user.id.to_string();
  let user = state
    .db
    .users
    .find_one(doc! { "google_id": &google_id }, None)
    .await
    .context("failed at find user query from mongo")?;
  let jwt = match user {
    Some(user) => state
      .jwt
      .generate(user.id)
      .context("failed to generate jwt")?,
    None => {
      let ts = unix_timestamp_ms() as i64;
      let no_users_exist =
        state.db.users.find_one(None, None).await?.is_none();
      let user = User {
        username: google_user
          .email
          .split('@')
          .collect::<Vec<&str>>()
          .first()
          .unwrap()
          .to_string(),
        avatar: google_user.picture.into(),
        google_id: google_id.into(),
        enabled: no_users_exist,
        admin: no_users_exist,
        create_server_permissions: no_users_exist,
        create_build_permissions: no_users_exist,
        updated_at: ts,
        ..Default::default()
      };
      let user_id = state
        .db
        .users
        .insert_one(user, None)
        .await
        .context("failed to create user on mongo")?
        .inserted_id
        .as_object_id()
        .context("inserted_id is not ObjectId")?
        .to_string();
      state
        .jwt
        .generate(user_id)
        .context("failed to generate jwt")?
    }
  };
  let exchange_token = state.jwt.create_exchange_token(jwt).await;
  Ok(Redirect::to(&format!(
    "{}?token={exchange_token}",
    state.config.host
  )))
}
