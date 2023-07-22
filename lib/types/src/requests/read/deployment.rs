use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::{
        deployment::{
            Deployment, DeploymentActionState, DockerContainerState, DockerContainerStats,
        },
        update::Log,
    },
    MongoDocument, U64,
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
#[response(Vec<DeploymentListItem>)]
pub struct ListDeployments {
    pub query: Option<MongoDocument>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeploymentListItem {
    pub id: String,
    pub name: String,
    pub tags: Vec<String>,
    pub state: DockerContainerState,
    pub status: Option<String>,
    pub image: String,
    pub version: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetDeploymentStatusResponse)]
pub struct GetDeploymentStatus {
    pub id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDeploymentStatusResponse {
    pub state: DockerContainerState,
    pub status: Option<String>,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct GetLog {
    pub deployment_id: String,
    #[serde(default = "default_tail")]
    pub tail: U64,
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
