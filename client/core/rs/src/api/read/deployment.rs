use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  deployment::{
    Deployment, DeploymentActionState, DeploymentListItem,
    DeploymentQuery, DeploymentState,
  },
  docker::container::{ContainerListItem, ContainerStats},
  update::Log,
  SearchCombinator, I64, U64,
};

use super::MonitorReadRequest;

//

/// Get a specific deployment by name or id. Response: [Deployment].
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

/// List deployments matching optional query.
/// Response: [ListDeploymentsResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListDeploymentsResponse)]
pub struct ListDeployments {
  /// optional structured query to filter deployments.
  #[serde(default)]
  pub query: DeploymentQuery,
}

#[typeshare]
pub type ListDeploymentsResponse = Vec<DeploymentListItem>;

//

/// List deployments matching optional query.
/// Response: [ListFullDeploymentsResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListFullDeploymentsResponse)]
pub struct ListFullDeployments {
  /// optional structured query to filter deployments.
  #[serde(default)]
  pub query: DeploymentQuery,
}

#[typeshare]
pub type ListFullDeploymentsResponse = Vec<Deployment>;

//

/// Get the container, including image / status, of the target deployment.
/// Response: [GetDeploymentContainerResponse].
///
/// Note. This does not hit the server directly. The status comes from an
/// in memory cache on the core, which hits the server periodically
/// to keep it up to date.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDeploymentContainerResponse)]
pub struct GetDeploymentContainer {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub deployment: String,
}

/// Response for [GetDeploymentContainer].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDeploymentContainerResponse {
  pub state: DeploymentState,
  pub container: Option<ContainerListItem>,
}

//

/// Get the deployment log's tail, split by stdout/stderr.
/// Response: [Log].
///
/// Note. This call will hit the underlying server directly for most up to date log.
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
  /// The number of lines of the log tail to include.
  /// Default: 100.
  /// Max: 5000.
  #[serde(default = "default_tail")]
  pub tail: U64,
}

fn default_tail() -> u64 {
  50
}

#[typeshare]
pub type GetLogResponse = Log;

//

/// Search the deployment log's tail using `grep`. All lines go to stdout.
/// Response: [Log].
///
/// Note. This call will hit the underlying server directly for most up to date log.
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
  /// The terms to search for.
  pub terms: Vec<String>,
  /// When searching for multiple terms, can use `AND` or `OR` combinator.
  ///
  /// - `AND`: Only include lines with **all** terms present in that line.
  /// - `OR`: Include lines that have one or more matches in the terms.
  #[serde(default)]
  pub combinator: SearchCombinator,
  /// Invert the results, ie return all lines that DON'T match the terms / combinator.
  #[serde(default)]
  pub invert: bool,
}

#[typeshare]
pub type SearchLogResponse = Log;

//

/// Get the deployment container's stats using `docker stats`.
/// Response: [DockerContainerStats].
///
/// Note. This call will hit the underlying server directly for most up to date stats.
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
pub type GetDeploymentStatsResponse = ContainerStats;

//

/// Get current action state for the deployment.
/// Response: [DeploymentActionState].
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

/// Gets a summary of data relating to all deployments.
/// Response: [GetDeploymentsSummaryResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetDeploymentsSummaryResponse)]
pub struct GetDeploymentsSummary {}

/// Response for [GetDeploymentsSummary].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetDeploymentsSummaryResponse {
  pub total: I64,
  pub running: I64,
  pub stopped: I64,
  pub not_deployed: I64,
  pub unknown: I64,
}

//

/// Gets a list of existing values used as extra args across other deployments.
/// Useful to offer suggestions. Response: [ListCommonDeploymentExtraArgsResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListCommonDeploymentExtraArgsResponse)]
pub struct ListCommonDeploymentExtraArgs {
  /// optional structured query to filter deployments.
  #[serde(default)]
  pub query: DeploymentQuery,
}

#[typeshare]
pub type ListCommonDeploymentExtraArgsResponse = Vec<String>;
