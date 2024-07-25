use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{builder::{
  Builder, BuilderListItem, BuilderQuery,
}, config::{DockerAccount, GitAccount}};

use super::MonitorReadRequest;

//

/// Get a specific builder by id or name. Response: [Builder].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetBuilderResponse)]
pub struct GetBuilder {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub builder: String,
}

#[typeshare]
pub type GetBuilderResponse = Builder;

//

/// List builders matching structured query. Response: [ListBuildersResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListBuildersResponse)]
pub struct ListBuilders {
  #[serde(default)]
  pub query: BuilderQuery,
}

#[typeshare]
pub type ListBuildersResponse = Vec<BuilderListItem>;

//

/// List builders matching structured query. Response: [ListFullBuildersResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListFullBuildersResponse)]
pub struct ListFullBuilders {
  #[serde(default)]
  pub query: BuilderQuery,
}

#[typeshare]
pub type ListFullBuildersResponse = Vec<Builder>;

//

/// Gets a summary of data relating to all builders.
/// Response: [GetBuildersSummaryResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetBuildersSummaryResponse)]
pub struct GetBuildersSummary {}

/// Response for [GetBuildersSummary].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBuildersSummaryResponse {
  /// The total number of builders.
  pub total: u32,
}

//

/// Get the docker / github accounts which are available for use on the builder.
/// Response: [GetBuilderAvailableAccountsResponse].
///
/// Note. Builds using this builder can only use the docker / github accounts available in this response.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetBuilderAvailableAccountsResponse)]
pub struct GetBuilderAvailableAccounts {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub builder: String,
}

/// Response for [GetBuilderAvailableAccounts].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBuilderAvailableAccountsResponse {
  pub git: Vec<GitAccount>,
  pub docker: Vec<DockerAccount>,
}
