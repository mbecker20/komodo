use std::time::Instant;

use async_trait::async_trait;
use axum::{
    headers::ContentType, http::StatusCode, middleware, routing::post, Extension, Json, Router,
    TypedHeader,
};
use monitor_types::requests::read::*;
use resolver_api::{derive::Resolver, Resolve, ResolveToString, Resolver};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use uuid::Uuid;

use crate::{
    auth::{auth_request, RequestUser, RequestUserExtension},
    state::{State, StateExtension},
};

mod alerter;
mod build;
mod builder;
mod deployment;
mod repo;
mod search;
mod server;
mod tag;
mod update;
mod user;
mod alert;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(RequestUser)]
#[serde(tag = "type", content = "params")]
enum ReadRequest {
    GetVersion(GetVersion),
    GetUser(GetUser),
    GetUsername(GetUsername),

    // ==== SEARCH ====
    FindResources(FindResources),

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
    GetServerAvailableAccounts(GetServerAvailableAccounts),
    GetAvailableNetworks(GetAvailableNetworks),

    // ==== DEPLOYMENT ====
    GetDeploymentsSummary(GetDeploymentsSummary),
    GetDeployment(GetDeployment),
    ListDeployments(ListDeployments),
    GetDeploymentStatus(GetDeploymentStatus),
    GetDeploymentActionState(GetDeploymentActionState),
    GetDeployedVersion(GetDeployedVersion),
    GetDeploymentStats(GetDeploymentStats),
    GetLog(GetLog),

    // ==== BUILD ====
    GetBuildsSummary(GetBuildsSummary),
    GetBuild(GetBuild),
    ListBuilds(ListBuilds),
    GetBuildActionState(GetBuildActionState),
    GetBuildMonthlyStats(GetBuildMonthlyStats),
    GetBuildVersions(GetBuildVersions),

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
        .route(
            "/",
            post(
                |state: StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(request): Json<ReadRequest>| async move {
                    let timer = Instant::now();
                    let req_id = Uuid::new_v4();
                    debug!(
                        "/read request {req_id} | user: {} ({}) | {request:?}",
                        user.username, user.id
                    );
                    let res = state
                        .resolve_request(request, user)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:#?}")));
                    if let Err(e) = &res {
                        warn!("/read request {req_id} ERROR: {e:#?}");
                    }
                    let res = res?;
                    let elapsed = timer.elapsed();
                    debug!("/read request {req_id} | resolve time: {elapsed:?}");
                    Result::<_, (StatusCode, String)>::Ok((TypedHeader(ContentType::json()), res))
                },
            ),
        )
        .layer(middleware::from_fn(auth_request))
}

#[async_trait]
impl Resolve<GetVersion, RequestUser> for State {
    async fn resolve(
        &self,
        GetVersion {}: GetVersion,
        _: RequestUser,
    ) -> anyhow::Result<GetVersionResponse> {
        Ok(GetVersionResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }
}
