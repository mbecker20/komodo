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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoActionResponse {
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
