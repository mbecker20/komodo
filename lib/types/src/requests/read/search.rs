use resolver_api::derive::Request;
use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use crate::entities::tag::Tag;

use super::{ServerListItem, DeploymentListItem, BuildListItem, RepoListItem};

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(FindResourcesResponse)]
pub struct FindResources {
	pub tags: Vec<Tag>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FindResourcesResponse {
	pub servers: Vec<ServerListItem>,
	pub deployments: Vec<DeploymentListItem>,
	pub builds: Vec<BuildListItem>,
	pub repos: Vec<RepoListItem>,
}