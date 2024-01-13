use derive_builder::Builder;
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
pub type Repo = Resource<RepoConfig, RepoInfo>;

#[typeshare]
pub type RepoListItem = ResourceListItem<RepoInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepoInfo {
  pub last_pulled_at: I64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[skip_serializing_none]
#[partial_from]
pub struct RepoConfig {
  pub server_id: String,

  pub repo: String,

  #[serde(default = "default_branch")]
  #[builder(default = "default_branch()")]
  #[partial_default(default_branch())]
  pub branch: String,

  #[serde(default)]
  #[builder(default)]
  pub github_account: String,

  #[serde(default)]
  #[builder(default)]
  pub on_clone: SystemCommand,

  #[serde(default)]
  #[builder(default)]
  pub on_pull: SystemCommand,
}

fn default_branch() -> String {
  String::from("main")
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepoActionState {
  pub cloning: bool,
  pub pulling: bool,
  pub updating: bool,
  pub deleting: bool,
}

#[typeshare]
pub type RepoQuery = ResourceQuery<RepoQuerySpecifics>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
