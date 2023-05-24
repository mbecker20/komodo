use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::{get, post},
    Extension, Json, Router,
};
use helpers::handle_anyhow_error;
use serde::{Deserialize, Serialize};
use types::CoreConfig;
use typeshare::typeshare;

mod github;
mod google;
mod jwt;
mod local;
mod secret;

use crate::state::StateExtension;

pub use self::jwt::{JwtClaims, JwtClient, JwtExtension, RequestUser, RequestUserExtension};

#[typeshare]
#[derive(Serialize)]
struct LoginOptions {
    local: bool,
    github: bool,
    google: bool,
}

pub fn router(config: &CoreConfig) -> Router {
    let mut router = Router::new()
        .route(
            "/options",
            get(|Extension(state): StateExtension| async move {
                Json(LoginOptions {
                    local: state.config.local_auth,
                    github: state.config.github_oauth.enabled
                        && state.config.github_oauth.id.len() > 0
                        && state.config.github_oauth.secret.len() > 0,
                    google: state.config.google_oauth.enabled
                        && state.config.google_oauth.id.len() > 0
                        && state.config.google_oauth.secret.len() > 0,
                })
            }),
        )
        .route(
            "/exchange",
            post(|jwt, body| async {
                exchange_for_jwt(jwt, body)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .nest("/secret", secret::router());

    if config.local_auth {
        router = router.nest("/local", local::router());
    }

    if config.github_oauth.enabled
        && config.github_oauth.id.len() > 0
        && config.github_oauth.secret.len() > 0
    {
        router = router.nest("/github", github::router(config));
    }

    if config.google_oauth.enabled
        && config.google_oauth.id.len() > 0
        && config.google_oauth.secret.len() > 0
    {
        router = router.nest("/google", google::router(config));
    }

    router
}

#[typeshare]
#[derive(Deserialize)]
struct TokenExchangeBody {
    token: String,
}

async fn exchange_for_jwt(
    Extension(jwt): JwtExtension,
    Json(body): Json<TokenExchangeBody>,
) -> anyhow::Result<String> {
    let jwt = jwt.redeem_exchange_token(&body.token)?;
    Ok(jwt)
}

pub async fn auth_request(
    mut req: Request<Body>,
    next: Next<Body>,
) -> Result<Response, (StatusCode, String)> {
    let jwt_client = req.extensions().get::<Arc<JwtClient>>().ok_or((
        StatusCode::UNAUTHORIZED,
        "failed to get jwt client extension".to_string(),
    ))?;
    let user = jwt_client
        .authenticate_check_enabled(&req)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("{e:#?}")))?;
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
