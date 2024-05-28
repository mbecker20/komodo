use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::alerter::{
  Alerter, AlerterListItem, AlerterQuery,
};

use super::MonitorReadRequest;

//

/// Get a specific alerter. Response: [Alerter].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetAlerterResponse)]
pub struct GetAlerter {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub alerter: String,
}

#[typeshare]
pub type GetAlerterResponse = Alerter;

//

/// List alerters matching optional query. Response: [ListAlertersResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListAlertersResponse)]
pub struct ListAlerters {
  /// Structured query to filter alerters.
  #[serde(default)]
  pub query: AlerterQuery,
}

#[typeshare]
pub type ListAlertersResponse = Vec<AlerterListItem>;

/// List full alerters matching optional query. Response: [ListFullAlertersResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListFullAlertersResponse)]
pub struct ListFullAlerters {
  /// Structured query to filter alerters.
  #[serde(default)]
  pub query: AlerterQuery,
}

#[typeshare]
pub type ListFullAlertersResponse = Vec<Alerter>;

//

/// Gets a summary of data relating to all alerters.
/// Response: [GetAlertersSummaryResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetAlertersSummaryResponse)]
pub struct GetAlertersSummary {}

/// Response for [GetAlertersSummary].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAlertersSummaryResponse {
  pub total: u32,
}
