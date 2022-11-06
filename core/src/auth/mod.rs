use axum::Router;
use types::CoreConfig;

mod github;
mod jwt;
mod local;

pub use self::jwt::{JwtClaims, JwtClient, JwtExtension};

pub fn router(config: &CoreConfig) -> Router {
    Router::new()
        .nest("/local", local::router())
        .nest("/github", github::router(config))
}
