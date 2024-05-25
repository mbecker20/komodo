use bson::{doc, Document};
use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum::Display;
use typeshare::typeshare;

use crate::entities::I64;

use super::{
  resource::{Resource, ResourceListItem, ResourceQuery},
  SystemCommand,
};

#[typeshare]
pub type RepoListItem = ResourceListItem<RepoListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepoListItemInfo {
  /// The server that repo sits on.
  pub server_id: String,
  /// Repo last cloned / pulled timestamp in ms.
  pub last_pulled_at: I64,
  /// The configured github repo
  pub repo: String,
  /// The configured branch
  pub branch: String,
  /// The repo state
  pub state: RepoState,
  /// If the repo is cloned, will be the latest short commit hash.
  pub latest_hash: Option<String>,
  /// If the repo is cloned, will be the latest commit message.
  pub latest_message: Option<String>,
}

#[typeshare]
#[derive(
  Debug, Clone, Copy, Default, Serialize, Deserialize, Display,
)]
pub enum RepoState {
  /// Unknown case
  #[default]
  Unknown,
  /// Last clone / pull successful (or never cloned)
  Ok,
  /// Last clone / pull failed
  Failed,
  /// Currently cloning
  Cloning,
  /// Currently pullling
  Pulling,
}

#[typeshare]
pub type Repo = Resource<RepoConfig, RepoInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepoInfo {
  /// When repo was last pulled
  pub last_pulled_at: I64,
}

#[typeshare(serialized_as = "Partial<RepoConfig>")]
pub type _PartialRepoConfig = PartialRepoConfig;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[partial(skip_serializing_none, from, diff)]
pub struct RepoConfig {
  /// The server to clone the repo on.
  #[serde(default, alias = "server")]
  #[partial_attr(serde(alias = "server"))]
  #[builder(default)]
  pub server_id: String,

  /// The github repo to clone.
  #[serde(default)]
  #[builder(default)]
  pub repo: String,

  /// The repo branch.
  #[serde(default = "default_branch")]
  #[builder(default = "default_branch()")]
  #[partial_default(default_branch())]
  pub branch: String,

  /// Optionally set a specific commit hash.
  #[serde(default)]
  #[builder(default)]
  pub commit: String,

  /// The github account to use to clone.
  /// It must be available in the server's periphery config.
  #[serde(default)]
  #[builder(default)]
  pub github_account: String,

  /// Command to be run after the repo is cloned.
  /// The path is relative to the root of the repo.
  #[serde(default)]
  #[builder(default)]
  pub on_clone: SystemCommand,

  /// Command to be run after the repo is pulled.
  /// The path is relative to the root of the repo.
  #[serde(default)]
  #[builder(default)]
  pub on_pull: SystemCommand,

  /// Whether incoming webhooks actually trigger action.
  #[serde(default = "default_webhook_enabled")]
  #[builder(default = "default_webhook_enabled()")]
  #[partial_default(default_webhook_enabled())]
  pub webhook_enabled: bool,
}

impl RepoConfig {
  pub fn builder() -> RepoConfigBuilder {
    RepoConfigBuilder::default()
  }
}

fn default_branch() -> String {
  String::from("main")
}

fn default_webhook_enabled() -> bool {
  true
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct RepoActionState {
  /// Whether repo currently cloning
  pub cloning: bool,
  /// Whether repo currently pulling
  pub pulling: bool,
}

#[typeshare]
pub type RepoQuery = ResourceQuery<RepoQuerySpecifics>;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, DefaultBuilder,
)]
pub struct RepoQuerySpecifics {
  /// Filter builds by their repo.
  pub repos: Vec<String>,
}

impl super::resource::AddFilters for RepoQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    if !self.repos.is_empty() {
      filters.insert("config.repo", doc! { "$in": &self.repos });
    }
  }
}
