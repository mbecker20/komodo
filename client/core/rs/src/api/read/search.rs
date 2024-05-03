use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  build::BuildListItem, deployment::DeploymentListItem,
  procedure::ProcedureListItem, repo::RepoListItem,
  server::ServerListItem, update::ResourceTargetVariant,
  MongoDocument,
};

use super::MonitorReadRequest;

//

/// Find resources matching a common query. Response: [FindResourcesResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(FindResourcesResponse)]
pub struct FindResources {
  /// The mongo query as JSON
  #[serde(default)]
  pub query: MongoDocument,
  /// The resource variants to include in the response.
  #[serde(default)]
  pub resources: Vec<ResourceTargetVariant>,
}

/// Response for [FindResources].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FindResourcesResponse {
  /// The matching servers.
  pub servers: Vec<ServerListItem>,
  /// The matching deployments.
  pub deployments: Vec<DeploymentListItem>,
  /// The matching builds.
  pub builds: Vec<BuildListItem>,
  /// The matching repos.
  pub repos: Vec<RepoListItem>,
  /// The matching procedures.
  pub procedures: Vec<ProcedureListItem>,
}
