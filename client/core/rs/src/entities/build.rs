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
  EnvironmentVar, SystemCommand, Version,
};

#[typeshare]
pub type Build = Resource<BuildConfig, BuildInfo>;

#[typeshare]
pub type BuildListItem = ResourceListItem<BuildListItemInfo>;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildListItemInfo {
  /// Unix timestamp in milliseconds of last build
  pub last_built_at: I64,
  /// The current version of the build
  pub version: Version,
  /// The Github repo used as the source of the build
  pub repo: String,
  /// The branch of the repo
  pub branch: String,
  /// State of the build. Reflects whether most recent build successful.
  pub state: BuildState,
}

#[typeshare]
#[derive(
  Debug, Clone, Copy, Default, Serialize, Deserialize, Display,
)]
pub enum BuildState {
  /// Last build successful (or never built)
  Ok,
  /// Last build failed
  Failed,
  /// Currently building
  Building,
  /// Other case
  #[default]
  Unknown,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildInfo {
  pub last_built_at: I64,
}

#[typeshare(serialized_as = "Partial<BuildConfig>")]
pub type _PartialBuildConfig = PartialBuildConfig;

/// The build configuration.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[partial(skip_serializing_none, from, diff)]
pub struct BuildConfig {
  /// Which builder is used to build the image.
  #[serde(default, alias = "builder")]
  #[partial_attr(serde(alias = "builder"))]
  #[builder(default)]
  pub builder_id: String,

  /// Whether to skip secret interpolation in the build_args.
  #[serde(default)]
  #[builder(default)]
  pub skip_secret_interp: bool,

  /// The current version of the build.
  #[serde(default)]
  #[builder(default)]
  pub version: Version,

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

  /// The dockerhub account used to push the image to dockerhub.
  /// Empty string means no dockerhub push (server local build).
  #[serde(default)]
  #[builder(default)]
  pub docker_account: String,

  /// The docker organization which the image should be pushed under.
  /// Empty string means no organization.
  #[serde(default)]
  #[builder(default)]
  pub docker_organization: String,

  /// The optional command run after repo clone and before docker build.
  #[serde(default)]
  #[builder(default)]
  pub pre_build: SystemCommand,

  /// The path of the docker build context relative to the root of the repo.
  /// Default: "." (the root of the repo).
  #[serde(default = "default_build_path")]
  #[builder(default = "default_build_path()")]
  #[partial_default(default_build_path())]
  pub build_path: String,

  /// The path of the dockerfile relative to the build path.
  #[serde(default = "default_dockerfile_path")]
  #[builder(default = "default_dockerfile_path()")]
  #[partial_default(default_dockerfile_path())]
  pub dockerfile_path: String,

  /// Docker build arguments
  #[serde(default)]
  #[builder(default)]
  pub build_args: Vec<EnvironmentVar>,

  /// Docker labels
  #[serde(default)]
  #[builder(default)]
  pub labels: Vec<EnvironmentVar>,

  /// Any extra docker cli arguments to be included in the build command
  #[serde(default)]
  #[builder(default)]
  pub extra_args: Vec<String>,

  /// Whether to use buildx to build (eg `docker buildx build ...`)
  #[serde(default)]
  #[builder(default)]
  pub use_buildx: bool,

  /// Whether incoming webhooks actually trigger action.
  #[serde(default = "default_webhook_enabled")]
  #[builder(default = "default_webhook_enabled()")]
  #[partial_default(default_webhook_enabled())]
  pub webhook_enabled: bool,
}

impl BuildConfig {
  pub fn builder() -> BuildConfigBuilder {
    BuildConfigBuilder::default()
  }
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

fn default_webhook_enabled() -> bool {
  true
}

impl Default for BuildConfig {
  fn default() -> Self {
    Self {
      builder_id: Default::default(),
      skip_secret_interp: Default::default(),
      version: Default::default(),
      repo: Default::default(),
      branch: default_branch(),
      commit: Default::default(),
      github_account: Default::default(),
      docker_account: Default::default(),
      docker_organization: Default::default(),
      pre_build: Default::default(),
      build_path: default_build_path(),
      dockerfile_path: default_dockerfile_path(),
      build_args: Default::default(),
      labels: Default::default(),
      extra_args: Default::default(),
      use_buildx: Default::default(),
      webhook_enabled: default_webhook_enabled(),
    }
  }
}

#[typeshare]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct BuildActionState {
  pub building: bool,
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

impl super::resource::AddFilters for BuildQuerySpecifics {
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
