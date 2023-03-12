use std::{net::SocketAddr, sync::Arc};

use axum::{
    body::Body,
    extract::ConnectInfo,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    Json, RequestExt,
};
use serde_json::Value;
use types::{monitor_timestamp, PeripheryConfig};

pub async fn guard_request_by_passkey(
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

pub async fn guard_request_by_ip(
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
