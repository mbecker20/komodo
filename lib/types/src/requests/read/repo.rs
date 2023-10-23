use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
  entities::repo::{Repo, RepoActionState, RepoListItem},
  MongoDocument,
};

use super::MonitorReadRequest;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(Repo)]
pub struct GetRepo {
  pub id: String,
}

#[typeshare]
pub type GetRepoResponse = Repo;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(Vec<RepoListItem>)]
pub struct ListRepos {
  pub query: Option<MongoDocument>,
}

#[typeshare]
pub type ListReposResponse = Vec<RepoListItem>;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetRepoActionStateResponse)]
pub struct GetRepoActionState {
  pub id: String,
}

#[typeshare]
pub type GetRepoActionStateResponse = RepoActionState;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetReposSummaryResponse)]
pub struct GetReposSummary {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetReposSummaryResponse {
  pub total: u32,
}
