use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    routing::post,
    Extension, Json, Router,
};
use helpers::handle_anyhow_error;
use mungos::Deserialize;
use types::CoreConfig;
use typeshare::typeshare;

mod github;
mod google;
mod jwt;
mod local;
mod secret;

pub use self::jwt::{JwtClaims, JwtClient, JwtExtension, RequestUser, RequestUserExtension};

pub fn router(config: &CoreConfig) -> Router {
    Router::new()
        .route(
            "/exchange",
            post(|jwt, body| async {
                exchange_for_jwt(jwt, body)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .nest("/local", local::router())
        .nest("/github", github::router(config))
        .nest("/google", google::router(config))
        .nest("/secret", secret::router())
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
