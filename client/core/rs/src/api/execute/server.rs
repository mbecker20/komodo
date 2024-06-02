use clap::Parser;
use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::update::Update;

use super::MonitorExecuteRequest;

//

/// Prunes the docker networks on the target server. Response: [Update].
///
/// 1. Runs `docker network prune -f`.
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
pub struct PruneNetworks {
  /// Id or name
  pub server: String,
}

//

/// Prunes the docker images on the target server. Response: [Update].
///
/// 1. Runs `docker image prune -a -f`.
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
pub struct PruneImages {
  /// Id or name
  pub server: String,
}

//

/// Prunes the docker containers on the target server. Response: [Update].
///
/// 1. Runs `docker container prune -f`.
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
pub struct PruneContainers {
  /// Id or name
  pub server: String,
}
