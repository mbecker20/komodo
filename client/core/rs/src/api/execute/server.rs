use clap::Parser;
use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::update::Update;

use super::MonitorExecuteRequest;

//

/// Stops all containers on the target server. Response: [Update]
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
pub struct StopAllContainers {
  /// Name or id
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

/// Prunes the docker volumes on the target server. Response: [Update].
///
/// 1. Runs `docker volume prune -a -f`.
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
pub struct PruneVolumes {
  /// Id or name
  pub server: String,
}

/// Prunes the docker system on the target server, including volumes. Response: [Update].
///
/// 1. Runs `docker system prune -a -f --volumes`.
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
pub struct PruneSystem {
  /// Id or name
  pub server: String,
}
