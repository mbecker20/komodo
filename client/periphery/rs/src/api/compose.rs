use monitor_client::entities::{stack::Stack, update::Log};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeUpResponse)]
pub struct ComposeUp {
  /// The stack to deploy
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub registry_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComposeUpResponse {
  /// The logs produced by the deploy
  pub logs: Vec<Log>,
  /// whether stack was successfully deployed
  pub deployed: bool,
  /// The deploy compose file contents if they could be acquired, or null.
  pub file_contents: Option<String>,
  /// If its a repo based stack, will include the latest commit hash
  pub commit_hash: Option<String>,
  /// If its a repo based stack, will include the latest commit message
  pub commit_message: Option<String>,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<Log>)]
pub struct ComposeStart {
  /// The compose file contents
  pub file: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<Log>)]
pub struct ComposeRestart {
  /// The compose file contents
  pub file: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<Log>)]
pub struct ComposePause {
  /// The compose file contents
  pub file: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<Log>)]
pub struct ComposeUnpause {
  /// The compose file contents
  pub file: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<Log>)]
pub struct ComposeStop {
  /// The compose file contents
  pub file: String,
  /// The timeout before killing the process. Optional
  pub timeout: Option<i32>,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<Log>)]
pub struct ComposeDown {
  /// The compose file contents
  pub file: String,
  /// Pass `--remove-orphans`.
  /// See https://docs.docker.com/reference/cli/docker/compose/down.
  pub remove_orphans: bool,
  /// The timeout before killing the process. Optional
  pub timeout: Option<i32>,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeUpResponse)]
pub struct ComposeServiceUp {
  /// The stack to deploy
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub registry_token: Option<String>,
  /// The service name
  pub service: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<Log>)]
pub struct ComposeServiceStart {
  /// The compose file contents
  pub file: String,
  /// The service name
  pub service: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<Log>)]
pub struct ComposeServiceRestart {
  /// The compose file contents
  pub file: String,
  /// The service name
  pub service: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<Log>)]
pub struct ComposeServicePause {
  /// The compose file contents
  pub file: String,
  /// The service name
  pub service: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<Log>)]
pub struct ComposeServiceUnpause {
  /// The compose file contents
  pub file: String,
  /// The service name
  pub service: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<Log>)]
pub struct ComposeServiceStop {
  /// The compose file contents
  pub file: String,
  /// The service name
  pub service: String,
  /// The timeout before killing the process. Optional
  pub timeout: Option<i32>,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Vec<Log>)]
pub struct ComposeServiceDown {
  /// The compose file contents
  pub file: String,
  /// The service name
  pub service: String,
  /// Pass `--remove-orphans`.
  /// See https://docs.docker.com/reference/cli/docker/compose/down.
  pub remove_orphans: bool,
  /// The timeout before killing the process. Optional
  pub timeout: Option<i32>,
}
