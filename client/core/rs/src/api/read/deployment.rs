use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  deployment::{
    Deployment, DeploymentActionState, DeploymentListItem,
    DeploymentQuery, DockerContainerState, DockerContainerStats,
  },
  update::Log,
  I64, U64,
};

use super::MonitorReadRequest;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDeploymentResponse)]
pub struct GetDeployment {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub deployment: String,
}

#[typeshare]
pub type GetDeploymentResponse = Deployment;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListDeploymentsResponse)]
pub struct ListDeployments {
  #[serde(default)]
  pub query: DeploymentQuery,
}

#[typeshare]
pub type ListDeploymentsResponse = Vec<DeploymentListItem>;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDeploymentStatusResponse)]
pub struct GetDeploymentStatus {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub deployment: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDeploymentStatusResponse {
  pub state: DockerContainerState,
  pub status: Option<String>,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetLogResponse)]
pub struct GetLog {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub deployment: String,
  #[serde(default = "default_tail")]
  pub tail: U64,
}

fn default_tail() -> u64 {
  50
}

#[typeshare]
pub type GetLogResponse = Log;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(SearchLogResponse)]
pub struct SearchLog {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub deployment: String,
  pub search: String,
}

#[typeshare]
pub type SearchLogResponse = Log;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDeployedVersionResponse)]
pub struct GetDeployedVersion {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub deployment: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDeployedVersionResponse {
  pub version: String,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDeploymentStatsResponse)]
pub struct GetDeploymentStats {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub deployment: String,
}

#[typeshare]
pub type GetDeploymentStatsResponse = DockerContainerStats;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(DeploymentActionState)]
pub struct GetDeploymentActionState {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub deployment: String,
}

#[typeshare]
pub type GetDeploymentActionStateResponse = DeploymentActionState;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDeploymentsSummaryResponse)]
pub struct GetDeploymentsSummary {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetDeploymentsSummaryResponse {
  pub total: I64,
  pub running: I64,
  pub stopped: I64,
  pub not_deployed: I64,
  pub unknown: I64,
}
