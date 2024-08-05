use std::path::PathBuf;

use monitor_client::entities::{
  stack::Stack, update::Log, SearchCombinator,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

//

/// Get the compose file health, contents, json
#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(GetComposeInfoReponse)]
pub struct GetComposeInfo {
  /// The stack name, to get the root folder.
  pub name: String,
  /// The run directory. Relative to root of the folder.
  pub run_directory: String,
  /// The compose file path to check.
  /// Relative to `run_directory`.
  pub file_path: String,
  // The compose project name
  pub project: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetComposeInfoReponse {
  /// If the file is missing. Everything else will be null in this case.
  pub file_missing: bool,
  /// The compose project is missing on the host
  pub project_missing: bool,
  // /// The compose file contents.
  // pub file_contents: Option<String>,
  // /// If there was an error in getting the contents.
  // pub content_error: Option<String>,
  // /// The compose file json representation.
  // pub json: Option<String>,
  // /// If there was an error in getting the compose file json representation.
  // pub json_error: Option<String>,
  // /// If its a repo based stack, will include the latest commit hash
  // pub commit_hash: Option<String>,
  // /// If its a repo based stack, will include the latest commit message
  // pub commit_message: Option<String>,
}

//

/// The stack folder must already exist for this to work
#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
pub struct GetComposeServiceLog {
  /// The name of the stack (always set as the compose project name)
  pub name: String,
  /// The path of the compose file relative to periphery `stack_dir`.
  pub run_directory: PathBuf,
  /// The path of the compose file, relative to the run path.
  pub file_path: String,
  /// The service name
  pub service: String,
  /// pass `--tail` for only recent log contents
  #[serde(default = "default_tail")]
  pub tail: u64,
}

fn default_tail() -> u64 {
  50
}

//

/// The stack folder must already exist for this to work
#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
pub struct GetComposeServiceLogSearch {
  /// The name of the stack (always set as the compose project name)
  pub name: String,
  /// The path of the compose file relative to periphery `stack_dir`.
  pub run_directory: PathBuf,
  /// The path of the compose file, relative to the run path.
  pub file_path: String,
  /// The service name
  pub service: String,
  /// The search terms.
  pub terms: Vec<String>,
  /// And: Only lines matching all terms
  /// Or: Lines matching any one of the terms
  #[serde(default)]
  pub combinator: SearchCombinator,
  /// Invert the search (search for everything not matching terms)
  #[serde(default)]
  pub invert: bool,
}

//

/// Rewrites the compose directory, pulls any images, takes down existing containers,
/// and runs docker compose up.
#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeUpResponse)]
pub struct ComposeUp {
  /// The stack to deploy
  pub stack: Stack,
  /// Only deploy one service
  pub service: Option<String>,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub registry_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComposeUpResponse {
  /// If the file is missing
  pub file_missing: bool,
  /// The logs produced by the deploy
  pub logs: Vec<Log>,
  /// whether stack was successfully deployed
  pub deployed: bool,
  /// The deploy compose file contents if they could be acquired, or null.
  pub file_contents: Option<String>,
  /// The error in getting remote file contents, or null
  pub remote_error: Option<String>,
  /// If its a repo based stack, will include the latest commit hash
  pub commit_hash: Option<String>,
  /// If its a repo based stack, will include the latest commit message
  pub commit_message: Option<String>,
}

//

/// General compose command runner
#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
pub struct ComposeExecution {
  /// The compose project name to run the execution on.
  /// Usually its he name of the stack / folder under the `stack_dir`.
  pub project: String,
  /// The command in `docker compose -p {project} {command}`
  pub command: String,
}
