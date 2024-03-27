use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use mungos::mongodb::bson::{doc, Document};
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::I64;

use super::{
  resource::{AddFilters, Resource, ResourceListItem, ResourceQuery},
  EnvironmentVar, SystemCommand, Version,
};

#[typeshare]
pub type Build = Resource<BuildConfig, BuildInfo>;

#[typeshare]
pub type BuildListItem = ResourceListItem<BuildListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildListItemInfo {
  pub last_built_at: I64,
  pub version: Version,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildInfo {
  pub last_built_at: I64,
}

#[typeshare(serialized_as = "Partial<BuildConfig>")]
pub type _PartialBuildConfig = PartialBuildConfig;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[skip_serializing_none]
#[partial_from]
pub struct BuildConfig {
  #[serde(default, alias = "builder")]
  #[partial_attr(serde(alias = "builder"))]
  #[builder(default)]
  pub builder_id: String,

  #[serde(default)]
  #[builder(default)]
  pub skip_secret_interp: bool,

  #[serde(default)]
  #[builder(default)]
  pub version: Version,

  #[serde(default)]
  #[builder(default)]
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
  pub docker_account: String,

  #[serde(default)]
  #[builder(default)]
  pub docker_organization: String,

  #[serde(default)]
  #[builder(default)]
  pub pre_build: SystemCommand,

  #[serde(default = "default_build_path")]
  #[builder(default = "default_build_path()")]
  #[partial_default(default_build_path())]
  pub build_path: String,

  #[serde(default = "default_dockerfile_path")]
  #[builder(default = "default_dockerfile_path()")]
  #[partial_default(default_dockerfile_path())]
  pub dockerfile_path: String,

  #[serde(default)]
  #[builder(default)]
  pub build_args: Vec<EnvironmentVar>,

  #[serde(default)]
  #[builder(default)]
  pub extra_args: Vec<String>,

  #[serde(default)]
  #[builder(default)]
  pub use_buildx: bool,
}

fn default_branch() -> String {
  String::from("main")
}

fn default_build_path() -> String {
  String::from(".")
}

fn default_dockerfile_path() -> String {
  String::from("Dockerfile")
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildActionState {
  pub building: bool,
  pub updating: bool,
}

#[typeshare]
pub type BuildQuery = ResourceQuery<BuildQuerySpecifics>;

#[typeshare]
#[derive(
  Debug, Clone, Default, Serialize, Deserialize, DefaultBuilder,
)]
pub struct BuildQuerySpecifics {
  #[serde(default)]
  pub builder_ids: Vec<String>,

  #[serde(default)]
  pub repos: Vec<String>,

  /// query for builds last built more recently than this timestamp
  /// defaults to 0 which is a no op
  #[serde(default)]
  pub built_since: I64,
}

impl AddFilters for BuildQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    if !self.builder_ids.is_empty() {
      filters.insert(
        "config.builder_id",
        doc! { "$in": &self.builder_ids },
      );
    }
    if !self.repos.is_empty() {
      filters.insert("config.repo", doc! { "$in": &self.repos });
    }
    if self.built_since > 0 {
      filters.insert(
        "info.last_built_at",
        doc! { "$gte": self.built_since },
      );
    }
  }
}
