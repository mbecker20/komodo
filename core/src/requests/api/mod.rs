use monitor_types::requests::api::{
    CreateLoginSecret, CreateServer, DeleteLoginSecret, DeleteServer, GetAllSystemStats,
    GetBasicSystemStats, GetCpuUsage, GetDiskUsage, GetNetworkUsage, GetPeripheryVersion,
    GetServer, GetSystemComponents, GetSystemInformation, GetSystemProcesses, ListServers,
    RenameServer, UpdateServer,
};
use resolver_api::{derive::Resolver, Resolve, ResolveToString};
use serde::{Deserialize, Serialize};

use crate::{auth::RequestUser, state::State};

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
    GetServer(GetServer),
    ListServers(ListServers),
    // CRUD
    CreateServer(CreateServer),
    DeleteServer(DeleteServer),
    UpdateServer(UpdateServer),
    RenameServer(RenameServer),
    // Stats
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

    //
    // ==== DEPLOYMENT ====
    //
}
