use axum::{middleware, routing::get, Json, Router};

use crate::{helpers::docker::DockerClient, HomeDirExtension, PeripheryConfigExtension};

use self::stats::{StatsClient, StatsExtension};

mod accounts;
mod build;
mod command;
mod container;
mod git;
mod guard;
mod image;
mod network;
mod stats;

pub fn router(config: PeripheryConfigExtension, home_dir: HomeDirExtension) -> Router {
    Router::new()
        .route("/health", get(|| async {}))
        .route("/version", get(|| async { env!("CARGO_PKG_VERSION") }))
        .route(
            "/system_information",
            get(|sys: StatsExtension| async move { Json(sys.read().unwrap().info.clone()) }),
        )
        .route("/accounts/:account_type", get(accounts::get_accounts))
        .route("/secrets", get(get_available_secrets))
        .nest("/command", command::router())
        .nest("/container", container::router())
        .nest("/network", network::router())
        .nest("/stats", stats::router())
        .nest("/git", git::router())
        .nest("/build", build::router())
        .nest("/image", image::router())
        .layer(DockerClient::extension())
        .layer(middleware::from_fn(guard::guard_request_by_ip))
        .layer(middleware::from_fn(guard::guard_request_by_passkey))
        .layer(StatsClient::extension(
            config.stats_polling_rate.to_string().parse().unwrap(),
        ))
        .layer(config)
        .layer(home_dir)
}

async fn get_available_secrets(config: PeripheryConfigExtension) -> Json<Vec<String>> {
    let mut vars: Vec<String> = config.secrets.keys().map(|k| k.clone()).collect();
    vars.sort();
    Json(vars)
}
