use std::time::Instant;

use anyhow::{anyhow, Context};
use axum::{middleware, routing::post, Extension, Json, Router};
use axum_extra::{headers::ContentType, TypedHeader};
use monitor_client::{api::execute::*, entities::user::User};
use resolver_api::{derive::Resolver, Resolve, Resolver};
use serde::{Deserialize, Serialize};
use serror::AppResult;
use typeshare::typeshare;
use uuid::Uuid;

use crate::{auth::auth_request, state::State};

mod build;
mod deployment;
mod procedure;
mod repo;
mod server;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(User)]
#[serde(tag = "type", content = "params")]
enum ExecuteRequest {
  // ==== SERVER ====
  PruneContainers(PruneDockerContainers),
  PruneImages(PruneDockerImages),
  PruneNetworks(PruneDockerNetworks),

  // ==== DEPLOYMENT ====
  Deploy(Deploy),
  StartContainer(StartContainer),
  StopContainer(StopContainer),
  StopAllContainers(StopAllContainers),
  RemoveContainer(RemoveContainer),

  // ==== BUILD ====
  RunBuild(RunBuild),
  CancelBuild(CancelBuild),

  // ==== REPO ====
  CloneRepo(CloneRepo),
  PullRepo(PullRepo),

  // ==== PROCEDURE ====
  RunProcedure(RunProcedure),
}

pub fn router() -> Router {
  Router::new()
    .route("/", post(handler))
    .layer(middleware::from_fn(auth_request))
}

async fn handler(
  Extension(user): Extension<User>,
  Json(request): Json<ExecuteRequest>,
) -> AppResult<(TypedHeader<ContentType>, String)> {
  let req_id = Uuid::new_v4();

  let res = tokio::spawn(task(req_id, request, user))
    .await
    .context("failure in spawned execute task");

  if let Err(e) = &res {
    warn!("/execute request {req_id} spawn error: {e:#}",);
  }

  AppResult::Ok((TypedHeader(ContentType::json()), res??))
}

#[instrument(name = "ExecuteRequest", skip(user))]
async fn task(
  req_id: Uuid,
  request: ExecuteRequest,
  user: User,
) -> anyhow::Result<String> {
  info!(
    "/execute request {req_id} | user: {} ({})",
    user.username, user.id
  );
  let timer = Instant::now();

  let res =
    State
      .resolve_request(request, user)
      .await
      .map_err(|e| match e {
        resolver_api::Error::Serialization(e) => {
          anyhow!("{e:?}").context("response serialization error")
        }
        resolver_api::Error::Inner(e) => e,
      });

  if let Err(e) = &res {
    warn!("/execute request {req_id} error: {e:#}");
  }

  let elapsed = timer.elapsed();
  info!("/execute request {req_id} | resolve time: {elapsed:?}");

  res
}
