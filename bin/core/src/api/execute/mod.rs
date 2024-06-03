use std::time::Instant;

use anyhow::anyhow;
use axum::{middleware, routing::post, Extension, Router};
use monitor_client::{
  api::execute::*,
  entities::{update::Update, user::User},
};
use resolver_api::{derive::Resolver, Resolver};
use serde::{Deserialize, Serialize};
use serror::{serialize_error_pretty, Json};
use typeshare::typeshare;
use uuid::Uuid;

use crate::{
  auth::auth_request,
  helpers::update::{init_execution_update, update_update},
  state::State,
};

mod build;
mod deployment;
mod procedure;
mod repo;
mod server;
mod server_template;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args((User, Update))]
#[serde(tag = "type", content = "params")]
pub enum ExecuteRequest {
  // ==== SERVER ====
  PruneContainers(PruneContainers),
  PruneImages(PruneImages),
  PruneNetworks(PruneNetworks),

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

  // ==== SERVER TEMPLATE ====
  LaunchServer(LaunchServer),
}

pub fn router() -> Router {
  Router::new()
    .route("/", post(handler))
    .layer(middleware::from_fn(auth_request))
}

async fn handler(
  Extension(user): Extension<User>,
  Json(request): Json<ExecuteRequest>,
) -> serror::Result<Json<Update>> {
  let req_id = Uuid::new_v4();

  // need to validate no cancel is active before any update is created.
  build::validate_cancel_build(&request).await?;

  let update = init_execution_update(&request, &user).await?;

  let handle =
    tokio::spawn(task(req_id, request, user, update.clone()));

  tokio::spawn({
    let mut update = update.clone();
    async move {
      match handle.await {
        Ok(Err(e)) => {
          warn!("/execute request {req_id} task error: {e:#}",);
          update
            .push_error_log("task error", serialize_error_pretty(&e));
          update.finalize();
          let _ = update_update(update).await;
        }
        Err(e) => {
          warn!("/execute request {req_id} spawn error: {e:?}",);
          update.push_error_log("spawn error", format!("{e:#?}"));
          update.finalize();
          let _ = update_update(update).await;
        }
        _ => {}
      }
    }
  });

  Ok(Json(update))
}

#[instrument(name = "ExecuteRequest", skip(user))]
async fn task(
  req_id: Uuid,
  request: ExecuteRequest,
  user: User,
  update: Update,
) -> anyhow::Result<String> {
  info!(
    "/execute request {req_id} | user: {} ({})",
    user.username, user.id
  );
  let timer = Instant::now();

  let res = State
    .resolve_request(request, (user, update))
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
