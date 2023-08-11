use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{entities::builder::{Builder, BuilderListItem}, MongoDocument};

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

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetBuilderAvailableAccountsResponse)]
pub struct GetBuilderAvailableAccounts {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBuilderAvailableAccountsResponse {
    pub github: Vec<String>,
    pub docker: Vec<String>,
}