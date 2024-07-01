use std::{net::SocketAddr, time::Instant};

use anyhow::{anyhow, Context};
use axum::{
  body::Body,
  extract::ConnectInfo,
  http::{Request, StatusCode},
  middleware::{self, Next},
  response::Response,
  routing::post,
  Router,
};
use axum_extra::{headers::ContentType, TypedHeader};
use resolver_api::Resolver;
use serror::{AddStatusCode, AddStatusCodeError, Json};
use uuid::Uuid;

use crate::{config::periphery_config, State};

pub fn router() -> Router {
  Router::new()
    .route("/", post(handler))
    .layer(middleware::from_fn(guard_request_by_ip))
    .layer(middleware::from_fn(guard_request_by_passkey))
}

async fn handler(
  Json(request): Json<crate::api::PeripheryRequest>,
) -> serror::Result<(TypedHeader<ContentType>, String)> {
  let req_id = Uuid::new_v4();

  let res = tokio::spawn(task(req_id, request))
    .await
    .context("task handler spawn error");

  if let Err(e) = &res {
    warn!("request {req_id} spawn error: {e:#}");
  }

  Ok((TypedHeader(ContentType::json()), res??))
}

#[instrument(name = "PeripheryHandler")]
async fn task(
  req_id: Uuid,
  request: crate::api::PeripheryRequest,
) -> anyhow::Result<String> {
  let timer = Instant::now();

  let res =
    State
      .resolve_request(request, ())
      .await
      .map_err(|e| match e {
        resolver_api::Error::Serialization(e) => {
          anyhow!("{e:?}").context("response serialization error")
        }
        resolver_api::Error::Inner(e) => e,
      });

  if let Err(e) = &res {
    warn!("request {req_id} error: {e:#}");
  }

  let elapsed = timer.elapsed();
  debug!("request {req_id} | resolve time: {elapsed:?}");

  res
}

#[instrument(level = "debug")]
async fn guard_request_by_passkey(
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
async fn guard_request_by_ip(
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
