use std::{sync::OnceLock, time::Instant};

use anyhow::{anyhow, Context};
use axum::{middleware, routing::post, Extension, Router};
use axum_extra::{headers::ContentType, TypedHeader};
use monitor_client::{api::read::*, entities::user::User};
use resolver_api::{derive::Resolver, ResolveToString, Resolver};
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
mod sync;
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
  #[to_string_resolver]
  GetVersion(GetVersion),
  #[to_string_resolver]
  GetCoreInfo(GetCoreInfo),
  #[to_string_resolver]
  GetAvailableAwsEcrLabels(GetAvailableAwsEcrLabels),

  // ==== USER ====
  GetUsername(GetUsername),
  GetPermissionLevel(GetPermissionLevel),
  FindUser(FindUser),
  ListUsers(ListUsers),
  ListApiKeys(ListApiKeys),
  ListApiKeysForServiceUser(ListApiKeysForServiceUser),
  ListPermissions(ListPermissions),
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
  ListFullProcedures(ListFullProcedures),

  // ==== SERVER TEMPLATE ====
  GetServerTemplate(GetServerTemplate),
  GetServerTemplatesSummary(GetServerTemplatesSummary),
  ListServerTemplates(ListServerTemplates),
  ListFullServerTemplates(ListFullServerTemplates),

  // ==== SERVER ====
  GetServersSummary(GetServersSummary),
  GetServer(GetServer),
  GetServerState(GetServerState),
  GetPeripheryVersion(GetPeripheryVersion),
  GetDockerContainers(GetDockerContainers),
  GetDockerImages(GetDockerImages),
  GetDockerNetworks(GetDockerNetworks),
  GetServerActionState(GetServerActionState),
  GetHistoricalServerStats(GetHistoricalServerStats),
  GetAvailableAccounts(GetAvailableAccounts),
  GetAvailableSecrets(GetAvailableSecrets),
  ListServers(ListServers),
  ListFullServers(ListFullServers),

  // ==== DEPLOYMENT ====
  GetDeploymentsSummary(GetDeploymentsSummary),
  GetDeployment(GetDeployment),
  GetDeploymentContainer(GetDeploymentContainer),
  GetDeploymentActionState(GetDeploymentActionState),
  GetDeploymentStats(GetDeploymentStats),
  GetLog(GetLog),
  SearchLog(SearchLog),
  ListDeployments(ListDeployments),
  ListFullDeployments(ListFullDeployments),
  ListCommonDeploymentExtraArgs(ListCommonDeploymentExtraArgs),

  // ==== BUILD ====
  GetBuildsSummary(GetBuildsSummary),
  GetBuild(GetBuild),
  GetBuildActionState(GetBuildActionState),
  GetBuildMonthlyStats(GetBuildMonthlyStats),
  GetBuildVersions(GetBuildVersions),
  GetBuildWebhookEnabled(GetBuildWebhookEnabled),
  ListBuilds(ListBuilds),
  ListFullBuilds(ListFullBuilds),
  ListCommonBuildExtraArgs(ListCommonBuildExtraArgs),
  #[to_string_resolver]
  ListGithubOrganizations(ListGithubOrganizations),
  #[to_string_resolver]
  ListDockerOrganizations(ListDockerOrganizations),

  // ==== REPO ====
  GetReposSummary(GetReposSummary),
  GetRepo(GetRepo),
  GetRepoActionState(GetRepoActionState),
  GetRepoWebhooksEnabled(GetRepoWebhooksEnabled),
  ListRepos(ListRepos),
  ListFullRepos(ListFullRepos),

  // ==== SYNC ====
  GetResourceSyncsSummary(GetResourceSyncsSummary),
  GetResourceSync(GetResourceSync),
  GetResourceSyncActionState(GetResourceSyncActionState),
  GetSyncWebhooksEnabled(GetSyncWebhooksEnabled),
  ListResourceSyncs(ListResourceSyncs),
  ListFullResourceSyncs(ListFullResourceSyncs),

  // ==== BUILDER ====
  GetBuildersSummary(GetBuildersSummary),
  GetBuilder(GetBuilder),
  GetBuilderAvailableAccounts(GetBuilderAvailableAccounts),
  ListBuilders(ListBuilders),
  ListFullBuilders(ListFullBuilders),

  // ==== ALERTER ====
  GetAlertersSummary(GetAlertersSummary),
  GetAlerter(GetAlerter),
  ListAlerters(ListAlerters),
  ListFullAlerters(ListFullAlerters),

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

#[instrument(name = "ReadHandler", level = "debug", skip(user), fields(user_id = user.id))]
async fn handler(
  Extension(user): Extension<User>,
  Json(request): Json<ReadRequest>,
) -> serror::Result<(TypedHeader<ContentType>, String)> {
  let timer = Instant::now();
  let req_id = Uuid::new_v4();
  debug!("/read request | user: {}", user.username);
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
    debug!("/read request {req_id} error: {e:#}");
  }
  let elapsed = timer.elapsed();
  debug!("/read request {req_id} | resolve time: {elapsed:?}");
  Ok((TypedHeader(ContentType::json()), res?))
}

fn version() -> &'static String {
  static VERSION: OnceLock<String> = OnceLock::new();
  VERSION.get_or_init(|| {
    serde_json::to_string(&GetVersionResponse {
      version: env!("CARGO_PKG_VERSION").to_string(),
    })
    .context("failed to serialize GetVersionResponse")
    .unwrap()
  })
}

impl ResolveToString<GetVersion, User> for State {
  async fn resolve_to_string(
    &self,
    GetVersion {}: GetVersion,
    _: User,
  ) -> anyhow::Result<String> {
    Ok(version().to_string())
  }
}

fn core_info() -> &'static String {
  static CORE_INFO: OnceLock<String> = OnceLock::new();
  CORE_INFO.get_or_init(|| {
    let config = core_config();
    let info = GetCoreInfoResponse {
      title: config.title.clone(),
      monitoring_interval: config.monitoring_interval,
      github_webhook_base_url: config
        .github_webhook_base_url
        .clone()
        .unwrap_or_else(|| config.host.clone()),
      transparent_mode: config.transparent_mode,
      ui_write_disabled: config.ui_write_disabled,
      github_webhook_owners: config
        .github_webhook_app
        .installations
        .iter()
        .map(|i| i.namespace.to_string())
        .collect(),
    };
    serde_json::to_string(&info)
      .context("failed to serialize GetCoreInfoResponse")
      .unwrap()
  })
}

impl ResolveToString<GetCoreInfo, User> for State {
  async fn resolve_to_string(
    &self,
    GetCoreInfo {}: GetCoreInfo,
    _: User,
  ) -> anyhow::Result<String> {
    Ok(core_info().to_string())
  }
}

fn ecr_labels() -> &'static String {
  static ECR_LABELS: OnceLock<String> = OnceLock::new();
  ECR_LABELS.get_or_init(|| {
    serde_json::to_string(
      &core_config()
        .aws_ecr_registries
        .keys()
        .cloned()
        .collect::<Vec<_>>(),
    )
    .context("failed to serialize ecr registries")
    .unwrap()
  })
}

impl ResolveToString<GetAvailableAwsEcrLabels, User> for State {
  async fn resolve_to_string(
    &self,
    GetAvailableAwsEcrLabels {}: GetAvailableAwsEcrLabels,
    _: User,
  ) -> anyhow::Result<String> {
    Ok(ecr_labels().to_string())
  }
}
