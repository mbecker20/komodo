use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use mungos::mongodb::bson::{doc, Document};
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::I64;

use super::{
  resource::{AddFilters, Resource, ResourceListItem, ResourceQuery},
  SystemCommand,
};

#[typeshare]
pub type RepoListItem = ResourceListItem<RepoListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepoListItemInfo {
  pub last_pulled_at: I64,
  pub repo: String,
  pub branch: String,
}

#[typeshare]
pub type Repo = Resource<RepoConfig, RepoInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepoInfo {
  pub last_pulled_at: I64,
}

#[typeshare(serialized_as = "Partial<RepoConfig>")]
pub type _PartialRepoConfig = PartialRepoConfig;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[skip_serializing_none]
#[partial_from]
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
}

impl RepoConfig {
  pub fn builder() -> RepoConfigBuilder {
    RepoConfigBuilder::default()
  }
}

fn default_branch() -> String {
  String::from("main")
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct RepoActionState {
  pub cloning: bool,
  pub pulling: bool,
  pub updating: bool,
  pub deleting: bool,
}

#[typeshare]
pub type RepoQuery = ResourceQuery<RepoQuerySpecifics>;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, DefaultBuilder,
)]
pub struct RepoQuerySpecifics {
  pub repos: Vec<String>,
}

impl AddFilters for RepoQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    if !self.repos.is_empty() {
      filters.insert("config.repo", doc! { "$in": &self.repos });
    }
  }
}
