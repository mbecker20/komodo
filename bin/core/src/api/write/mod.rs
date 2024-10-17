use std::time::Instant;

use anyhow::{anyhow, Context};
use axum::{middleware, routing::post, Extension, Router};
use axum_extra::{headers::ContentType, TypedHeader};
use derive_variants::{EnumVariants, ExtractVariant};
use komodo_client::{api::write::*, entities::user::User};
use resolver_api::{derive::Resolver, Resolver};
use serde::{Deserialize, Serialize};
use serror::Json;
use typeshare::typeshare;
use uuid::Uuid;

use crate::{auth::auth_request, state::State};

mod action;
mod alerter;
mod build;
mod builder;
mod deployment;
mod description;
mod permissions;
mod procedure;
mod provider;
mod repo;
mod server;
mod server_template;
mod service_user;
mod stack;
mod sync;
mod tag;
mod user;
mod user_group;
mod variable;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Resolver, EnumVariants,
)]
#[variant_derive(Debug)]
#[resolver_target(State)]
#[resolver_args(User)]
#[serde(tag = "type", content = "params")]
pub enum WriteRequest {
  // ==== USER ====
  UpdateUserUsername(UpdateUserUsername),
  UpdateUserPassword(UpdateUserPassword),
  DeleteUser(DeleteUser),

  // ==== SERVICE USER ====
  CreateServiceUser(CreateServiceUser),
  UpdateServiceUserDescription(UpdateServiceUserDescription),
  CreateApiKeyForServiceUser(CreateApiKeyForServiceUser),
  DeleteApiKeyForServiceUser(DeleteApiKeyForServiceUser),

  // ==== USER GROUP ====
  CreateUserGroup(CreateUserGroup),
  RenameUserGroup(RenameUserGroup),
  DeleteUserGroup(DeleteUserGroup),
  AddUserToUserGroup(AddUserToUserGroup),
  RemoveUserFromUserGroup(RemoveUserFromUserGroup),
  SetUsersInUserGroup(SetUsersInUserGroup),

  // ==== PERMISSIONS ====
  UpdateUserAdmin(UpdateUserAdmin),
  UpdateUserBasePermissions(UpdateUserBasePermissions),
  UpdatePermissionOnResourceType(UpdatePermissionOnResourceType),
  UpdatePermissionOnTarget(UpdatePermissionOnTarget),

  // ==== DESCRIPTION ====
  UpdateDescription(UpdateDescription),

  // ==== SERVER ====
  CreateServer(CreateServer),
  DeleteServer(DeleteServer),
  UpdateServer(UpdateServer),
  RenameServer(RenameServer),
  CreateNetwork(CreateNetwork),

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
  RefreshBuildCache(RefreshBuildCache),
  CreateBuildWebhook(CreateBuildWebhook),
  DeleteBuildWebhook(DeleteBuildWebhook),

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
  RefreshRepoCache(RefreshRepoCache),
  CreateRepoWebhook(CreateRepoWebhook),
  DeleteRepoWebhook(DeleteRepoWebhook),

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

  // ==== ACTION ====
  CreateAction(CreateAction),
  CopyAction(CopyAction),
  DeleteAction(DeleteAction),
  UpdateAction(UpdateAction),

  // ==== SYNC ====
  CreateResourceSync(CreateResourceSync),
  CopyResourceSync(CopyResourceSync),
  DeleteResourceSync(DeleteResourceSync),
  UpdateResourceSync(UpdateResourceSync),
  WriteSyncFileContents(WriteSyncFileContents),
  CommitSync(CommitSync),
  RefreshResourceSyncPending(RefreshResourceSyncPending),
  CreateSyncWebhook(CreateSyncWebhook),
  DeleteSyncWebhook(DeleteSyncWebhook),

  // ==== STACK ====
  CreateStack(CreateStack),
  CopyStack(CopyStack),
  DeleteStack(DeleteStack),
  UpdateStack(UpdateStack),
  RenameStack(RenameStack),
  WriteStackFileContents(WriteStackFileContents),
  RefreshStackCache(RefreshStackCache),
  CreateStackWebhook(CreateStackWebhook),
  DeleteStackWebhook(DeleteStackWebhook),

  // ==== TAG ====
  CreateTag(CreateTag),
  DeleteTag(DeleteTag),
  RenameTag(RenameTag),
  UpdateTagsOnResource(UpdateTagsOnResource),

  // ==== VARIABLE ====
  CreateVariable(CreateVariable),
  UpdateVariableValue(UpdateVariableValue),
  UpdateVariableDescription(UpdateVariableDescription),
  UpdateVariableIsSecret(UpdateVariableIsSecret),
  DeleteVariable(DeleteVariable),

  // ==== PROVIDERS ====
  CreateGitProviderAccount(CreateGitProviderAccount),
  UpdateGitProviderAccount(UpdateGitProviderAccount),
  DeleteGitProviderAccount(DeleteGitProviderAccount),
  CreateDockerRegistryAccount(CreateDockerRegistryAccount),
  UpdateDockerRegistryAccount(UpdateDockerRegistryAccount),
  DeleteDockerRegistryAccount(DeleteDockerRegistryAccount),
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

#[instrument(
  name = "WriteRequest",
  skip(user, request),
  fields(
    user_id = user.id,
    request = format!("{:?}", request.extract_variant())
  )
)]
async fn task(
  req_id: Uuid,
  request: WriteRequest,
  user: User,
) -> anyhow::Result<String> {
  info!("/write request | user: {}", user.username);

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
  debug!("/write request {req_id} | resolve time: {elapsed:?}");

  res
}
