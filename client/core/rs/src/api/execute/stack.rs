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
  /// Override the default termination max time.
  pub stop_time: Option<i32>,
}

//

/// Deploys the target stack service. `docker compose up -d service`. Response: [Update]
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
pub struct DeployStackService {
  /// Id or name
  pub stack: String,
  /// The service name
  pub service: String,
  /// Override the default termination max time.
  /// Only used if the stack needs to be taken down first.
  pub stop_time: Option<i32>,
}

//

/// Starts the target stack service. `docker compose start service`. Response: [Update]
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
pub struct StartStackService {
  /// Id or name
  pub stack: String,
  /// The service name
  pub service: String,
}

//

/// Restarts the target stack service. `docker compose restart service`. Response: [Update]
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
pub struct RestartStackService {
  /// Id or name
  pub stack: String,
  /// The service name
  pub service: String,
}

//

/// Pauses the target stack service. `docker compose pause service`. Response: [Update]
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
pub struct PauseStackService {
  /// Id or name
  pub stack: String,
  /// The service name
  pub service: String,
}

//

/// Unpauses the target stack service. `docker compose unpause service`. Response: [Update].
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
pub struct UnpauseStackService {
  /// Id or name
  pub stack: String,
  /// The service name
  pub service: String,
}

//

/// Starts the target stack service. `docker compose stop service`. Response: [Update]
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
pub struct StopStackService {
  /// Id or name
  pub stack: String,
  /// The service name
  pub service: String,
  /// Override the default termination max time.
  pub stop_time: Option<i32>,
}

//

/// Destoys the target stack service. `docker compose down service`. Response: [Update]
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
pub struct DestroyStackService {
  /// Id or name
  pub stack: String,
  /// The service name
  pub service: String,
  /// Override the default termination max time.
  pub stop_time: Option<i32>,
}

//
