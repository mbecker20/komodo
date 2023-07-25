use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::repo::{Repo, RepoActionState},
    MongoDocument, I64,
};

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Repo)]
pub struct GetRepo {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<RepoListItem>)]
pub struct ListRepos {
    pub query: Option<MongoDocument>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoListItem {
    pub id: String,
    pub name: String,
    pub last_pulled_at: I64,
    pub tags: Vec<String>,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(RepoActionState)]
pub struct GetRepoActionState {
    pub id: String,
}

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetReposSummaryResponse)]
pub struct GetReposSummary {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetReposSummaryResponse {}
