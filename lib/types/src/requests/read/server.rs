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
                AllSystemStats, BasicSystemStats, CpuUsage, DiskUsage, NetworkUsage,
                SystemComponent, SystemInformation, SystemProcess,
            },
            Server, ServerActionState, ServerStatus,
        },
    },
    MongoDocument,
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

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerListItem {
    pub id: String,
    pub name: String,
    pub status: ServerStatus,
    pub tags: Vec<String>,
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
