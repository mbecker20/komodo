use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::alerter::{Alerter, AlerterListItem},
    MongoDocument,
};

use super::MonitorReadRequest;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetAlerterResponse)]
pub struct GetAlerter {
    pub id: String,
}

#[typeshare]
pub type GetAlerterResponse = Alerter;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListAlertersResponse)]
pub struct ListAlerters {
    pub query: Option<MongoDocument>,
}

#[typeshare]
pub type ListAlertersResponse = Vec<AlerterListItem>;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetAlertersSummaryResponse)]
pub struct GetAlertersSummary {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAlertersSummaryResponse {
    pub total: u32,
}
