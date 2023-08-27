use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{entities::alert::Alert, I64, U64, MongoDocument};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
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
