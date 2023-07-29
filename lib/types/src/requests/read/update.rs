use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{entities::update::Update, MongoDocument};

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
    pub updates: Vec<Update>,
    pub next_page: Option<u32>
}
