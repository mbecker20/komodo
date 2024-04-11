use std::time::Instant;

use axum::Json;
use axum_extra::{headers::ContentType, TypedHeader};
use resolver_api::Resolver;
use serror::AppResult;
use uuid::Uuid;

use crate::State;

#[instrument(name = "periphery_handler")]
pub async fn handler(
  Json(request): Json<crate::api::PeripheryRequest>,
) -> AppResult<(TypedHeader<ContentType>, String)> {
  let timer = Instant::now();
  let req_id = Uuid::new_v4();
  info!("request {req_id} | {request:?}");
  let res = tokio::spawn(async move {
    let res = State.resolve_request(request, ()).await;
    if let Err(resolver_api::Error::Serialization(e)) = &res {
      warn!("request {req_id} serialization error: {e:?}");
    }
    if let Err(resolver_api::Error::Inner(e)) = &res {
      warn!("request {req_id} error: {e:#}");
    }
    let elapsed = timer.elapsed();
    info!("request {req_id} | resolve time: {elapsed:?}");
    res
  })
  .await;
  if let Err(e) = &res {
    warn!("request {req_id} spawn error: {e:#}");
  }
  AppResult::Ok((TypedHeader(ContentType::json()), res??))
}
