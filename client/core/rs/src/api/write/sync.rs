use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  sync::{ResourceSync, _PartialResourceSyncConfig},
  NoData,
};

use super::MonitorWriteRequest;

//

/// Create a sync. Response: [ResourceSync].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(ResourceSync)]
pub struct CreateResourceSync {
  /// The name given to newly created sync.
  pub name: String,
  /// Optional partial config to initialize the sync with.
  pub config: _PartialResourceSyncConfig,
}

//

/// Creates a new sync with given `name` and the configuration
/// of the sync at the given `id`. Response: [ResourceSync].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(ResourceSync)]
pub struct CopyResourceSync {
  /// The name of the new sync.
  pub name: String,
  /// The id of the sync to copy.
  pub id: String,
}

//

/// Deletes the sync at the given id, and returns the deleted sync.
/// Response: [ResourceSync]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(ResourceSync)]
pub struct DeleteResourceSync {
  /// The id or name of the sync to delete.
  pub id: String,
}

//

/// Update the sync at the given id, and return the updated sync.
/// Response: [ResourceSync].
///
/// Note. If the attached server for the sync changes,
/// the sync will be deleted / cleaned up on the old server.
///
/// Note. This method updates only the fields which are set in the [_PartialResourceSyncConfig],
/// effectively merging diffs into the final document.
/// This is helpful when multiple users are using
/// the same resources concurrently by ensuring no unintentional
/// field changes occur from out of date local state.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(ResourceSync)]
pub struct UpdateResourceSync {
  /// The id of the sync to update.
  pub id: String,
  /// The partial config update to apply.
  pub config: _PartialResourceSyncConfig,
}

//

/// Trigger a refresh of the computed diff logs for view.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(ResourceSync)]
pub struct RefreshResourceSyncPending {
  /// Id or name
  pub sync: String,
}

//

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncWebhookAction {
  Refresh,
  Sync,
}

/// Create a webhook on the github repo attached to the sync
/// passed in request. Response: [CreateSyncWebhookResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateSyncWebhookResponse)]
pub struct CreateSyncWebhook {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub sync: String,
  /// "Refresh" or "Sync"
  pub action: SyncWebhookAction,
}

#[typeshare]
pub type CreateSyncWebhookResponse = NoData;

//

/// Delete the webhook on the github repo attached to the sync
/// passed in request. Response: [DeleteSyncWebhookResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(DeleteSyncWebhookResponse)]
pub struct DeleteSyncWebhook {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub sync: String,
  /// "Refresh" or "Sync"
  pub action: SyncWebhookAction,
}

#[typeshare]
pub type DeleteSyncWebhookResponse = NoData;
