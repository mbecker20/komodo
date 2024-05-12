use monitor_client::entities::{
  update::Log, CloneArgs, SystemCommand,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(GetLatestCommitResponse)]
pub struct GetLatestCommit {
  pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLatestCommitResponse {
  pub hash: String,
  pub message: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Log>)]
pub struct CloneRepo {
  pub args: CloneArgs,
  /// Override github token with one sent from core.
  pub github_token: Option<String>,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<Log>)]
pub struct PullRepo {
  pub name: String,
  pub branch: Option<String>,
  pub on_pull: Option<SystemCommand>,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct DeleteRepo {
  pub name: String,
}
