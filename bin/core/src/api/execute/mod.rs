use std::time::Instant;

use anyhow::Context;
use axum::{middleware, routing::post, Extension, Json, Router};
use axum_extra::{headers::ContentType, TypedHeader};
use monitor_client::api::execute::*;
use resolver_api::{derive::Resolver, Resolve, Resolver};
use serde::{Deserialize, Serialize};
use serror::AppResult;
use typeshare::typeshare;
use uuid::Uuid;

use crate::{
  auth::{auth_request, RequestUser, RequestUserExtension},
  state::{State, StateExtension},
};

mod build;
mod deployment;
mod procedure;
mod repo;
mod server;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(RequestUser)]
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

  // ==== REPO ====
  CloneRepo(CloneRepo),
  PullRepo(PullRepo),

  // ==== PROCEDURE ====
  RunProcedure(RunProcedure),
}

pub fn router() -> Router {
  Router::new()
    .route(
      "/",
      post(
        |state: StateExtension,
         Extension(user): RequestUserExtension,
         Json(request): Json<ExecuteRequest>| async move {
          let timer = Instant::now();
          let req_id = Uuid::new_v4();
          info!(
            "/execute request {req_id} | user: {} ({}) | {request:?}",
            user.username, user.id
          );
          let res = tokio::spawn(async move {
            let res = state.resolve_request(request, user).await;
            if let Err(e) = &res {
              info!("/execute request {req_id} ERROR: {e:#?}");
            }
            let elapsed = timer.elapsed();
            info!(
              "/execute request {req_id} | resolve time: {elapsed:?}"
            );
            res
          })
          .await
          .context("failure in spawned execute task");
          if let Err(e) = &res {
            info!("/execute request {req_id} SPAWN ERROR: {e:#?}",);
          }
          AppResult::Ok((TypedHeader(ContentType::json()), res??))
        },
      ),
    )
    .layer(middleware::from_fn(auth_request))
}
