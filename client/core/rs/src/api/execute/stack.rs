use clap::Parser;
use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::update::Update;

use super::MonitorExecuteRequest;

/// Deploys the target stack. `docker compose up`. Response: [Update]
///
/// Note. If the stack is already deployed, it will be destroyed first.
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct DeployStack {
  /// Id or name
  pub stack: String,
  /// Optionally deploy only a specific service. Experimental.
  pub service: Option<String>,
  /// Override the default termination max time.
  /// Only used if the stack needs to be taken down first.
  pub stop_time: Option<i32>,
}

//

/// Starts the target stack. `docker compose start`. Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct StartStack {
  /// Id or name
  pub stack: String,
  /// Optionally specify a specific service to start
  pub service: Option<String>,
}

//

/// Restarts the target stack. `docker compose restart`. Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct RestartStack {
  /// Id or name
  pub stack: String,
  /// Optionally specify a specific service to restart
  pub service: Option<String>,
}

//

/// Pauses the target stack. `docker compose pause`. Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct PauseStack {
  /// Id or name
  pub stack: String,
  /// Optionally specify a specific service to pause
  pub service: Option<String>,
}

//

/// Unpauses the target stack. `docker compose unpause`. Response: [Update].
///
/// Note. This is the only way to restart a paused container.
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct UnpauseStack {
  /// Id or name
  pub stack: String,
  /// Optionally specify a specific service to unpause
  pub service: Option<String>,
}

//

/// Starts the target stack. `docker compose stop`. Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct StopStack {
  /// Id or name
  pub stack: String,
  /// Override the default termination max time.
  pub stop_time: Option<i32>,
  /// Optionally specify a specific service to stop
  pub service: Option<String>,
}

//

/// Destoys the target stack. `docker compose down`. Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct DestroyStack {
  /// Id or name
  pub stack: String,
  /// Pass `--remove-orphans`
  #[serde(default)]
  pub remove_orphans: bool,
  /// Override the default termination max time.
  pub stop_time: Option<i32>,
  /// Optionally specify a specific service to destroy
  pub service: Option<String>,
}
