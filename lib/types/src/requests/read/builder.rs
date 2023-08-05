use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{entities::builder::Builder, MongoDocument};

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Builder)]
pub struct GetBuilder {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<BuilderListItem>)]
pub struct ListBuilders {
    pub query: Option<MongoDocument>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuilderListItem {
    pub id: String,
    pub name: String,
    pub provider: String,
    pub instance_type: Option<String>,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetBuildersSummaryResponse)]
pub struct GetBuildersSummary {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBuildersSummaryResponse {
    pub total: u32,
}
