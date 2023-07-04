use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::{
        build::{Build, PartialBuildConfig, BuildActionState},
        update::Update,
    },
    MongoDocument,
};

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Build)]
pub struct GetBuild {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Build>)]
pub struct ListBuilds {
    pub query: Option<MongoDocument>,
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
#[response(Build)]
pub struct CreateBuild {
    pub name: String,
    pub config: PartialBuildConfig,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Build)]
pub struct CopyBuild {
    pub name: String,
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Build)]
pub struct DeleteBuild {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Build)]
pub struct UpdateBuild {
    pub id: String,
    pub config: PartialBuildConfig,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct RunBuild {
    pub build_id: String,
}
