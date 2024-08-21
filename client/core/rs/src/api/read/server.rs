use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  docker::{
    container::{Container, ContainerListItem},
    image::{Image, ImageHistoryResponseItem, ImageListItem},
    network::{Network, NetworkListItem},
    volume::{Volume, VolumeListItem},
  },
  server::{
    Server, ServerActionState, ServerListItem, ServerQuery,
    ServerState,
  },
  stack::ComposeProject,
  stats::{
    SystemInformation, SystemProcess, SystemStats, SystemStatsRecord,
  },
  Timelength, I64,
};

use super::MonitorReadRequest;

//

/// Get a specific server. Response: [Server].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(Server)]
pub struct GetServer {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type GetServerResponse = Server;

//

/// List servers matching optional query. Response: [ListServersResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListServersResponse)]
pub struct ListServers {
  /// optional structured query to filter servers.
  #[serde(default)]
  pub query: ServerQuery,
}

#[typeshare]
pub type ListServersResponse = Vec<ServerListItem>;

//

/// List servers matching optional query. Response: [ListFullServersResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListFullServersResponse)]
pub struct ListFullServers {
  /// optional structured query to filter servers.
  #[serde(default)]
  pub query: ServerQuery,
}

#[typeshare]
pub type ListFullServersResponse = Vec<Server>;

//

/// Get the state of the target server. Response: [GetServerStateResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetServerStateResponse)]
pub struct GetServerState {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

/// The response for [GetServerState].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetServerStateResponse {
  /// The server status.
  pub status: ServerState,
}

//

/// Get current action state for the servers. Response: [ServerActionState].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ServerActionState)]
pub struct GetServerActionState {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type GetServerActionStateResponse = ServerActionState;

//

/// Get the version of the monitor periphery agent on the target server.
/// Response: [GetPeripheryVersionResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetPeripheryVersionResponse)]
pub struct GetPeripheryVersion {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

/// Response for [GetPeripheryVersion].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPeripheryVersionResponse {
  /// The version of periphery.
  pub version: String,
}

//

/// List the docker networks on the server. Response: [ListDockerNetworksResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListDockerNetworksResponse)]
pub struct ListDockerNetworks {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type ListDockerNetworksResponse = Vec<NetworkListItem>;

//

/// Inspect a docker network on the server. Response: [InspectDockerNetworkResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(InspectDockerNetworkResponse)]
pub struct InspectDockerNetwork {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
  /// The network name
  pub network: String,
}

#[typeshare]
pub type InspectDockerNetworkResponse = Network;

//

/// List the docker images locally cached on the target server.
/// Response: [ListDockerImagesResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListDockerImagesResponse)]
pub struct ListDockerImages {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type ListDockerImagesResponse = Vec<ImageListItem>;

//

/// Inspect a docker image on the server. Response: [Image].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(InspectDockerImageResponse)]
pub struct InspectDockerImage {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
  /// The image name
  pub image: String,
}

#[typeshare]
pub type InspectDockerImageResponse = Image;

//

/// Get image history from the server. Response: [ListDockerImageHistoryResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListDockerImageHistoryResponse)]
pub struct ListDockerImageHistory {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
  /// The image name
  pub image: String,
}

#[typeshare]
pub type ListDockerImageHistoryResponse =
  Vec<ImageHistoryResponseItem>;

//

/// List all docker containers on the target server.
/// Response: [ListDockerContainersResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListDockerContainersResponse)]
pub struct ListDockerContainers {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type ListDockerContainersResponse = Vec<ContainerListItem>;

//

/// Inspect a docker container on the server. Response: [Container].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(InspectDockerContainerResponse)]
pub struct InspectDockerContainer {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
  /// The container name
  pub container: String,
}

#[typeshare]
pub type InspectDockerContainerResponse = Container;

//

/// List all docker volumes on the target server.
/// Response: [ListDockerVolumesResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListDockerVolumesResponse)]
pub struct ListDockerVolumes {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type ListDockerVolumesResponse = Vec<VolumeListItem>;

//

/// Inspect a docker volume on the server. Response: [Volume].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(InspectDockerVolumeResponse)]
pub struct InspectDockerVolume {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
  /// The volume name
  pub volume: String,
}

#[typeshare]
pub type InspectDockerVolumeResponse = Volume;

//

/// List all docker compose projects on the target server.
/// Response: [ListComposeProjectsResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListComposeProjectsResponse)]
pub struct ListComposeProjects {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type ListComposeProjectsResponse = Vec<ComposeProject>;

//

/// Get the system information of the target server.
/// Response: [SystemInformation].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetSystemInformationResponse)]
pub struct GetSystemInformation {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type GetSystemInformationResponse = SystemInformation;

//

/// Get the system stats on the target server. Response: [SystemStats].
///
/// Note. This does not hit the server directly. The stats come from an
/// in memory cache on the core, which hits the server periodically
/// to keep it up to date.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetSystemStatsResponse)]
pub struct GetSystemStats {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type GetSystemStatsResponse = SystemStats;

//

/// List the processes running on the target server.
/// Response: [ListSystemProcessesResponse].
///
/// Note. This does not hit the server directly. The procedures come from an
/// in memory cache on the core, which hits the server periodically
/// to keep it up to date.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListSystemProcessesResponse)]
pub struct ListSystemProcesses {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
}

#[typeshare]
pub type ListSystemProcessesResponse = Vec<SystemProcess>;

//

/// Paginated endpoint serving historical (timeseries) server stats for graphing.
/// Response: [GetHistoricalServerStatsResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetHistoricalServerStatsResponse)]
pub struct GetHistoricalServerStats {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub server: String,
  /// The granularity of the data.
  pub granularity: Timelength,
  /// Page of historical data. Default is 0, which is the most recent data.
  /// Use with the `next_page` field of the response.
  #[serde(default)]
  pub page: u32,
}

/// Response to [GetHistoricalServerStats].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHistoricalServerStatsResponse {
  /// The timeseries page of data.
  pub stats: Vec<SystemStatsRecord>,
  /// If there is a next page of data, pass this to `page` to get it.
  pub next_page: Option<u32>,
}

//

/// Gets a summary of data relating to all servers.
/// Response: [GetServersSummaryResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetServersSummaryResponse)]
pub struct GetServersSummary {}

/// Response for [GetServersSummary].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetServersSummaryResponse {
  /// The total number of servers.
  pub total: I64,
  /// The number of healthy (`status: OK`) servers.
  pub healthy: I64,
  /// The number of unhealthy servers.
  pub unhealthy: I64,
  /// The number of disabled servers.
  pub disabled: I64,
}
