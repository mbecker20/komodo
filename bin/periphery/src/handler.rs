use std::time::Instant;

use anyhow::{anyhow, Context};
use axum_extra::{headers::ContentType, TypedHeader};
use resolver_api::Resolver;
use serror::Json;
use uuid::Uuid;

use crate::State;

pub async fn handler(
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
  info!("request {req_id} | {request:?}");
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
  info!("request {req_id} | resolve time: {elapsed:?}");

  res
}
