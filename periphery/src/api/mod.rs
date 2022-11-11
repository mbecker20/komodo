use axum::{routing::get, Router};

mod accounts;
mod container;
mod stats;

pub fn router() -> Router {
    Router::new()
        .nest("/container", container::router())
        .nest("/stats", stats::router())
        .route("/accounts/:account_type", get(accounts::get_accounts))
}
