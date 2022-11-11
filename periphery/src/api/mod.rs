use axum::Router;

mod container;
mod stats;

pub fn router() -> Router {
    Router::new()
        .nest("/container", container::router())
        .nest("/stats", stats::router())
}
