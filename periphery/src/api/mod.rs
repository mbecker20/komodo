use axum::{routing::get, Router};

mod accounts;
mod build;
mod container;
mod git;
mod network;
mod stats;

pub fn router() -> Router {
    Router::new()
        .route("/accounts/:account_type", get(accounts::get_accounts))
        .nest("/container", container::router())
        .nest("/network", network::router())
        .nest("/stats", stats::router())
        .nest("/git", git::router())
        .nest("/build", build::router())
}
