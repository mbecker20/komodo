use std::time::Instant;

use anyhow::{anyhow, Context};
use axum::{middleware, routing::post, Extension, Router};
use derive_variants::{EnumVariants, ExtractVariant};
use formatting::format_serror;
use komodo_client::{
  api::execute::*,
  entities::{
    update::{Log, Update},
    user::User,
  },
};
use mungos::by_id::find_one_by_id;
use resolver_api::{derive::Resolver, Resolver};
use serde::{Deserialize, Serialize};
use serror::Json;
use typeshare::typeshare;
use uuid::Uuid;

use crate::{
  auth::auth_request,
  helpers::update::{init_execution_update, update_update},
  state::{db_client, State},
};

mod action;
mod build;
mod deployment;
mod procedure;
mod repo;
mod server;
mod server_template;
mod stack;
mod sync;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Resolver, EnumVariants,
)]
#[variant_derive(Debug)]
#[resolver_target(State)]
#[resolver_args((User, Update))]
#[serde(tag = "type", content = "params")]
pub enum ExecuteRequest {
  // ==== SERVER ====
  StartContainer(StartContainer),
  RestartContainer(RestartContainer),
  PauseContainer(PauseContainer),
  UnpauseContainer(UnpauseContainer),
  StopContainer(StopContainer),
  DestroyContainer(DestroyContainer),
  StartAllContainers(StartAllContainers),
  RestartAllContainers(RestartAllContainers),
  PauseAllContainers(PauseAllContainers),
  UnpauseAllContainers(UnpauseAllContainers),
  StopAllContainers(StopAllContainers),
  PruneContainers(PruneContainers),
  DeleteNetwork(DeleteNetwork),
  PruneNetworks(PruneNetworks),
  DeleteImage(DeleteImage),
  PruneImages(PruneImages),
  DeleteVolume(DeleteVolume),
  PruneVolumes(PruneVolumes),
  PruneDockerBuilders(PruneDockerBuilders),
  PruneBuildx(PruneBuildx),
  PruneSystem(PruneSystem),

  // ==== DEPLOYMENT ====
  Deploy(Deploy),
  StartDeployment(StartDeployment),
  RestartDeployment(RestartDeployment),
  PauseDeployment(PauseDeployment),
  UnpauseDeployment(UnpauseDeployment),
  StopDeployment(StopDeployment),
  DestroyDeployment(DestroyDeployment),

  // ==== STACK ====
  DeployStack(DeployStack),
  DeployStackIfChanged(DeployStackIfChanged),
  StartStack(StartStack),
  RestartStack(RestartStack),
  StopStack(StopStack),
  PauseStack(PauseStack),
  UnpauseStack(UnpauseStack),
  DestroyStack(DestroyStack),

  // ==== BUILD ====
  RunBuild(RunBuild),
  CancelBuild(CancelBuild),

  // ==== REPO ====
  CloneRepo(CloneRepo),
  PullRepo(PullRepo),
  BuildRepo(BuildRepo),
  CancelRepoBuild(CancelRepoBuild),

  // ==== PROCEDURE ====
  RunProcedure(RunProcedure),

  // ==== ACTION ====
  RunAction(RunAction),

  // ==== SERVER TEMPLATE ====
  LaunchServer(LaunchServer),

  // ==== SYNC ====
  RunSync(RunSync),
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
    let update_id = update.id.clone();
    async move {
      let log = match handle.await {
        Ok(Err(e)) => {
          warn!("/execute request {req_id} task error: {e:#}",);
          Log::error("task error", format_serror(&e.into()))
        }
        Err(e) => {
          warn!("/execute request {req_id} spawn error: {e:?}",);
          Log::error("spawn error", format!("{e:#?}"))
        }
        _ => return,
      };
      let res = async {
        let mut update =
          find_one_by_id(&db_client().updates, &update_id)
            .await
            .context("failed to query to db")?
            .context("no update exists with given id")?;
        update.logs.push(log);
        update.finalize();
        update_update(update).await
      }
      .await;

      if let Err(e) = res {
        warn!("failed to update update with task error log | {e:#}");
      }
    }
  });

  Ok(Json(update))
}

#[instrument(
  name = "ExecuteRequest",
  skip(user, update),
  fields(
    user_id = user.id,
    update_id = update.id,
    request = format!("{:?}", request.extract_variant()))
  )
]
async fn task(
  req_id: Uuid,
  request: ExecuteRequest,
  user: User,
  update: Update,
) -> anyhow::Result<String> {
  info!("/execute request {req_id} | user: {}", user.username);
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
  debug!("/execute request {req_id} | resolve time: {elapsed:?}");

  res
}
