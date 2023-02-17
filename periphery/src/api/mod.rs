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

use self::stats::{StatsClient, StatsExtension};

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
        .route(
            "/system_information",
            get(|sys: StatsExtension| async move { Json(sys.read().unwrap().info.clone()) }),
        )
        .route("/accounts/:account_type", get(accounts::get_accounts))
        .nest("/command", command::router())
        .nest("/container", container::router())
        .nest("/network", network::router())
        .nest("/stats", stats::router())
        .nest("/git", git::router())
        .nest("/build", build::router())
        .nest("/image", image::router())
        .layer(DockerClient::extension())
        .layer(middleware::from_fn(guard_request_by_ip))
        .layer(middleware::from_fn(guard_request_by_passkey))
        .layer(StatsClient::extension(
            config.stats_polling_rate.to_string().parse().unwrap(),
        ))
        .layer(config)
        .layer(home_dir)
}

async fn guard_request_by_passkey(
    req: Request<Body>,
    next: Next<Body>,
) -> Result<Response, (StatusCode, String)> {
    let config = req.extensions().get::<Arc<PeripheryConfig>>().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        "could not get periphery config".to_string(),
    ))?;
    if config.passkeys.is_empty() {
        return Ok(next.run(req).await);
    }
    let req_passkey = req.headers().get("authorization");
    if req_passkey.is_none() {
        return Err((
            StatusCode::UNAUTHORIZED,
            format!("request was not sent with passkey"),
        ));
    }
    let req_passkey = req_passkey
        .unwrap()
        .to_str()
        .map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                format!("failed to get passkey from authorization header as str: {e:?}"),
            )
        })?
        .to_string();
    if config.passkeys.contains(&req_passkey) {
        Ok(next.run(req).await)
    } else {
        let ConnectInfo(socket_addr) =
            req.extensions().get::<ConnectInfo<SocketAddr>>().ok_or((
                StatusCode::UNAUTHORIZED,
                "could not get socket addr of request".to_string(),
            ))?;
        let ip = socket_addr.ip();
        let method = req.method().to_owned();
        let uri = req.uri().to_owned();
        let body = req
            .extract::<Json<Value>, _>()
            .await
            .ok()
            .map(|Json(body)| body);
        eprintln!(
            "{} | unauthorized request from {ip} (bad passkey) | method: {method} | uri: {uri} | body: {body:?}",
            monitor_timestamp(),
        );
        Err((StatusCode::UNAUTHORIZED, format!("request passkey invalid")))
    }
}

async fn guard_request_by_ip(
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
            "{} | unauthorized request from {ip} | method: {method} | uri: {uri} | body: {body:?}",
            monitor_timestamp()
        );
        Err((
            StatusCode::UNAUTHORIZED,
            format!("requesting ip {ip} not allowed"),
        ))
    }
}
