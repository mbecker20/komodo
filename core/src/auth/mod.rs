use std::sync::Arc;

use anyhow::Context;
use axum::{
    body::Body,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Router,
};
use types::{CoreConfig, UserId};

mod github;
mod jwt;
mod local;

pub use self::jwt::{JwtClaims, JwtClient, JwtExtension};

pub fn router(config: &CoreConfig) -> Router {
    Router::new()
        .nest("/local", local::router())
        .nest("/github", github::router(config))
}

pub async fn auth_request(
    mut req: Request<Body>,
    next: Next<Body>,
) -> Result<Response, (StatusCode, String)> {
    let jwt_client = req.extensions().get::<Arc<JwtClient>>().ok_or((
        StatusCode::UNAUTHORIZED,
        "failed to get jwt client extension".to_string(),
    ))?;
    let user_id: UserId = jwt_client
        .authenticate(&req)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, format!("error: {e:#?}")))?;
    req.extensions_mut().insert(user_id);
    Ok(next.run(req).await)
}
