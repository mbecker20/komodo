use std::time::Instant;

use async_trait::async_trait;
use axum::{middleware, routing::post, Extension, Json, Router};
use axum_extra::{headers::ContentType, TypedHeader};
use monitor_client::{api::read::*, entities::user::User};
use resolver_api::{
  derive::Resolver, Resolve, ResolveToString, Resolver,
};
use serde::{Deserialize, Serialize};
use serror::AppResult;
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
mod tag;
mod update;
mod user;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(User)]
#[serde(tag = "type", content = "params")]
enum ReadRequest {
  GetVersion(GetVersion),
  GetCoreInfo(GetCoreInfo),

  // ==== USER ====
  GetUsers(GetUsers),
  GetUsername(GetUsername),
  ListApiKeys(ListApiKeys),
  ListUserPermissions(ListUserPermissions),

  // ==== SEARCH ====
  FindResources(FindResources),

  // ==== PROCEDURE ====
  GetProceduresSummary(GetProceduresSummary),
  GetProcedure(GetProcedure),
  GetProcedureActionState(GetProcedureActionState),
  ListProcedures(ListProcedures),

  // ==== SERVER ====
  GetServersSummary(GetServersSummary),
  GetServer(GetServer),
  ListServers(ListServers),
  GetServerStatus(GetServerStatus),
  GetPeripheryVersion(GetPeripheryVersion),
  GetSystemInformation(GetSystemInformation),
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
  GetDeploymentStatus(GetDeploymentStatus),
  GetDeploymentActionState(GetDeploymentActionState),
  GetDeployedVersion(GetDeployedVersion),
  GetDeploymentStats(GetDeploymentStats),
  GetLog(GetLog),
  SearchLog(SearchLog),

  // ==== BUILD ====
  GetBuildsSummary(GetBuildsSummary),
  GetBuild(GetBuild),
  ListBuilds(ListBuilds),
  GetBuildActionState(GetBuildActionState),
  GetBuildMonthlyStats(GetBuildMonthlyStats),
  GetBuildVersions(GetBuildVersions),
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

  // ==== TAG ====
  GetTag(GetTag),
  ListTags(ListTags),

  // ==== UPDATE ====
  GetUpdate(GetUpdate),
  ListUpdates(ListUpdates),

  // ==== ALERT ====
  ListAlerts(ListAlerts),

  // ==== SERVER STATS ====
  #[to_string_resolver]
  GetAllSystemStats(GetAllSystemStats),
  #[to_string_resolver]
  GetBasicSystemStats(GetBasicSystemStats),
  #[to_string_resolver]
  GetCpuUsage(GetCpuUsage),
  #[to_string_resolver]
  GetDiskUsage(GetDiskUsage),
  #[to_string_resolver]
  GetNetworkUsage(GetNetworkUsage),
  #[to_string_resolver]
  GetSystemProcesses(GetSystemProcesses),
  #[to_string_resolver]
  GetSystemComponents(GetSystemComponents),
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
) -> AppResult<(TypedHeader<ContentType>, String)> {
  let timer = Instant::now();
  let req_id = Uuid::new_v4();
  debug!(
    "/read request {req_id} | user: {} ({})",
    user.username, user.id
  );
  let res = State.resolve_request(request, user).await;
  if let Err(resolver_api::Error::Serialization(e)) = &res {
    warn!("/read request {req_id} serialization error: {e:?}");
  }
  if let Err(resolver_api::Error::Inner(e)) = &res {
    warn!("/read request {req_id} error: {e:#}");
  }
  let elapsed = timer.elapsed();
  debug!("/read request {req_id} | resolve time: {elapsed:?}");
  AppResult::Ok((TypedHeader(ContentType::json()), res?))
}

#[async_trait]
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

#[async_trait]
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
