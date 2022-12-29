use std::sync::Arc;

use anyhow::{anyhow, Context};
use axum::{Router, Extension, routing::get, response::Redirect, extract::Query};
use axum_oauth2::google::{GoogleOauthClient, GoogleOauthExtension};
use helpers::handle_anyhow_error;
use mungos::{Deserialize, doc};
use types::{CoreConfig, monitor_timestamp, User};

use crate::{state::StateExtension, response};

use super::JwtExtension;

pub fn router(config: &CoreConfig) -> Router {
	let client = GoogleOauthClient::new(
        config.google_oauth.id.clone(),
        config.google_oauth.secret.clone(),
        format!("{}/auth/google/callback", config.host),
        &[],
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
    state: String,
    code: String,
}

async fn callback(
    Extension(client): GoogleOauthExtension,
    Extension(jwt_client): JwtExtension,
    Extension(state): StateExtension,
    Query(query): Query<CallbackQuery>,
) -> anyhow::Result<Redirect> {
    if !client.check_state(&query.state) {
        return Err(anyhow!("state mismatch"));
    }
    let token = client.get_access_token(&query.code).await?;
    let google_user = client.get_google_user(&token.access_token)?;
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
            let user = User {
                username: google_user.email.split("@").collect::<Vec<&str>>().get(0).unwrap().to_string(),
                avatar: google_user.picture.into(),
                google_id: google_id.into(),
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