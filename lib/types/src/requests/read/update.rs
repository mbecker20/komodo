use resolver_api::derive::Request;
use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use crate::{entities::update::Update, MongoDocument};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Update>)]
pub struct ListUpdates {
    pub query: Option<MongoDocument>,
}