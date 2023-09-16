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

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Server)]
pub struct GetServer {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<ServerListItem>)]
pub struct ListServers {
    pub query: Option<MongoDocument>,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
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
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(ServerActionState)]
pub struct GetServerActionState {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
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
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(SystemInformation)]
pub struct GetSystemInformation {
    pub server_id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(AllSystemStats)]
pub struct GetAllSystemStats {
    pub server_id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(BasicSystemStats)]
pub struct GetBasicSystemStats {
    pub server_id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(CpuUsage)]
pub struct GetCpuUsage {
    pub server_id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(DiskUsage)]
pub struct GetDiskUsage {
    pub server_id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(NetworkUsage)]
pub struct GetNetworkUsage {
    pub server_id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<SystemProcess>)]
pub struct GetSystemProcesses {
    pub server_id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<SystemComponent>)]
pub struct GetSystemComponents {
    pub server_id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<DockerNetwork>)]
pub struct GetDockerNetworks {
    pub server_id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
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
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<ImageSummary>)]
pub struct GetDockerImages {
    pub server_id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<ContainerSummary>)]
pub struct GetDockerContainers {
    pub server_id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
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
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
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
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetAvailableNetworksResponse)]
pub struct GetAvailableNetworks {
    pub server_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAvailableNetworksResponse {
    pub networks: Vec<DockerNetwork>,
}
