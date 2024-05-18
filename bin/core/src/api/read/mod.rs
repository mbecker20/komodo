use std::time::Instant;

use anyhow::anyhow;
use axum::{middleware, routing::post, Extension, Router};
use axum_extra::{headers::ContentType, TypedHeader};
use monitor_client::{api::read::*, entities::user::User};
use resolver_api::{derive::Resolver, Resolve, Resolver};
use serde::{Deserialize, Serialize};
use serror::Json;
use typeshare::typeshare;
use uuid::Uuid;

use crate::{auth::auth_request, config::core_config, state::State};

mod alert;
mod alerter;
mod build;
mod builder;
mod deployment;
mod permission;
mod procedure;
mod repo;
mod search;
mod server;
mod server_template;
mod tag;
mod toml;
mod update;
mod user;
mod user_group;
mod variable;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(User)]
#[serde(tag = "type", content = "params")]
enum ReadRequest {
  GetVersion(GetVersion),
  GetCoreInfo(GetCoreInfo),

  // ==== USER ====
  ListUsers(ListUsers),
  GetUsername(GetUsername),
  ListApiKeys(ListApiKeys),
  ListApiKeysForServiceUser(ListApiKeysForServiceUser),
  ListPermissions(ListPermissions),
  GetPermissionLevel(GetPermissionLevel),
  ListUserTargetPermissions(ListUserTargetPermissions),

  // ==== USER GROUP ====
  GetUserGroup(GetUserGroup),
  ListUserGroups(ListUserGroups),

  // ==== SEARCH ====
  FindResources(FindResources),

  // ==== PROCEDURE ====
  GetProceduresSummary(GetProceduresSummary),
  GetProcedure(GetProcedure),
  GetProcedureActionState(GetProcedureActionState),
  ListProcedures(ListProcedures),

  // ==== SERVER TEMPLATE ====
  GetServerTemplate(GetServerTemplate),
  ListServerTemplates(ListServerTemplates),
  GetServerTemplatesSummary(GetServerTemplatesSummary),

  // ==== SERVER ====
  GetServersSummary(GetServersSummary),
  GetServer(GetServer),
  ListServers(ListServers),
  GetServerState(GetServerState),
  GetPeripheryVersion(GetPeripheryVersion),
  GetDockerContainers(GetDockerContainers),
  GetDockerImages(GetDockerImages),
  GetDockerNetworks(GetDockerNetworks),
  GetServerActionState(GetServerActionState),
  GetHistoricalServerStats(GetHistoricalServerStats),
  GetAvailableAccounts(GetAvailableAccounts),
  GetAvailableSecrets(GetAvailableSecrets),

  // ==== DEPLOYMENT ====
  GetDeploymentsSummary(GetDeploymentsSummary),
  GetDeployment(GetDeployment),
  ListDeployments(ListDeployments),
  GetDeploymentContainer(GetDeploymentContainer),
  GetDeploymentActionState(GetDeploymentActionState),
  GetDeploymentStats(GetDeploymentStats),
  GetLog(GetLog),
  SearchLog(SearchLog),
  ListCommonDeploymentExtraArgs(ListCommonDeploymentExtraArgs),

  // ==== BUILD ====
  GetBuildsSummary(GetBuildsSummary),
  GetBuild(GetBuild),
  ListBuilds(ListBuilds),
  GetBuildActionState(GetBuildActionState),
  GetBuildMonthlyStats(GetBuildMonthlyStats),
  GetBuildVersions(GetBuildVersions),
  ListCommonBuildExtraArgs(ListCommonBuildExtraArgs),
  #[to_string_resolver]
  ListDockerOrganizations(ListDockerOrganizations),

  // ==== REPO ====
  GetReposSummary(GetReposSummary),
  GetRepo(GetRepo),
  ListRepos(ListRepos),
  GetRepoActionState(GetRepoActionState),

  // ==== BUILDER ====
  GetBuildersSummary(GetBuildersSummary),
  GetBuilder(GetBuilder),
  ListBuilders(ListBuilders),
  GetBuilderAvailableAccounts(GetBuilderAvailableAccounts),

  // ==== ALERTER ====
  GetAlertersSummary(GetAlertersSummary),
  GetAlerter(GetAlerter),
  ListAlerters(ListAlerters),

  // ==== TOML ====
  ExportAllResourcesToToml(ExportAllResourcesToToml),
  ExportResourcesToToml(ExportResourcesToToml),

  // ==== TAG ====
  GetTag(GetTag),
  ListTags(ListTags),

  // ==== UPDATE ====
  GetUpdate(GetUpdate),
  ListUpdates(ListUpdates),

  // ==== ALERT ====
  ListAlerts(ListAlerts),
  GetAlert(GetAlert),

  // ==== SERVER STATS ====
  #[to_string_resolver]
  GetSystemInformation(GetSystemInformation),
  #[to_string_resolver]
  GetSystemStats(GetSystemStats),
  #[to_string_resolver]
  GetSystemProcesses(GetSystemProcesses),

  // ==== VARIABLE ====
  GetVariable(GetVariable),
  ListVariables(ListVariables),
}

pub fn router() -> Router {
  Router::new()
    .route("/", post(handler))
    .layer(middleware::from_fn(auth_request))
}

#[instrument(name = "ReadHandler", level = "debug", skip(user))]
async fn handler(
  Extension(user): Extension<User>,
  Json(request): Json<ReadRequest>,
) -> serror::Result<(TypedHeader<ContentType>, String)> {
  let timer = Instant::now();
  let req_id = Uuid::new_v4();
  debug!(
    "/read request {req_id} | user: {} ({})",
    user.username, user.id
  );
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
    warn!("/read request {req_id} error: {e:#}");
  }
  let elapsed = timer.elapsed();
  debug!("/read request {req_id} | resolve time: {elapsed:?}");
  Ok((TypedHeader(ContentType::json()), res?))
}

impl Resolve<GetVersion, User> for State {
  #[instrument(name = "GetVersion", level = "debug", skip(self))]
  async fn resolve(
    &self,
    GetVersion {}: GetVersion,
    _: User,
  ) -> anyhow::Result<GetVersionResponse> {
    Ok(GetVersionResponse {
      version: env!("CARGO_PKG_VERSION").to_string(),
    })
  }
}

impl Resolve<GetCoreInfo, User> for State {
  #[instrument(name = "GetCoreInfo", level = "debug", skip(self))]
  async fn resolve(
    &self,
    GetCoreInfo {}: GetCoreInfo,
    _: User,
  ) -> anyhow::Result<GetCoreInfoResponse> {
    let config = core_config();
    Ok(GetCoreInfoResponse {
      title: config.title.clone(),
      monitoring_interval: config.monitoring_interval,
      github_webhook_base_url: config
        .github_webhook_base_url
        .clone()
        .unwrap_or_else(|| config.host.clone()),
    })
  }
}
