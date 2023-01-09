use std::{net::SocketAddr, sync::Arc};

use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::get,
    Json, RequestExt, Router,
};
use helpers::docker::DockerClient;
use serde_json::Value;
use types::{monitor_timestamp, PeripheryConfig};

use crate::{HomeDirExtension, PeripheryConfigExtension};

mod accounts;
mod build;
mod command;
mod container;
mod git;
mod image;
mod network;
mod stats;

pub fn router(config: PeripheryConfigExtension, home_dir: HomeDirExtension) -> Router {
    Router::new()
        .route("/health", get(|| async {}))
        .route("/version", get(|| async { env!("CARGO_PKG_VERSION") }))
        .route("/accounts/:account_type", get(accounts::get_accounts))
        .nest("/command", command::router())
        .nest("/container", container::router())
        .nest("/network", network::router())
        .nest(
            "/stats",
            stats::router(config.stats_polling_rate.to_string().parse().unwrap()),
        )
        .nest("/git", git::router())
        .nest("/build", build::router())
        .nest("/image", image::router())
        .layer(DockerClient::extension())
        .layer(middleware::from_fn(guard_request))
        .layer(config)
        .layer(home_dir)
}

async fn guard_request(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<Response, (StatusCode, String)> {
    let config = req.extensions().get::<Arc<PeripheryConfig>>().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        "could not get periphery config".to_string(),
    ))?;
    if config.allowed_ips.is_empty() {
        return Ok(next.run(req).await);
    }
    let ConnectInfo(socket_addr) = req.extensions().get::<ConnectInfo<SocketAddr>>().ok_or((
        StatusCode::UNAUTHORIZED,
        "could not get socket addr of request".to_string(),
    ))?;
    let ip = socket_addr.ip();
    if config.allowed_ips.contains(&ip) {
        Ok(next.run(req).await)
    } else {
        let method = req.method().to_owned();
        let uri = req.uri().to_owned();
        let body = req
            .extract::<Json<Value>, _>()
            .await
            .ok()
            .map(|Json(body)| body);
        eprintln!(
            "{} | unauthorized request from {ip} | method: {method} | uri: {uri} | body: {:?}",
            monitor_timestamp(),
            body
        );
        Err((
            StatusCode::UNAUTHORIZED,
            format!("requesting ip {ip} not allowed"),
        ))
    }
}
