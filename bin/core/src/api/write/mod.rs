use std::time::Instant;

use anyhow::{anyhow, Context};
use axum::{middleware, routing::post, Extension, Router};
use axum_extra::{headers::ContentType, TypedHeader};
use monitor_client::{api::write::*, entities::user::User};
use resolver_api::{derive::Resolver, Resolve, Resolver};
use serde::{Deserialize, Serialize};
use serror::Json;
use typeshare::typeshare;
use uuid::Uuid;

use crate::{auth::auth_request, state::State};

mod alerter;
mod api_key;
mod build;
mod builder;
mod deployment;
mod description;
mod permissions;
mod procedure;
mod repo;
mod server;
mod server_template;
mod tag;
mod user;
mod user_group;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(User)]
#[serde(tag = "type", content = "params")]
enum WriteRequest {
  // ==== API KEY ====
  CreateApiKey(CreateApiKey),
  DeleteApiKey(DeleteApiKey),
  CreateApiKeyForServiceUser(CreateApiKeyForServiceUser),
  DeleteApiKeyForServiceUser(DeleteApiKeyForServiceUser),

  // ==== USER ====
  PushRecentlyViewed(PushRecentlyViewed),
  SetLastSeenUpdate(SetLastSeenUpdate),
  CreateServiceUser(CreateServiceUser),
  UpdateServiceUserDescription(UpdateServiceUserDescription),

  // ==== USER GROUP ====
  CreateUserGroup(CreateUserGroup),
  RenameUserGroup(RenameUserGroup),
  DeleteUserGroup(DeleteUserGroup),
  AddUserToUserGroup(AddUserToUserGroup),
  RemoveUserFromUserGroup(RemoveUserFromUserGroup),
  SetUsersInUserGroup(SetUsersInUserGroup),

  // ==== PERMISSIONS ====
  UpdateUserBasePermissions(UpdateUserBasePermissions),
  UpdatePermissionOnTarget(UpdatePermissionOnTarget),

  // ==== DESCRIPTION ====
  UpdateDescription(UpdateDescription),

  // ==== SERVER ====
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

  // ==== SERVER TEMPLATE ====
  CreateServerTemplate(CreateServerTemplate),
  CopyServerTemplate(CopyServerTemplate),
  DeleteServerTemplate(DeleteServerTemplate),
  UpdateServerTemplate(UpdateServerTemplate),

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
  RenameTag(RenameTag),
  UpdateTagsOnResource(UpdateTagsOnResource),
}

pub fn router() -> Router {
  Router::new()
    .route("/", post(handler))
    .layer(middleware::from_fn(auth_request))
}

async fn handler(
  Extension(user): Extension<User>,
  Json(request): Json<WriteRequest>,
) -> serror::Result<(TypedHeader<ContentType>, String)> {
  let req_id = Uuid::new_v4();

  let res = tokio::spawn(task(req_id, request, user))
    .await
    .context("failure in spawned task");

  if let Err(e) = &res {
    warn!("/write request {req_id} spawn error: {e:#}");
  }

  Ok((TypedHeader(ContentType::json()), res??))
}

#[instrument(name = "WriteRequest", skip(user))]
async fn task(
  req_id: Uuid,
  request: WriteRequest,
  user: User,
) -> anyhow::Result<String> {
  info!(
    "/write request {req_id} | user: {} ({})",
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
    warn!("/write request {req_id} error: {e:#}");
  }

  let elapsed = timer.elapsed();
  info!("/write request {req_id} | resolve time: {elapsed:?}");

  res
}
