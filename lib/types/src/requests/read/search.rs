use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{entities::update::ResourceTargetVariant, MongoDocument};

use super::{BuildListItem, DeploymentListItem, RepoListItem, ServerListItem};

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(FindResourcesResponse)]
pub struct FindResources {
    pub query: Option<MongoDocument>,
    pub resources: Option<Vec<ResourceTargetVariant>>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FindResourcesResponse {
    pub servers: Vec<ServerListItem>,
    pub deployments: Vec<DeploymentListItem>,
    pub builds: Vec<BuildListItem>,
    pub repos: Vec<RepoListItem>,
}
