use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{entities::update::Update, MongoDocument};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Update>)]
pub struct ListUpdates {
    pub query: Option<MongoDocument>,
}
