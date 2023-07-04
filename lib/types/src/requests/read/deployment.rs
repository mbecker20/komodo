use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::{
        deployment::{Deployment, DeploymentActionState, DockerContainerStats},
        update::Log,
    },
    MongoDocument,
};

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Deployment)]
pub struct GetDeployment {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Deployment>)]
pub struct ListDeployments {
    pub query: Option<MongoDocument>,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct GetLog {
    pub deployment_id: String,
    #[serde(default = "default_tail")]
    pub tail: u64,
}

fn default_tail() -> u64 {
    50
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetDeployedVersionResponse)]
pub struct GetDeployedVersion {
    pub deployment_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDeployedVersionResponse {
    pub version: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(DockerContainerStats)]
pub struct GetDeploymentStats {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(DeploymentActionState)]
pub struct GetDeploymentActionState {
    pub id: String,
}
