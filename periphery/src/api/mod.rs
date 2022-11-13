use axum::{routing::get, Router};

mod accounts;
mod container;
mod git;
mod stats;

pub fn router() -> Router {
    Router::new()
        .route("/accounts/:account_type", get(accounts::get_accounts))
        .nest("/container", container::router())
        .nest("/stats", stats::router())
        .nest("/git", git::router())
}
