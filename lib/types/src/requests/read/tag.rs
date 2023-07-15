use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{entities::tag::CustomTag, MongoDocument};

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(CustomTag)]
pub struct GetTag {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<CustomTag>)]
pub struct ListTags {
    pub query: Option<MongoDocument>,
}
