use axum::Router;

mod github;

pub fn router() -> Router {
    Router::new().nest("/github", github::router())
}
