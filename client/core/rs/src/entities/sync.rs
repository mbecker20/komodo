use bson::{doc, Document};
use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum::Display;
use typeshare::typeshare;

use super::{
  resource::{Resource, ResourceListItem, ResourceQuery},
  I64,
};

#[typeshare]
pub type ResourceSyncListItem =
  ResourceListItem<ResourceSyncListItemInfo>;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSyncListItemInfo {
  /// Unix timestamp of last sync, or 0
  pub last_sync_ts: I64,
  /// Short commit hash of last sync, or empty string
  pub last_sync_hash: String,
  /// Commit message of last sync, or empty string
  pub last_sync_message: String,
  /// The Github repo used as the source of the sync resources
  pub repo: String,
  /// The branch of the repo
  pub branch: String,
  /// State of the sync. Reflects whether most recent sync successful.
  pub state: SyncState,
}

#[typeshare]
#[derive(
  Debug, Clone, Copy, Default, Serialize, Deserialize, Display,
)]
pub enum SyncState {
  /// Last sync successful (or never synced)
  Ok,
  /// Last sync failed
  Failed,
  /// Currently syncing
  Syncing,
  /// Other case
  #[default]
  Unknown,
}

#[typeshare]
pub type ResourceSync =
  Resource<ResourceSyncConfig, ResourceSyncInfo>;

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourceSyncInfo {
  /// Unix timestamp of last sync
  pub last_sync_ts: I64,
  /// Short commit hash of last sync
  pub last_sync_hash: String,
  /// Commit message of last sync
  pub last_sync_message: String,
}

#[typeshare(serialized_as = "Partial<ResourceSyncConfig>")]
pub type _PartialResourceSyncConfig = PartialResourceSyncConfig;

/// The sync configuration.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Partial)]
#[partial_derive(Debug, Clone, Default, Serialize, Deserialize)]
#[partial(skip_serializing_none, from, diff)]
pub struct ResourceSyncConfig {
  /// The Github repo used as the source of the build.
  #[serde(default)]
  #[builder(default)]
  pub repo: String,

  /// The branch of the repo.
  #[serde(default = "default_branch")]
  #[builder(default = "default_branch()")]
  #[partial_default(default_branch())]
  pub branch: String,

  /// Optionally set a specific commit hash.
  #[serde(default)]
  #[builder(default)]
  pub commit: String,

  /// The github account used to clone (used to access private repos).
  /// Empty string is public clone (only public repos).
  #[serde(default)]
  #[builder(default)]
  pub github_account: String,

  /// The github account used to clone (used to access private repos).
  /// Empty string is public clone (only public repos).
  #[serde(default = "default_resource_path")]
  #[builder(default = "default_resource_path()")]
  #[partial_default(default_resource_path())]
  pub resource_path: String,

  /// Whether sync should delete resources
  /// not declared in the resource files
  #[serde(default)]
  #[builder(default)]
  pub delete: bool,

  /// Whether incoming webhooks actually trigger action.
  #[serde(default = "default_webhook_enabled")]
  #[builder(default = "default_webhook_enabled()")]
  #[partial_default(default_webhook_enabled())]
  pub webhook_enabled: bool,
}

impl ResourceSyncConfig {
  pub fn builder() -> ResourceSyncConfigBuilder {
    ResourceSyncConfigBuilder::default()
  }
}

fn default_branch() -> String {
  String::from("main")
}

fn default_resource_path() -> String {
  String::from("resources")
}

fn default_webhook_enabled() -> bool {
  true
}

impl Default for ResourceSyncConfig {
  fn default() -> Self {
    Self {
      repo: Default::default(),
      branch: default_branch(),
      commit: Default::default(),
      github_account: Default::default(),
      resource_path: default_resource_path(),
      delete: Default::default(),
      webhook_enabled: default_webhook_enabled(),
    }
  }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct ResourceSyncActionState {
  /// Whether sync currently syncing
  pub syncing: bool,
}

#[typeshare]
pub type ResourceSyncQuery =
  ResourceQuery<ResourceSyncQuerySpecifics>;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, DefaultBuilder,
)]
pub struct ResourceSyncQuerySpecifics {
  /// Filter syncs by their repo.
  pub repos: Vec<String>,
}

impl super::resource::AddFilters for ResourceSyncQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    if !self.repos.is_empty() {
      filters.insert("config.repo", doc! { "$in": &self.repos });
    }
  }
}
