use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::{
        deployment::ContainerSummary,
        server::{
            docker_image::ImageSummary,
            docker_network::DockerNetwork,
            stats::{
                AllSystemStats, BasicSystemStats, CpuUsage,
                DiskUsage, NetworkUsage, SystemComponent,
                SystemInformation, SystemProcess, SystemStatsRecord,
            },
            Server, ServerActionState, ServerListItem, ServerStatus,
        },
        Timelength,
    },
    MongoDocument, I64,
};

use super::MonitorReadRequest;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(Server)]
pub struct GetServer {
    pub id: String,
}

#[typeshare]
pub type GetServerResponse = Server;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListServersResponse)]
pub struct ListServers {
    pub query: Option<MongoDocument>,
}

#[typeshare]
pub type ListServersResponse = Vec<ServerListItem>;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetServerStatusResponse)]
pub struct GetServerStatus {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetServerStatusResponse {
    pub status: ServerStatus,
}

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ServerActionState)]
pub struct GetServerActionState {
    pub id: String,
}

#[typeshare]
pub type GetServerActionStateResponse = ServerActionState;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetPeripheryVersionResponse)]
pub struct GetPeripheryVersion {
    pub server_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPeripheryVersionResponse {
    pub version: String,
}

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetSystemInformationResponse)]
pub struct GetSystemInformation {
    pub server_id: String,
}

#[typeshare]
pub type GetSystemInformationResponse = SystemInformation;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetAllSystemStatsResponse)]
pub struct GetAllSystemStats {
    pub server_id: String,
}

#[typeshare]
pub type GetAllSystemStatsResponse = AllSystemStats;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetBasicSystemStatsResponse)]
pub struct GetBasicSystemStats {
    pub server_id: String,
}

#[typeshare]
pub type GetBasicSystemStatsResponse = BasicSystemStats;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetCpuUsageResponse)]
pub struct GetCpuUsage {
    pub server_id: String,
}

#[typeshare]
pub type GetCpuUsageResponse = CpuUsage;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDiskUsageResponse)]
pub struct GetDiskUsage {
    pub server_id: String,
}

#[typeshare]
pub type GetDiskUsageResponse = DiskUsage;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetNetworkUsageResponse)]
pub struct GetNetworkUsage {
    pub server_id: String,
}

#[typeshare]
pub type GetNetworkUsageResponse = NetworkUsage;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetSystemProcessesResponse)]
pub struct GetSystemProcesses {
    pub server_id: String,
}

#[typeshare]
pub type GetSystemProcessesResponse = Vec<SystemProcess>;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetSystemComponentsResponse)]
pub struct GetSystemComponents {
    pub server_id: String,
}

#[typeshare]
pub type GetSystemComponentsResponse = Vec<SystemComponent>;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDockerNetworksResponse)]
pub struct GetDockerNetworks {
    pub server_id: String,
}

#[typeshare]
pub type GetDockerNetworksResponse = Vec<DockerNetwork>;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetHistoricalServerStatsResponse)]
pub struct GetHistoricalServerStats {
    pub server_id: String,
    pub interval: Timelength,
    #[serde(default)]
    pub page: u32,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHistoricalServerStatsResponse {
    pub stats: Vec<SystemStatsRecord>,
    pub next_page: Option<u32>,
}

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDockerImagesResponse)]
pub struct GetDockerImages {
    pub server_id: String,
}

#[typeshare]
pub type GetDockerImagesResponse = Vec<ImageSummary>;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDockerContainersResponse)]
pub struct GetDockerContainers {
    pub server_id: String,
}

#[typeshare]
pub type GetDockerContainersResponse = Vec<ContainerSummary>;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetServersSummaryResponse)]
pub struct GetServersSummary {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetServersSummaryResponse {
    pub total: I64,
    pub healthy: I64,
    pub unhealthy: I64,
    pub disabled: I64,
}

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetServerAvailableAccountsResponse)]
pub struct GetServerAvailableAccounts {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetServerAvailableAccountsResponse {
    pub github: Vec<String>,
    pub docker: Vec<String>,
}

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetAvailableSecretsResponse)]
pub struct GetAvailableSecrets {
    pub server_id: String,
}

#[typeshare]
pub type GetAvailableSecretsResponse = Vec<String>;
