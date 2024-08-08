use monitor_client::entities::{
  update::Log, CloneArgs, LatestCommit, SystemCommand,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(LatestCommit)]
pub struct GetLatestCommit {
  pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(RepoActionResponse)]
pub struct CloneRepo {
  pub args: CloneArgs,
  /// Override git token with one sent from core.
  pub git_token: Option<String>,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(RepoActionResponse)]
pub struct PullRepo {
  pub name: String,
  pub branch: Option<String>,
  pub commit: Option<String>,
  pub on_pull: Option<SystemCommand>,
}

//

/// Backward compat adapter for v1.13 upgrade.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum RepoActionResponse {
  V1_13(RepoActionResponseV1_13),
  V1_12(Vec<Log>),
}

impl From<RepoActionResponse> for RepoActionResponseV1_13 {
  fn from(value: RepoActionResponse) -> Self {
    match value {
      RepoActionResponse::V1_13(response) => response,
      RepoActionResponse::V1_12(logs) => RepoActionResponseV1_13 {
        logs,
        commit_hash: None,
        commit_message: None,
      },
    }
  }
}

impl From<RepoActionResponseV1_13> for RepoActionResponse {
  fn from(value: RepoActionResponseV1_13) -> Self {
    RepoActionResponse::V1_13(value)
  }
}

//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoActionResponseV1_13 {
  pub logs: Vec<Log>,
  pub commit_hash: Option<String>,
  pub commit_message: Option<String>,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct DeleteRepo {
  pub name: String,
}
