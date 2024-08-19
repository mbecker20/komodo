use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  _server::{Server, _PartialServerConfig},
  update::Update,
};

use super::MonitorWriteRequest;

//

/// Create a server. Response: [Server].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Server)]
pub struct CreateServer {
  /// The name given to newly created server.
  pub name: String,
  /// Optional partial config to initialize the server with.
  pub config: _PartialServerConfig,
}

//

/// Deletes the server at the given id, and returns the deleted server.
/// Response: [Server]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Server)]
pub struct DeleteServer {
  /// The id or name of the server to delete.
  pub id: String,
}

//

/// Update the server at the given id, and return the updated server.
/// Response: [Server].
///
/// Note. This method updates only the fields which are set in the [_PartialServerConfig],
/// effectively merging diffs into the final document.
/// This is helpful when multiple users are using
/// the same resources concurrently by ensuring no unintentional
/// field changes occur from out of date local state.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Server)]
pub struct UpdateServer {
  /// The id of the server to update.
  pub id: String,
  /// The partial config update to apply.
  pub config: _PartialServerConfig,
}

//

/// Rename the server at id to the given name. Response: [Update].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Update)]
pub struct RenameServer {
  /// The id of the server to rename.
  pub id: String,
  /// The new name.
  pub name: String,
}

//

/// Create a docker network on the server.
/// Respone: [Update]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Update)]
pub struct CreateNetwork {
  /// Id or name
  pub server: String,
  /// The name of the network to create.
  pub name: String,
}

//

/// Delete a docker network.
/// Response: [Update]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Update)]
pub struct DeleteNetwork {
  /// Id or name.
  pub server: String,
  /// The name of the network to delete.
  pub name: String,
}
