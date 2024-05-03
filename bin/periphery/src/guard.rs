use std::net::SocketAddr;

use anyhow::{anyhow, Context};
use axum::{
  body::Body,
  extract::ConnectInfo,
  http::{Request, StatusCode},
  middleware::Next,
  response::Response,
};
use serror::{AddStatusCode, AddStatusCodeError};

use crate::config::periphery_config;

#[instrument(level = "debug")]
pub async fn guard_request_by_passkey(
  req: Request<Body>,
  next: Next,
) -> serror::Result<Response> {
  if periphery_config().passkeys.is_empty() {
    return Ok(next.run(req).await);
  }
  let Some(req_passkey) = req.headers().get("authorization") else {
    return Err(
      anyhow!("request was not sent with passkey")
        .status_code(StatusCode::UNAUTHORIZED),
    );
  };
  let req_passkey = req_passkey
    .to_str()
    .context("failed to convert passkey to str")
    .status_code(StatusCode::UNAUTHORIZED)?;
  if periphery_config()
    .passkeys
    .iter()
    .any(|passkey| passkey == req_passkey)
  {
    Ok(next.run(req).await)
  } else {
    Err(
      anyhow!("request passkey invalid")
        .status_code(StatusCode::UNAUTHORIZED),
    )
  }
}

#[instrument(level = "debug")]
pub async fn guard_request_by_ip(
  req: Request<Body>,
  next: Next,
) -> serror::Result<Response> {
  if periphery_config().allowed_ips.is_empty() {
    return Ok(next.run(req).await);
  }
  let ConnectInfo(socket_addr) = req
    .extensions()
    .get::<ConnectInfo<SocketAddr>>()
    .context("could not get ConnectionInfo of request")
    .status_code(StatusCode::UNAUTHORIZED)?;
  let ip = socket_addr.ip();
  if periphery_config().allowed_ips.contains(&ip) {
    Ok(next.run(req).await)
  } else {
    Err(
      anyhow!("requesting ip {ip} not allowed")
        .status_code(StatusCode::UNAUTHORIZED),
    )
  }
}
