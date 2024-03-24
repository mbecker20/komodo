use anyhow::{anyhow, Context};
use axum::{
  extract::Query, response::Redirect, routing::get, Router,
};
use monitor_client::entities::{monitor_timestamp, user::User};
use mungos::mongodb::bson::doc;
use serde::Deserialize;
use serror::AppError;

use crate::{config::core_config, db::db_client};

use self::client::github_oauth_client;

use super::jwt::jwt_client;

pub mod client;

pub fn router() -> Router {
  Router::new()
    .route(
      "/login",
      get(|| async {
        let redirect_to = github_oauth_client()
          .as_ref()
          // OK: the router is only mounted in case that the client is populated
          .unwrap()
          .get_login_redirect_url()
          .await;
        Redirect::to(&redirect_to)
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
  state: String,
  code: String,
}

async fn callback(
  Query(query): Query<CallbackQuery>,
) -> anyhow::Result<Redirect> {
  let client = github_oauth_client().as_ref().unwrap();
  if !client.check_state(&query.state).await {
    return Err(anyhow!("state mismatch"));
  }
  let token = client.get_access_token(&query.code).await?;
  let github_user =
    client.get_github_user(&token.access_token).await?;
  let github_id = github_user.id.to_string();
  let db_client = db_client();
  let user = db_client
    .users
    .find_one(doc! { "github_id": &github_id }, None)
    .await
    .context("failed at find user query from mongo")?;
  let jwt = match user {
    Some(user) => jwt_client()
      .generate(user.id)
      .context("failed to generate jwt")?,
    None => {
      let ts = monitor_timestamp();
      let no_users_exist =
        db_client.users.find_one(None, None).await?.is_none();
      let user = User {
        username: github_user.login,
        avatar: github_user.avatar_url.into(),
        github_id: github_id.into(),
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
  Ok(Redirect::to(&format!(
    "{}?token={exchange_token}",
    core_config().host
  )))
}
