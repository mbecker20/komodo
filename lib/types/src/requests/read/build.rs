use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::{
        build::{Build, BuildActionState},
        Version,
    },
    MongoDocument, I64,
};

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Build)]
pub struct GetBuild {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<BuildListItem>)]
pub struct ListBuilds {
    pub query: Option<MongoDocument>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildListItem {
    pub id: String,
    pub name: String,
    pub last_built_at: I64,
    pub version: Version,
    pub tags: Vec<String>,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(BuildActionState)]
pub struct GetBuildActionState {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetBuildsSummaryResponse)]
pub struct GetBuildsSummary {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBuildsSummaryResponse {
    pub total: u32,
}
