use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::update::Update;

use super::MonitorWriteRequest;

/// Launch an EC2 instance with the specified config.
/// Response: [Update].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Update)]
pub struct LaunchServer {
  /// The name of the created server.
  pub name: String,
  /// The server template used to define the config.
  pub server_template: String,
}
