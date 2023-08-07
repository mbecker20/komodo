use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::update::{Update, UpdateListItem},
    MongoDocument,
};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct GetUpdate {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
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
