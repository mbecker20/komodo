use monitor_client::entities::{stack::Stack, update::Log};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ComposeResponse {
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
#[response(ComposeResponse)]
pub struct ComposeUp {
  /// The stack to deploy
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub registry_token: Option<String>,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeResponse)]
pub struct ComposeStart {
  /// The stack to start
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeResponse)]
pub struct ComposeRestart {
  /// The stack to restart
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeResponse)]
pub struct ComposePause {
  /// The stack to pause
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeResponse)]
pub struct ComposeUnpause {
  /// The stack to unpause
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeResponse)]
pub struct ComposeStop {
  /// The stack to stop
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// The timeout before killing the process. Optional
  pub timeout: Option<i32>,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeResponse)]
pub struct ComposeDown {
  /// The stack to bring down
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// Pass `--remove-orphans`.
  /// See https://docs.docker.com/reference/cli/docker/compose/down.
  pub remove_orphans: bool,
  /// The timeout before killing the process. Optional
  pub timeout: Option<i32>,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeResponse)]
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
#[response(ComposeResponse)]
pub struct ComposeServiceStart {
  /// The stack to start
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// The service name
  pub service: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeResponse)]
pub struct ComposeServiceRestart {
  /// The stack to restart
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// The service name
  pub service: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeResponse)]
pub struct ComposeServicePause {
  /// The stack to pause
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// The service name
  pub service: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeResponse)]
pub struct ComposeServiceUnpause {
  /// The stack to unpause
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// The service name
  pub service: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeResponse)]
pub struct ComposeServiceStop {
  /// The stack to stop
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// The service name
  pub service: String,
  /// The timeout before killing the process. Optional
  pub timeout: Option<i32>,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(ComposeResponse)]
pub struct ComposeServiceDown {
  /// The stack to bring down
  pub stack: Stack,
  /// If provided, use it to login in. Otherwise check periphery local registries.
  pub git_token: Option<String>,
  /// The service name
  pub service: String,
  /// Pass `--remove-orphans`.
  /// See https://docs.docker.com/reference/cli/docker/compose/down.
  pub remove_orphans: bool,
  /// The timeout before killing the process. Optional
  pub timeout: Option<i32>,
}
