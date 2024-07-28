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
  EnvironmentVar, NoData, SystemCommand, Version,
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
  /// The git provider domain
  pub git_provider: String,
  /// The repo used as the source of the build
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
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Partial)]
#[partial_derive(Debug, Clone, Default, Serialize, Deserialize)]
#[partial(skip_serializing_none, from, diff)]
pub struct BuildConfig {
  /// Which builder is used to build the image.
  #[serde(default, alias = "builder")]
  #[partial_attr(serde(alias = "builder"))]
  #[builder(default)]
  pub builder_id: String,

  /// The current version of the build.
  #[serde(default)]
  #[builder(default)]
  pub version: Version,

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

  /// The repo used as the source of the build.
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
  #[serde(default, alias = "github_account")]
  #[builder(default)]
  pub git_account: String,

  /// The optional command run after repo clone and before docker build.
  #[serde(default)]
  #[builder(default)]
  pub pre_build: SystemCommand,

  /// Configuration for the registry to push the built image to.
  #[serde(default)]
  #[builder(default)]
  pub image_registry: ImageRegistry,

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

  /// Whether to skip secret interpolation in the build_args.
  #[serde(default)]
  #[builder(default)]
  pub skip_secret_interp: bool,

  /// Whether to use buildx to build (eg `docker buildx build ...`)
  #[serde(default)]
  #[builder(default)]
  pub use_buildx: bool,

  /// Whether incoming webhooks actually trigger action.
  #[serde(default = "default_webhook_enabled")]
  #[builder(default = "default_webhook_enabled()")]
  #[partial_default(default_webhook_enabled())]
  pub webhook_enabled: bool,

  /// Any extra docker cli arguments to be included in the build command
  #[serde(default)]
  #[builder(default)]
  pub extra_args: Vec<String>,

  /// Docker build arguments.
  ///
  /// These values are visible in the final image by running `docker inspect`.
  #[serde(
    default,
    deserialize_with = "super::env_vars_deserializer"
  )]
  #[partial_attr(serde(
    default,
    deserialize_with = "super::option_env_vars_deserializer"
  ))]
  #[builder(default)]
  pub build_args: Vec<EnvironmentVar>,

  /// Secret arguments.
  ///
  /// These values remain hidden in the final image by using
  /// docker secret mounts. See `<https://docs.docker.com/build/building/secrets>`.
  ///
  /// The values can be used in RUN commands:
  /// ```
  /// RUN --mount=type=secret,id=SECRET_KEY \
  ///   SECRET_KEY=$(cat /run/secrets/SECRET_KEY) ...
  /// ```
  #[serde(
    default,
    deserialize_with = "super::env_vars_deserializer"
  )]
  #[partial_attr(serde(
    default,
    deserialize_with = "super::option_env_vars_deserializer"
  ))]
  #[builder(default)]
  pub secret_args: Vec<EnvironmentVar>,

  /// Docker labels
  #[serde(
    default,
    deserialize_with = "super::env_vars_deserializer"
  )]
  #[partial_attr(serde(
    default,
    deserialize_with = "super::option_env_vars_deserializer"
  ))]
  #[builder(default)]
  pub labels: Vec<EnvironmentVar>,
}

impl BuildConfig {
  pub fn builder() -> BuildConfigBuilder {
    BuildConfigBuilder::default()
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
      git_provider: default_git_provider(),
      git_https: default_git_https(),
      repo: Default::default(),
      branch: default_branch(),
      commit: Default::default(),
      git_account: Default::default(),
      pre_build: Default::default(),
      build_path: default_build_path(),
      dockerfile_path: default_dockerfile_path(),
      build_args: Default::default(),
      secret_args: Default::default(),
      labels: Default::default(),
      extra_args: Default::default(),
      use_buildx: Default::default(),
      image_registry: Default::default(),
      webhook_enabled: default_webhook_enabled(),
    }
  }
}

/// Configuration for the registry to push the built image to.
#[typeshare]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "params")]
pub enum ImageRegistry {
  /// Don't push the image to any registry
  None(NoData),
  /// Push the image to DockerHub
  DockerHub(CloudRegistryConfig),
  /// Push the image to the Github Container Registry.
  ///
  /// See [the Github docs](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry#pushing-container-images)
  /// for information on creating an access token
  Ghcr(CloudRegistryConfig),
  /// Push the image to Aws Elastic Container Registry
  ///
  /// The string held in 'params' should match a label of an `aws_ecr_registry` in the core config.
  AwsEcr(String),
  /// Push the image to a custom image registry (any domain)
  Custom(CustomRegistryConfig),
}

impl Default for ImageRegistry {
  fn default() -> Self {
    Self::None(NoData {})
  }
}

/// Configuration for a cloud image registry, like account and organization.
#[typeshare]
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct CloudRegistryConfig {
  /// Specify an account to use with the cloud registry.
  #[serde(default)]
  pub account: String,

  /// Optional. Specify an organization to push the image under.
  /// Empty string means no organization.
  #[serde(default)]
  pub organization: String,
}

/// Configuration for a custom image registry
#[typeshare]
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct CustomRegistryConfig {
  /// Specify the registry provider domain. Eg. `docker.io`
  #[serde(default)]
  pub provider: String,

  /// Specify an account to use with the registry.
  #[serde(default)]
  pub account: String,

  /// Optional. Specify an organization to push the image under.
  /// Empty string means no organization.
  #[serde(default)]
  pub organization: String,
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
