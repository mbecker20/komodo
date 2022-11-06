use axum::Router;

mod github;
mod jwt;
mod local;

pub fn router() -> Router {
    Router::new().nest("/local", local::router())
}
