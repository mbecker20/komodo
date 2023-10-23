use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{entities::alert::Alert, MongoDocument, I64, U64};

use super::MonitorReadRequest;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListAlertsResponse)]
pub struct ListAlerts {
  pub query: Option<MongoDocument>,
  #[serde(default)]
  pub page: U64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListAlertsResponse {
  pub alerts: Vec<Alert>,
  pub next_page: Option<I64>,
}
