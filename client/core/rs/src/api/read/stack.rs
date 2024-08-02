use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  deployment::ContainerSummary,
  stack::{Stack, StackActionState, StackListItem, StackQuery},
  U64,
};

use super::MonitorReadRequest;

//

/// Get a specific stack. Response: [Stack].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetStackResponse)]
pub struct GetStack {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub stack: String,
}

#[typeshare]
pub type GetStackResponse = Stack;

//

/// Get a stacks compose-file JSON representation. Response: [serde_json::Value]
/// (No schema provided for this, it comes from docker).
///
/// Obtained through [docker compose config --format json](https://docs.docker.com/reference/cli/docker/compose/config/).
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetStackJsonResponse)]
pub struct GetStackJson {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub stack: String,
}

/// Response for [GetStackJson]
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetStackJsonResponse {
  pub json: serde_json::Value,
  pub error: bool,
}

//

/// Get a specific stacks containers. Response: [GetStackContainersResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetStackContainersResponse)]
pub struct GetStackContainers {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub stack: String,
}

#[typeshare]
pub type GetStackContainersResponse = Vec<ContainerSummary>;

/// Get a stack service's log. Response: [GetStackContainersResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetStackServiceLogResponse)]
pub struct GetStackServiceLog {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub stack: String,
  /// The service to get the log for.
  pub service: String,
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
pub type GetStackServiceLogResponse = Vec<ContainerSummary>;

//

/// List stacks matching optional query. Response: [ListStacksResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListStacksResponse)]
pub struct ListStacks {
  /// optional structured query to filter syncs.
  #[serde(default)]
  pub query: StackQuery,
}

#[typeshare]
pub type ListStacksResponse = Vec<StackListItem>;

//

/// List stacks matching optional query. Response: [ListFullStacksResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListFullStacksResponse)]
pub struct ListFullStacks {
  /// optional structured query to filter stacks.
  #[serde(default)]
  pub query: StackQuery,
}

#[typeshare]
pub type ListFullStacksResponse = Vec<Stack>;

//

/// Get current action state for the stack. Response: [StackActionState].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetStackActionStateResponse)]
pub struct GetStackActionState {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub stack: String,
}

#[typeshare]
pub type GetStackActionStateResponse = StackActionState;

//

/// Gets a summary of data relating to all syncs.
/// Response: [GetStacksSummaryResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetStacksSummaryResponse)]
pub struct GetStacksSummary {}

/// Response for [GetStacksSummary]
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetStacksSummaryResponse {
  /// The total number of syncs
  pub total: u32,
  /// The number of syncs with Healthy state.
  pub healthy: u32,
  /// The number of syncs with Unhealthy state.
  pub unhealthy: u32,
  /// The number of syncs with Down state.
  pub down: u32,
  /// The number of syncs with Failed state.
  pub failed: u32,
  /// The number of syncs with Unknown state.
  pub unknown: u32,
}

//

/// Get a target stack's configured webhooks. Response: [GetStackWebhooksEnabledResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetStackWebhooksEnabledResponse)]
pub struct GetStackWebhooksEnabled {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub stack: String,
}

/// Response for [GetStackWebhooksEnabled]
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetStackWebhooksEnabledResponse {
  /// Whether the repo webhooks can even be managed.
  /// The repo owner must be in `github_webhook_app.owners` list to be managed.
  pub managed: bool,
  /// Whether pushes to branch trigger refresh. Will always be false if managed is false.
  pub refresh_enabled: bool,
  /// Whether pushes to branch trigger stack execution. Will always be false if managed is false.
  pub deploy_enabled: bool,
}
