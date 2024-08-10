use clap::Parser;
use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::update::Update;

use super::MonitorExecuteRequest;

//

/// Runs the target build. Response: [Update].
///
/// 1. Get a handle to the builder. If using AWS builder, this means starting a builder ec2 instance.
/// 2. Clone the repo on the builder. If an `on_clone` commmand is given, it will be executed.
/// 3. Execute `docker build {...params}`, where params are determined using the builds configuration.
/// 4. If a dockerhub account is attached, the build will be pushed to that account.
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
pub struct RunBuild {
  /// Can be build id or name
  pub build: String,
}

//

/// Cancels the target build.
/// Only does anything if the build is `building` when called.
/// Response: [Update]
#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  PartialEq,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct CancelBuild {
  /// Can be id or name
  pub build: String,
}
