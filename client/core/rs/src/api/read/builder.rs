use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::builder::{
  Builder, BuilderListItem, BuilderQuery,
};

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
