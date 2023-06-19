use monitor_types::requests::api::{
    CreateLoginSecret, CreateServer, DeleteLoginSecret, DeleteServer, GetAllSystemStats,
    GetBasicSystemStats, GetCpuUsage, GetDiskUsage, GetNetworkUsage, GetPeripheryVersion,
    GetServer, GetSystemComponents, GetSystemInformation, GetSystemProcesses, ListServers,
    RenameServer, UpdateServer,
};
use resolver_api::{derive::Resolver, Resolve};
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
    GetAllSystemStats(GetAllSystemStats),
    GetBasicSystemStats(GetBasicSystemStats),
    GetCpuUsage(GetCpuUsage),
    GetDiskUsage(GetDiskUsage),
    GetNetworkUsage(GetNetworkUsage),
    GetSystemProcesses(GetSystemProcesses),
    GetSystemComponents(GetSystemComponents),
}
