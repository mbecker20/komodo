use monitor_macros::derive_crud_requests;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::{
        build::{Build, PartialBuildConfig},
        update::Update,
    },
    MongoDocument,
};

//

derive_crud_requests!(Build);

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct RunBuild {
    pub build_id: String,
}
