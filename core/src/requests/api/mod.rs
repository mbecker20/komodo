use std::time::Instant;

use axum::{Router, routing::post, Extension, Json, http::StatusCode, TypedHeader, headers::ContentType, middleware};
use monitor_types::requests::api::{
    CreateLoginSecret, CreateServer, DeleteLoginSecret, DeleteServer, GetAllSystemStats,
    GetBasicSystemStats, GetCpuUsage, GetDiskUsage, GetDockerContainers, GetDockerImages,
    GetDockerNetworks, GetNetworkUsage, GetPeripheryVersion, GetServer, GetSystemComponents,
    GetSystemInformation, GetSystemProcesses, ListServers, PruneDockerContainers,
    PruneDockerImages, PruneDockerNetworks, RenameServer, UpdateServer,
};
use resolver_api::{derive::Resolver, Resolve, ResolveToString, Resolver};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::{RequestUser, RequestUserExtension, auth_request}, state::{State, StateExtension}};

mod secret;
mod server;

#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(RequestUser)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
pub enum ApiRequest {
    // ==== SECRET ====
    CreateLoginSecret(CreateLoginSecret),
    DeleteLoginSecret(DeleteLoginSecret),

    //
    // ==== SERVER ====
    //
    GetPeripheryVersion(GetPeripheryVersion),
    GetSystemInformation(GetSystemInformation),
    GetDockerContainers(GetDockerContainers),
    GetDockerImages(GetDockerImages),
    GetDockerNetworks(GetDockerNetworks),
    GetServer(GetServer),
    ListServers(ListServers),
    // CRUD
    CreateServer(CreateServer),
    DeleteServer(DeleteServer),
    UpdateServer(UpdateServer),
    RenameServer(RenameServer),
    // STATS
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
    // ACTIONS
    PruneContainers(PruneDockerContainers),
    PruneImages(PruneDockerImages),
    PruneNetworks(PruneDockerNetworks),
    //
    // ==== DEPLOYMENT ====
    //
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            post(
                |state: StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(request): Json<ApiRequest>| async move {
                    let timer = Instant::now();
                    let req_id = Uuid::new_v4();
                    info!("/auth request {req_id} | {request:?}");
                    let res = state
                        .resolve_request(request, user)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("{e:?}")));
                    if let Err(e) = &res {
                        info!("/auth request {req_id} ERROR: {e:?}");
                    }
                    let res = res?;
                    let elapsed = timer.elapsed();
                    info!("/auth request {req_id} | resolve time: {elapsed:?}");
                    debug!("/auth request {req_id} RESPONSE: {res}");
                    Result::<_, (StatusCode, String)>::Ok((TypedHeader(ContentType::json()), res))
                },
            ),
        )
        .layer(middleware::from_fn(auth_request))
}