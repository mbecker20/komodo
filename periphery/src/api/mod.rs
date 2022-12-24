use axum::{routing::get, Router};
use helpers::docker::DockerClient;
use types::PeripheryConfig;

mod accounts;
mod build;
mod container;
mod git;
mod image;
mod network;
mod stats;

pub fn router(config: &PeripheryConfig) -> Router {
    Router::new()
        .route("/health", get(|| async {}))
        .route("/accounts/:account_type", get(accounts::get_accounts))
        .nest("/container", container::router())
        .nest("/network", network::router())
        .nest("/stats", stats::router(config.stats_polling_rate))
        .nest("/git", git::router())
        .nest("/build", build::router())
        .nest("/image", image::router())
        .layer(DockerClient::extension())
}
