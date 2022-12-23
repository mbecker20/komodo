use std::sync::Arc;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Router,
};
use types::CoreConfig;

mod github;
mod jwt;
mod local;
mod secret;

pub use self::jwt::{JwtClaims, JwtClient, JwtExtension, RequestUser, RequestUserExtension};

pub fn router(config: &CoreConfig) -> Router {
    Router::new()
        .nest("/local", local::router())
        .nest("/github", github::router(config))
        .nest("/secret", secret::router())
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
