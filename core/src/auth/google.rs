use std::sync::Arc;

use anyhow::{anyhow, Context};
use axum::{extract::Query, response::Redirect, routing::get, Extension, Router};
use axum_oauth2::google::{GoogleOauthClient, GoogleOauthExtension};
use helpers::handle_anyhow_error;
use mungos::mongodb::bson::doc;
use serde::Deserialize;
use types::{monitor_timestamp, CoreConfig, User};

use crate::{response, state::StateExtension};

use super::JwtExtension;

pub fn router(config: &CoreConfig) -> Router {
    let client = GoogleOauthClient::new(
        config.google_oauth.id.clone(),
        config.google_oauth.secret.clone(),
        format!("{}/auth/google/callback", config.host),
        &[
            "https://www.googleapis.com/auth/userinfo.profile",
            "https://www.googleapis.com/auth/userinfo.email",
        ],
        "monitor".to_string(),
    );
    Router::new()
        .route(
            "/login",
            get(|Extension(client): GoogleOauthExtension| async move {
                Redirect::to(&client.get_login_redirect_url())
            }),
        )
        .route(
            "/callback",
            get(|client, jwt, state, query| async {
                let redirect = callback(client, jwt, state, query)
                    .await
                    .map_err(handle_anyhow_error)?;
                response!(redirect)
            }),
        )
        .layer(Extension(Arc::new(client)))
}

#[derive(Deserialize)]
struct CallbackQuery {
    state: Option<String>,
    code: Option<String>,
    error: Option<String>,
}

async fn callback(
    Extension(client): GoogleOauthExtension,
    Extension(jwt_client): JwtExtension,
    Extension(state): StateExtension,
    Query(query): Query<CallbackQuery>,
) -> anyhow::Result<Redirect> {
    if let Some(error) = query.error {
        return Err(anyhow!("auth error from google: {error}"));
    }
    if !client.check_state(
        &query
            .state
            .ok_or(anyhow!("callback query does not contain state"))?,
    ) {
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
        Some(user) => jwt_client
            .generate(user.id)
            .context("failed to generate jwt")?,
        None => {
            let ts = monitor_timestamp();
            let no_users_exist = state.db.users.find_one(None, None).await?.is_none();
            let user = User {
                username: google_user
                    .email
                    .split("@")
                    .collect::<Vec<&str>>()
                    .get(0)
                    .unwrap()
                    .to_string(),
                avatar: google_user.picture.into(),
                google_id: google_id.into(),
                enabled: no_users_exist,
                admin: no_users_exist,
                create_server_permissions: no_users_exist,
                create_build_permissions: no_users_exist,
                created_at: ts.clone(),
                updated_at: ts,
                ..Default::default()
            };
            let user_id = state
                .db
                .users
                .create_one(user)
                .await
                .context("failed to create user on mongo")?;
            jwt_client
                .generate(user_id)
                .context("failed to generate jwt")?
        }
    };
    let exchange_token = jwt_client.create_exchange_token(jwt);
    Ok(Redirect::to(&format!(
        "{}?token={exchange_token}",
        state.config.host
    )))
}
