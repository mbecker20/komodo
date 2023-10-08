use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::update::{Update, UpdateListItem},
    MongoDocument,
};

use super::MonitorReadRequest;

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetUpdateResponse)]
pub struct GetUpdate {
    pub id: String,
}

#[typeshare]
pub type GetUpdateResponse = Update;

//

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListUpdatesResponse)]
pub struct ListUpdates {
    pub query: Option<MongoDocument>,
    #[serde(default)]
    pub page: u32,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListUpdatesResponse {
    pub updates: Vec<UpdateListItem>,
    pub next_page: Option<u32>,
}
