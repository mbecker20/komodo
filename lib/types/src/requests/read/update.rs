use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::{
        update::{ResourceTarget, UpdateStatus},
        Operation, Version,
    },
    MongoDocument, I64,
};

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

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateListItem {
    pub id: String,
    pub operation: Operation,
    pub start_ts: I64,
    pub success: bool,
    pub operator: String,
    pub operator_id: String,
    pub target: ResourceTarget,
    pub status: UpdateStatus,
    pub version: Version,
}
