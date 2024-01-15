use std::time::Instant;

use axum::{middleware, routing::post, Extension, Json, Router};
use axum_extra::{headers::ContentType, TypedHeader};
use monitor_client::api::write::*;
use resolver_api::{derive::Resolver, Resolve, Resolver};
use serde::{Deserialize, Serialize};
use serror::AppResult;
use typeshare::typeshare;
use uuid::Uuid;

use crate::{
  auth::{auth_request, RequestUser, RequestUserExtension},
  state::{State, StateExtension},
};

mod alerter;
mod api_key;
mod build;
mod builder;
mod deployment;
mod description;
mod launch;
mod permissions;
mod procedure;
mod repo;
mod server;
mod tag;
mod user;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(RequestUser)]
#[serde(tag = "type", content = "params")]
enum WriteRequest {
  // ==== API KEY ====
  CreateApiKey(CreateApiKey),
  DeleteApiKey(DeleteApiKey),

  // ==== USER ====
  PushRecentlyViewed(PushRecentlyViewed),
  SetLastSeenUpdate(SetLastSeenUpdate),

  // ==== PERMISSIONS ====
  UpdateUserPerimissions(UpdateUserPermissions),
  UpdateUserPermissionsOnTarget(UpdateUserPermissionsOnTarget),

  // ==== DESCRIPTION ====
  UpdateDescription(UpdateDescription),

  // ==== SERVER ====
  LaunchServer(LaunchServer),
  CreateServer(CreateServer),
  DeleteServer(DeleteServer),
  UpdateServer(UpdateServer),
  RenameServer(RenameServer),
  CreateNetwork(CreateNetwork),
  DeleteNetwork(DeleteNetwork),

  // ==== DEPLOYMENT ====
  CreateDeployment(CreateDeployment),
  CopyDeployment(CopyDeployment),
  DeleteDeployment(DeleteDeployment),
  UpdateDeployment(UpdateDeployment),
  RenameDeployment(RenameDeployment),

  // ==== BUILD ====
  CreateBuild(CreateBuild),
  CopyBuild(CopyBuild),
  DeleteBuild(DeleteBuild),
  UpdateBuild(UpdateBuild),

  // ==== BUILDER ====
  CreateBuilder(CreateBuilder),
  CopyBuilder(CopyBuilder),
  DeleteBuilder(DeleteBuilder),
  UpdateBuilder(UpdateBuilder),

  // ==== REPO ====
  CreateRepo(CreateRepo),
  CopyRepo(CopyRepo),
  DeleteRepo(DeleteRepo),
  UpdateRepo(UpdateRepo),

  // ==== ALERTER ====
  CreateAlerter(CreateAlerter),
  CopyAlerter(CopyAlerter),
  DeleteAlerter(DeleteAlerter),
  UpdateAlerter(UpdateAlerter),

  // ==== PROCEDURE ====
  CreateProcedure(CreateProcedure),
  CopyProcedure(CopyProcedure),
  DeleteProcedure(DeleteProcedure),
  UpdateProcedure(UpdateProcedure),

  // ==== TAG ====
  CreateTag(CreateTag),
  DeleteTag(DeleteTag),
  UpdateTag(UpdateTag),
}

pub fn router() -> Router {
  Router::new()
    .route(
      "/",
      post(
        |state: StateExtension,
         Extension(user): RequestUserExtension,
         Json(request): Json<WriteRequest>| async move {
          let timer = Instant::now();
          let req_id = Uuid::new_v4();
          info!(
            "/write request {req_id} | user: {} ({}) | {request:?}",
            user.username, user.id
          );
          let res = tokio::spawn(async move {
            let res = state.resolve_request(request, user).await;
            if let Err(e) = &res {
              info!("/write request {req_id} ERROR: {e:#?}");
            }
            let elapsed = timer.elapsed();
            info!(
              "/write request {req_id} | resolve time: {elapsed:?}"
            );
            res
          })
          .await;
          if let Err(e) = &res {
            info!("/write request {req_id} SPAWN ERROR: {e:#?}");
          }
          AppResult::Ok((TypedHeader(ContentType::json()), res??))
        },
      ),
    )
    .layer(middleware::from_fn(auth_request))
}
