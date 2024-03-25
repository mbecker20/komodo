use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{
  extract::Query, response::Redirect, routing::get, Router,
};
use monitor_client::entities::user::User;
use mungos::mongodb::bson::doc;
use serde::Deserialize;
use serror::AppError;

use crate::{config::core_config, db::db_client};

use self::client::google_oauth_client;

use super::{jwt::jwt_client, RedirectQuery, STATE_PREFIX_LENGTH};

pub mod client;

pub fn router() -> Router {
  Router::new()
    .route(
      "/login",
      get(|Query(query): Query<RedirectQuery>| async move {
        Redirect::to(
          &google_oauth_client()
            .as_ref()
            // OK: its not mounted unless the client is populated
            .unwrap()
            .get_login_redirect_url(query.redirect)
            .await,
        )
      }),
    )
    .route(
      "/callback",
      get(|query| async {
        let redirect = callback(query).await?;
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
  Query(query): Query<CallbackQuery>,
) -> anyhow::Result<Redirect> {
  let client = google_oauth_client().as_ref().unwrap();
  if let Some(error) = query.error {
    return Err(anyhow!("auth error from google: {error}"));
  }
  let state = query
    .state
    .context("callback query does not contain state")?;
  if !client.check_state(&state).await {
    return Err(anyhow!("state mismatch"));
  }
  let token = client
    .get_access_token(
      &query.code.context("callback query does not contain code")?,
    )
    .await?;
  let google_user = client.get_google_user(&token.id_token)?;
  let google_id = google_user.id.to_string();
  let db_client = db_client().await;
  let user = db_client
    .users
    .find_one(doc! { "google_id": &google_id }, None)
    .await
    .context("failed at find user query from mongo")?;
  let jwt = match user {
    Some(user) => jwt_client()
      .generate(user.id)
      .context("failed to generate jwt")?,
    None => {
      let ts = unix_timestamp_ms() as i64;
      let no_users_exist =
        db_client.users.find_one(None, None).await?.is_none();
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
      let user_id = db_client
        .users
        .insert_one(user, None)
        .await
        .context("failed to create user on mongo")?
        .inserted_id
        .as_object_id()
        .context("inserted_id is not ObjectId")?
        .to_string();
      jwt_client()
        .generate(user_id)
        .context("failed to generate jwt")?
    }
  };
  let exchange_token = jwt_client().create_exchange_token(jwt).await;
  let redirect = &state[STATE_PREFIX_LENGTH..];
  let redirect_url = if redirect.is_empty() {
    format!("{}?token={exchange_token}", core_config().host)
  } else {
    let splitter = if redirect.contains('?') { '&' } else { '?' };
    format!("{}{splitter}token={exchange_token}", redirect)
  };
  Ok(Redirect::to(&redirect_url))
}
