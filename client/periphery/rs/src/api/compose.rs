use komodo_client::entities::{
  stack::{ComposeProject, Stack, StackServiceNames},
  update::Log,
  FileContents, SearchCombinator,
};
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};

use super::git::RepoActionResponse;

/// List the compose project names that are on the host.
/// List running `docker compose ls`
///
/// Incoming from docker like:
/// [{"Name":"project_name","Status":"running(1)","ConfigFiles":"/root/compose/compose.yaml,/root/compose/compose2.yaml"}]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[response(Vec<ComposeProject>)]
#[error(serror::Error)]
pub struct ListComposeProjects {}

//

/// Get the compose contents on the host, for stacks using
/// `files_on_host`.
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[response(GetComposeContentsOnHostResponse)]
#[error(serror::Error)]
pub struct GetComposeContentsOnHost {
  /// The name of the stack
  pub name: String,
  pub run_directory: String,
  pub file_paths: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetComposeContentsOnHostResponse {
  pub contents: Vec<FileContents>,
  pub errors: Vec<FileContents>,
}

//

/// The stack folder must already exist for this to work
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct GetComposeServiceLog {
  /// The name of the project
  pub project: String,
  /// The service name
  pub service: String,
  /// Pass `--tail` for only recent log contents. Max of 5000
  #[serde(default = "default_tail")]
  pub tail: u64,
  /// Enable `--timestamps`
  #[serde(default)]
  pub timestamps: bool,
}

fn default_tail() -> u64 {
  50
}

//

/// The stack folder must already exist for this to work
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct GetComposeServiceLogSearch {
  /// The name of the project
  pub project: String,
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
  /// Enable `--timestamps`
  #[serde(default)]
  pub timestamps: bool,
}

//

/// Write the compose contents to the file on the host, for stacks using
/// `files_on_host`.
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct WriteComposeContentsToHost {
  /// The name of the stack
  pub name: String,
  /// The run directory of the stack
  pub run_directory: String,
  /// Relative to the stack folder + run directory,
  /// or absolute path.
  pub file_path: String,
  /// The contents to write.
  pub contents: String,
}

//

/// Write and commit compose contents.
/// Only works with git repo based stacks.
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[response(RepoActionResponse)]
#[error(serror::Error)]
pub struct WriteCommitComposeContents {
  /// The stack to write to.
  pub stack: Stack,
  /// The username of user which committed the file.
  pub username: Option<String>,
  /// Relative to the stack folder + run directory.
  pub file_path: String,
  /// The contents to write.
  pub contents: String,
  /// If provided, use it to login in. Otherwise check periphery local git providers.
  pub git_token: Option<String>,
}

//

/// Rewrites the compose directory, pulls any images, takes down existing containers,
/// and runs docker compose up. Response: [ComposePullResponse]
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[response(ComposePullResponse)]
#[error(serror::Error)]
pub struct ComposePull {
  /// The stack to deploy
  pub stack: Stack,
  /// Only deploy one service
  pub service: Option<String>,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// If provided, use it to login in. Otherwise check periphery local git providers.
  pub registry_token: Option<String>,
}

/// Response for [ComposePull]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComposePullResponse {
  pub logs: Vec<Log>,
}

//

/// docker compose up.
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[response(ComposeUpResponse)]
#[error(serror::Error)]
pub struct ComposeUp {
  /// The stack to deploy
  pub stack: Stack,
  /// Only deploy one service
  pub service: Option<String>,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// If provided, use it to login in. Otherwise check periphery local git providers.
  pub registry_token: Option<String>,
  /// Propogate any secret replacers from core interpolation.
  #[serde(default)]
  pub replacers: Vec<(String, String)>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComposeUpResponse {
  /// If any of the required files are missing, they will be here.
  pub missing_files: Vec<String>,
  /// The logs produced by the deploy
  pub logs: Vec<Log>,
  /// Whether stack was successfully deployed
  pub deployed: bool,
  /// The stack services.
  ///
  /// Note. The "image" is after interpolation.
  #[serde(default)]
  pub services: Vec<StackServiceNames>,
  /// The deploy compose file contents if they could be acquired, or empty vec.
  pub file_contents: Vec<FileContents>,
  /// The error in getting remote file contents at the path, or null
  pub remote_errors: Vec<FileContents>,
  /// If its a repo based stack, will include the latest commit hash
  pub commit_hash: Option<String>,
  /// If its a repo based stack, will include the latest commit message
  pub commit_message: Option<String>,
}

//

/// General compose command runner
#[derive(Debug, Clone, Serialize, Deserialize, Resolve)]
#[response(Log)]
#[error(serror::Error)]
pub struct ComposeExecution {
  /// The compose project name to run the execution on.
  /// Usually its he name of the stack / folder under the `stack_dir`.
  pub project: String,
  /// The command in `docker compose -p {project} {command}`
  pub command: String,
}
