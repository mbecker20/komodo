use std::time::Instant;

use axum::Json;
use axum_extra::{headers::ContentType, TypedHeader};
use resolver_api::Resolver;
use serror::AppResult;
use uuid::Uuid;

use crate::State;

#[instrument(name = "PeripheryHandler")]
pub async fn handler(
  Json(request): Json<crate::api::PeripheryRequest>,
) -> AppResult<(TypedHeader<ContentType>, String)> {
  let timer = Instant::now();
  let req_id = Uuid::new_v4();
  info!("request {req_id} | {request:?}");
  let res =
    tokio::spawn(
      async move { State.resolve_request(request, ()).await },
    )
    .await;

  let elapsed = timer.elapsed();
  info!("request {req_id} | resolve time: {elapsed:?}");

  if let Err(e) = &res {
    warn!("request {req_id} spawn error: {e:#}");
  }

  let res = res?;
  if let Err(resolver_api::Error::Serialization(e)) = &res {
    warn!("request {req_id} serialization error: {e:?}");
  }
  if let Err(resolver_api::Error::Inner(e)) = &res {
    warn!("request {req_id} error: {e:#}");
  }

  AppResult::Ok((TypedHeader(ContentType::json()), res?))
}
