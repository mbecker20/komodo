use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::builder::{
  Builder, BuilderListItem, BuilderQuery,
};

use super::MonitorReadRequest;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetBuilderResponse)]
pub struct GetBuilder {
  pub id: String,
}

#[typeshare]
pub type GetBuilderResponse = Builder;

//

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

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetBuildersSummaryResponse)]
pub struct GetBuildersSummary {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBuildersSummaryResponse {
  pub total: u32,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetBuilderAvailableAccountsResponse)]
pub struct GetBuilderAvailableAccounts {
  pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBuilderAvailableAccountsResponse {
  pub github: Vec<String>,
  pub docker: Vec<String>,
}
