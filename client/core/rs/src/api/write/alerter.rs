use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::alerter::{Alerter, PartialAlerterConfig};

use super::MonitorWriteRequest;

//

/// Create an alerter. Response: [Alerter].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Alerter)]
pub struct CreateAlerter {
  /// The name given to newly created alerter.
  pub name: String,
  /// Optional partial config to initialize the alerter with.
  pub config: PartialAlerterConfig,
}

//

/// Creates a new alerter with given `name` and the configuration
/// of the alerter at the given `id`. Response: [Alerter].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Alerter)]
pub struct CopyAlerter {
  /// The name of the new alerter.
  pub name: String,
  /// The id of the alerter to copy.
  pub id: String,
}

//

/// Deletes the alerter at the given id, and returns the deleted alerter.
/// Response: [Alerter]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Alerter)]
pub struct DeleteAlerter {
  /// The id of the alerter to delete.
  pub id: String,
}

//

/// Update the alerter at the given id, and return the updated alerter. Response: [Alerter].
///
/// Note. This method updates only the fields which are set in the [PartialAlerterConfig],
/// effectively merging diffs into the final document. This is helpful when multiple users are using
/// the same resources concurrently by ensuring no unintentional
/// field changes occur from out of date local state.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Alerter)]
pub struct UpdateAlerter {
  /// The id of the alerter to update.
  pub id: String,
  /// The partial config update to apply.
  pub config: PartialAlerterConfig,
}
