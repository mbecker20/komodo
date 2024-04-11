use std::net::SocketAddr;

use axum::{
  body::Body,
  extract::ConnectInfo,
  http::{Request, StatusCode},
  middleware::Next,
  response::Response,
  Json, RequestExt,
};
use serde_json::Value;

use crate::config::periphery_config;

#[instrument(level = "debug")]
pub async fn guard_request_by_passkey(
  req: Request<Body>,
  next: Next,
) -> Result<Response, (StatusCode, String)> {
  if periphery_config().passkeys.is_empty() {
    return Ok(next.run(req).await);
  }
  let Some(req_passkey) = req.headers().get("authorization") else {
    return Err((
      StatusCode::UNAUTHORIZED,
      String::from("request was not sent with passkey"),
    ));
  };
  let req_passkey = req_passkey
    .to_str()
    .map_err(|e| {
      (
        StatusCode::UNAUTHORIZED,
        format!("failed to get passkey from authorization header as str: {e:?}"),
      )
    })?
    .replace("Bearer ", "");
  if periphery_config().passkeys.contains(&req_passkey) {
    Ok(next.run(req).await)
  } else {
    let ConnectInfo(socket_addr) =
      req.extensions().get::<ConnectInfo<SocketAddr>>().ok_or((
        StatusCode::UNAUTHORIZED,
        "could not get socket addr of request".to_string(),
      ))?;
    let ip = socket_addr.ip();
    let body = req
      .extract::<Json<Value>, _>()
      .await
      .ok()
      .map(|Json(body)| body);
    warn!(
      "unauthorized request from {ip} (bad passkey) | body: {body:?}"
    );
    Err((
      StatusCode::UNAUTHORIZED,
      String::from("request passkey invalid"),
    ))
  }
}

#[instrument(level = "debug")]
pub async fn guard_request_by_ip(
  req: Request<Body>,
  next: Next,
) -> Result<Response, (StatusCode, String)> {
  if periphery_config().allowed_ips.is_empty() {
    return Ok(next.run(req).await);
  }
  let ConnectInfo(socket_addr) =
    req.extensions().get::<ConnectInfo<SocketAddr>>().ok_or((
      StatusCode::UNAUTHORIZED,
      "could not get socket addr of request".to_string(),
    ))?;
  let ip = socket_addr.ip();
  if periphery_config().allowed_ips.contains(&ip) {
    Ok(next.run(req).await)
  } else {
    let body = req
      .extract::<Json<Value>, _>()
      .await
      .ok()
      .map(|Json(body)| body);
    warn!(
      "unauthorized request from {ip} (unknown ip) | body: {body:?}"
    );
    Err((
      StatusCode::UNAUTHORIZED,
      format!("requesting ip {ip} not allowed"),
    ))
  }
}
