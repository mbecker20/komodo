use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::repo::{
  Repo, RepoActionState, RepoListItem, RepoQuery,
};

use super::MonitorReadRequest;

//

/// Get a specific repo. Response: [Repo].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(Repo)]
pub struct GetRepo {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub repo: String,
}

#[typeshare]
pub type GetRepoResponse = Repo;

//

/// List repos matching optional query. Response: [ListReposResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(Vec<RepoListItem>)]
pub struct ListRepos {
  /// optional structured query to filter repos.
  #[serde(default)]
  pub query: RepoQuery,
}

#[typeshare]
pub type ListReposResponse = Vec<RepoListItem>;

//

/// Get current action state for the repo. Response: [RepoActionState].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetRepoActionStateResponse)]
pub struct GetRepoActionState {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub repo: String,
}

#[typeshare]
pub type GetRepoActionStateResponse = RepoActionState;

//

/// Gets a summary of data relating to all repos.
/// Response: [GetReposSummaryResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetReposSummaryResponse)]
pub struct GetReposSummary {}

/// Response for [GetReposSummary]
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetReposSummaryResponse {
  /// The total number of repos
  pub total: u32,
  /// The number of repos with Ok state.
  pub ok: u32,
  /// The number of repos currently cloning.
  pub cloning: u32,
  /// The number of repos currently pulling.
  pub pulling: u32,
  /// The number of repos with failed state.
  pub failed: u32,
  /// The number of repos with unknown state.
  pub unknown: u32,
}
