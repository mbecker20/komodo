use std::path::PathBuf;

use komodo_client::entities::{
  update::Log, CloneArgs, EnvironmentVar, LatestCommit, SystemCommand,
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
  #[serde(default)]
  pub environment: Vec<EnvironmentVar>,
  #[serde(default = "default_env_file_path")]
  pub env_file_path: String,
  #[serde(default)]
  pub skip_secret_interp: bool,
  /// Override git token with one sent from core.
  pub git_token: Option<String>,
  /// Propogate any secret replacers from core interpolation.
  #[serde(default)]
  pub replacers: Vec<(String, String)>,
}

fn default_env_file_path() -> String {
  String::from(".env")
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(RepoActionResponse)]
pub struct PullRepo {
  pub name: String,
  pub branch: Option<String>,
  pub commit: Option<String>,
  pub on_pull: Option<SystemCommand>,
  #[serde(default)]
  pub environment: Vec<EnvironmentVar>,
  #[serde(default = "default_env_file_path")]
  pub env_file_path: String,
  #[serde(default)]
  pub skip_secret_interp: bool,
  /// Propogate any secret replacers from core interpolation.
  #[serde(default)]
  pub replacers: Vec<(String, String)>,
}

//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RepoActionResponse {
  pub logs: Vec<Log>,
  pub commit_hash: Option<String>,
  pub commit_message: Option<String>,
  /// Don't need to send this one to core, its only needed for calls local to single periphery
  #[serde(skip_serializing)]
  pub env_file_path: Option<PathBuf>,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct DeleteRepo {
  pub name: String,
}
