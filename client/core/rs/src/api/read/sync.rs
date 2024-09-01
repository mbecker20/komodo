use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::sync::{
  ResourceSync, ResourceSyncActionState, ResourceSyncListItem,
  ResourceSyncQuery,
};

use super::KomodoReadRequest;

//

/// Get a specific sync. Response: [ResourceSync].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(ResourceSync)]
pub struct GetResourceSync {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub sync: String,
}

#[typeshare]
pub type GetResourceSyncResponse = ResourceSync;

//

/// List syncs matching optional query. Response: [ListResourceSyncsResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(ListResourceSyncsResponse)]
pub struct ListResourceSyncs {
  /// optional structured query to filter syncs.
  #[serde(default)]
  pub query: ResourceSyncQuery,
}

#[typeshare]
pub type ListResourceSyncsResponse = Vec<ResourceSyncListItem>;

//

/// List syncs matching optional query. Response: [ListFullResourceSyncsResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(ListFullResourceSyncsResponse)]
pub struct ListFullResourceSyncs {
  /// optional structured query to filter syncs.
  #[serde(default)]
  pub query: ResourceSyncQuery,
}

#[typeshare]
pub type ListFullResourceSyncsResponse = Vec<ResourceSync>;

//

/// Get current action state for the sync. Response: [ResourceSyncActionState].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(GetResourceSyncActionStateResponse)]
pub struct GetResourceSyncActionState {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub sync: String,
}

#[typeshare]
pub type GetResourceSyncActionStateResponse = ResourceSyncActionState;

//

/// Gets a summary of data relating to all syncs.
/// Response: [GetResourceSyncsSummaryResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(GetResourceSyncsSummaryResponse)]
pub struct GetResourceSyncsSummary {}

/// Response for [GetResourceSyncsSummary]
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetResourceSyncsSummaryResponse {
  /// The total number of syncs
  pub total: u32,
  /// The number of syncs with Ok state.
  pub ok: u32,
  /// The number of syncs currently syncing.
  pub syncing: u32,
  /// The number of syncs with pending updates
  pub pending: u32,
  /// The number of syncs with failed state.
  pub failed: u32,
  /// The number of syncs with unknown state.
  pub unknown: u32,
}

//

/// Get a target Sync's configured webhooks. Response: [GetSyncWebhooksEnabledResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(GetSyncWebhooksEnabledResponse)]
pub struct GetSyncWebhooksEnabled {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub sync: String,
}

/// Response for [GetSyncWebhooksEnabled]
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSyncWebhooksEnabledResponse {
  /// Whether the repo webhooks can even be managed.
  /// The repo owner must be in `github_webhook_app.owners` list to be managed.
  pub managed: bool,
  /// Whether pushes to branch trigger refresh. Will always be false if managed is false.
  pub refresh_enabled: bool,
  /// Whether pushes to branch trigger sync execution. Will always be false if managed is false.
  pub sync_enabled: bool,
}
