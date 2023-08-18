use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::{
        deployment::{
            Deployment, DeploymentActionState, DeploymentListItem, DockerContainerState,
            DockerContainerStats,
        },
        update::Log,
    },
    MongoDocument, I64, U64,
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

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetDeploymentsSummaryResponse)]
pub struct GetDeploymentsSummary {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetDeploymentsSummaryResponse {
    pub total: I64,
    pub running: I64,
    pub stopped: I64,
    pub not_deployed: I64,
    pub unknown: I64,
}
