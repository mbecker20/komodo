use monitor_client::entities::update::Log;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
pub struct ComposeUp {
  /// The compose file contents
  pub file: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
pub struct ComposeStart {
  /// The compose file contents
  pub file: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
pub struct ComposeRestart {
  /// The compose file contents
  pub file: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
pub struct ComposePause {
  /// The compose file contents
  pub file: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
pub struct ComposeStop {
  /// The compose file contents
  pub file: String,
  /// The timeout before killing the process. Optional
  pub timeout: Option<i32>,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
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
#[response(Log)]
pub struct ComposeServiceUp {
  /// The compose file contents
  pub file: String,
  /// The service name
  pub service: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
pub struct ComposeServiceStart {
  /// The compose file contents
  pub file: String,
  /// The service name
  pub service: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
pub struct ComposeServiceRestart {
  /// The compose file contents
  pub file: String,
  /// The service name
  pub service: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
pub struct ComposeServicePause {
  /// The compose file contents
  pub file: String,
  /// The service name
  pub service: String,
}

//

#[derive(Debug, Clone, Serialize, Deserialize, Request)]
#[response(Log)]
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
#[response(Log)]
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
