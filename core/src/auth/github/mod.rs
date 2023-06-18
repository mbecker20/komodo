use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{extract::Query, http::StatusCode, response::Redirect, routing::get, Router};
use monitor_types::entities::user::User;
use mungos::mongodb::bson::doc;
use serde::Deserialize;

use crate::state::StateExtension;

pub mod client;

pub fn router() -> Router {
    Router::new()
        .route(
            "/login",
            get(|state: StateExtension| async move {
                let redirect_to = state
                    .github_auth
                    .as_ref()
                    .unwrap()
                    .get_login_redirect_url()
                    .await;
                Redirect::to(&redirect_to)
            }),
        )
        .route(
            "/callback",
            get(|state, query| async {
                let redirect = callback(state, query)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")))?;
                Result::<_, (StatusCode, String)>::Ok(redirect)
            }),
        )
}

#[derive(Deserialize)]
struct CallbackQuery {
    state: String,
    code: String,
}

async fn callback(
    state: StateExtension,
    Query(query): Query<CallbackQuery>,
) -> anyhow::Result<Redirect> {
    let client = state.github_auth.as_ref().unwrap();
    if !client.check_state(&query.state).await {
        return Err(anyhow!("state mismatch"));
    }
    let token = client.get_access_token(&query.code).await?;
    let github_user = client.get_github_user(&token.access_token).await?;
    let github_id = github_user.id.to_string();
    let user = state
        .db
        .users
        .find_one(doc! { "github_id": &github_id }, None)
        .await
        .context("failed at find user query from mongo")?;
    let jwt = match user {
        Some(user) => state
            .jwt
            .generate(user.id)
            .context("failed to generate jwt")?,
        None => {
            let ts = unix_timestamp_ms() as i64;
            let no_users_exist = state.db.users.find_one(None, None).await?.is_none();
            let user = User {
                username: github_user.login,
                avatar: github_user.avatar_url.into(),
                github_id: github_id.into(),
                enabled: no_users_exist,
                admin: no_users_exist,
                create_server_permissions: no_users_exist,
                create_build_permissions: no_users_exist,
                created_at: ts,
                updated_at: ts,
                ..Default::default()
            };
            let user_id = state
                .db
                .users
                .create_one(user)
                .await
                .context("failed to create user on mongo")?;
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
