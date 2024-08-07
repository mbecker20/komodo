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
  /// The git provider domain
  pub git_provider: String,
  /// The Github repo used as the source of the sync resources
  pub repo: String,
  /// The branch of the repo
  pub branch: String,
  /// State of the sync. Reflects whether most recent sync successful.
  pub state: ResourceSyncState,
}

#[typeshare]
#[derive(
  Debug, Clone, Copy, Default, Serialize, Deserialize, Display,
)]
pub enum ResourceSyncState {
  /// Last sync successful (or never synced). No Changes pending
  Ok,
  /// Last sync failed
  Failed,
  /// Currently syncing
  Syncing,
  /// Updates pending
  Pending,
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
  /// Unix timestamp of last applied sync
  pub last_sync_ts: I64,
  /// Short commit hash of last applied sync
  pub last_sync_hash: String,
  /// Commit message of last applied sync
  pub last_sync_message: String,
  /// Readable logs of pending updates
  pub pending: PendingSyncUpdates,
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PendingSyncUpdates {
  /// The commit hash which produced these pending updates
  pub hash: Option<String>,
  /// The commit message which produced these pending updates
  pub message: Option<String>,
  /// The data associated with the sync. Either Ok containing diffs,
  /// or Err containing an error message
  pub data: PendingSyncUpdatesData,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
#[allow(clippy::large_enum_variant)]
pub enum PendingSyncUpdatesData {
  Ok(PendingSyncUpdatesDataOk),
  Err(PendingSyncUpdatesDataErr),
}

impl Default for PendingSyncUpdatesData {
  fn default() -> Self {
    Self::Ok(Default::default())
  }
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PendingSyncUpdatesDataOk {
  /// Readable log of any pending server updates
  pub server_updates: Option<SyncUpdate>,
  /// Readable log of any pending deployment updates
  pub deployment_updates: Option<SyncUpdate>,
  /// Readable log of any pending deployment updates
  pub stack_updates: Option<SyncUpdate>,
  /// Readable log of any pending build updates
  pub build_updates: Option<SyncUpdate>,
  /// Readable log of any pending repo updates
  pub repo_updates: Option<SyncUpdate>,
  /// Readable log of any pending procedure updates
  pub procedure_updates: Option<SyncUpdate>,
  /// Readable log of any pending alerter updates
  pub alerter_updates: Option<SyncUpdate>,
  /// Readable log of any pending builder updates
  pub builder_updates: Option<SyncUpdate>,
  /// Readable log of any pending server template updates
  pub server_template_updates: Option<SyncUpdate>,
  /// Readable log of any pending resource sync updates
  pub resource_sync_updates: Option<SyncUpdate>,
  /// Readable log of any pending variable updates
  pub variable_updates: Option<SyncUpdate>,
  /// Readable log of any pending user group updates
  pub user_group_updates: Option<SyncUpdate>,
  /// Readable log of any deploy actions that will be performed
  pub deploy_updates: Option<SyncDeployUpdate>,
}

impl PendingSyncUpdatesDataOk {
  pub fn no_updates(&self) -> bool {
    self.server_updates.is_none()
      && self.deployment_updates.is_none()
      && self.build_updates.is_none()
      && self.repo_updates.is_none()
      && self.procedure_updates.is_none()
      && self.alerter_updates.is_none()
      && self.builder_updates.is_none()
      && self.server_template_updates.is_none()
      && self.resource_sync_updates.is_none()
      && self.variable_updates.is_none()
      && self.user_group_updates.is_none()
  }
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SyncUpdate {
  /// Resources to create
  pub to_create: i32,
  /// Resources to update
  pub to_update: i32,
  /// Resources to delete
  pub to_delete: i32,
  /// A readable log of all the changes to be applied
  pub log: String,
}

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SyncDeployUpdate {
  /// Resources to deploy
  pub to_deploy: i32,
  /// A readable log of all the changes to be applied
  pub log: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingSyncUpdatesDataErr {
  pub message: String,
}

#[typeshare(serialized_as = "Partial<ResourceSyncConfig>")]
pub type _PartialResourceSyncConfig = PartialResourceSyncConfig;

/// The sync configuration.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Partial)]
#[partial_derive(Debug, Clone, Default, Serialize, Deserialize)]
#[partial(skip_serializing_none, from, diff)]
pub struct ResourceSyncConfig {
  /// The git provider domain. Default: github.com
  #[serde(default = "default_git_provider")]
  #[builder(default = "default_git_provider()")]
  #[partial_default(default_git_provider())]
  pub git_provider: String,

  /// Whether to use https to clone the repo (versus http). Default: true
  ///
  /// Note. Monitor does not currently support cloning repos via ssh.
  #[serde(default = "default_git_https")]
  #[builder(default = "default_git_https()")]
  #[partial_default(default_git_https())]
  pub git_https: bool,

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

  /// The git account used to access private repos.
  /// Passing empty string can only clone public repos.
  ///
  /// Note. A token for the account must be available in the core config or the builder server's periphery config
  /// for the configured git provider.
  #[serde(default)]
  #[builder(default)]
  pub git_account: String,

  /// The path of the resource file(s) to sync, relative to the repo root.
  /// Can be a specific file, or a directory containing multiple files / folders.
  /// See `https://docs.monitor.mogh.tech/docs/sync-resources` for more information.
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

fn default_git_provider() -> String {
  String::from("github.com")
}

fn default_git_https() -> bool {
  true
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
      git_provider: default_git_provider(),
      git_https: default_git_https(),
      repo: Default::default(),
      branch: default_branch(),
      commit: Default::default(),
      git_account: Default::default(),
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
