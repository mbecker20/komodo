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
  /// The builder attached to build.
  pub builder_id: String,
  /// The git provider domain
  pub git_provider: String,
  /// The repo used as the source of the build
  pub repo: String,
  /// The branch of the repo
  pub branch: String,
  /// State of the build. Reflects whether most recent build successful.
  pub state: BuildState,
  /// Latest built short commit hash, or null.
  pub built_hash: Option<String>,
  /// Latest short commit hash, or null. Only for repo based stacks
  pub latest_hash: Option<String>,
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
  /// Latest built short commit hash, or null.
  pub built_hash: Option<String>,
  /// Latest built commit message, or null. Only for repo based stacks
  pub built_message: Option<String>,
  /// Latest remote short commit hash, or null.
  pub latest_hash: Option<String>,
  /// Latest remote commit message, or null
  pub latest_message: Option<String>,
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

  /// Whether to automatically increment the patch on every build.
  /// Default is `true`
  #[serde(default = "default_auto_increment_version")]
  #[builder(default = "default_auto_increment_version()")]
  #[partial_default(default_auto_increment_version())]
  pub auto_increment_version: bool,

  /// An alternate name for the image pushed to the repository.
  /// If this is empty, it will use the build name.
  ///
  /// Can be used in conjunction with `image_tag` to direct multiple builds
  /// with different configs to push to the same image registry, under different,
  /// independantly versioned tags.
  #[serde(default)]
  #[builder(default)]
  pub image_name: String,

  /// An extra tag put before the build version, for the image pushed to the repository.
  /// Eg. in image tag of `aarch64` would push to mbecker20/komodo:1.13.2-aarch64.
  /// If this is empty, the image tag will just be the build version.
  ///
  /// Can be used in conjunction with `image_name` to direct multiple builds
  /// with different configs to push to the same image registry, under different,
  /// independantly versioned tags.
  #[serde(default)]
  #[builder(default)]
  pub image_tag: String,

  /// Configure quick links that are displayed in the resource header
  #[serde(default)]
  #[builder(default)]
  pub links: Vec<String>,

  /// The git provider domain. Default: github.com
  #[serde(default = "default_git_provider")]
  #[builder(default = "default_git_provider()")]
  #[partial_default(default_git_provider())]
  pub git_provider: String,

  /// Whether to use https to clone the repo (versus http). Default: true
  ///
  /// Note. Komodo does not currently support cloning repos via ssh.
  #[serde(default = "default_git_https")]
  #[builder(default = "default_git_https()")]
  #[partial_default(default_git_https())]
  pub git_https: bool,

  /// The git account used to access private repos.
  /// Passing empty string can only clone public repos.
  ///
  /// Note. A token for the account must be available in the core config or the builder server's periphery config
  /// for the configured git provider.
  #[serde(default)]
  #[builder(default)]
  pub git_account: String,

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

  /// Whether incoming webhooks actually trigger action.
  #[serde(default = "default_webhook_enabled")]
  #[builder(default = "default_webhook_enabled()")]
  #[partial_default(default_webhook_enabled())]
  pub webhook_enabled: bool,

  /// Optionally provide an alternate webhook secret for this build.
  /// If its an empty string, use the default secret from the config.
  #[serde(default)]
  #[builder(default)]
  pub webhook_secret: String,

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

fn default_auto_increment_version() -> bool {
  true
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
      auto_increment_version: default_auto_increment_version(),
      image_name: Default::default(),
      image_tag: Default::default(),
      links: Default::default(),
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
      webhook_secret: Default::default(),
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
  /// Push the image to a standard image registry (any domain)
  Standard(StandardRegistryConfig),
  /// Push the image to Aws Elastic Container Registry
  ///
  /// The string held in 'params' should match a label of an `aws_ecr_registry` in the core config.
  AwsEcr(String),
}

impl Default for ImageRegistry {
  fn default() -> Self {
    Self::None(NoData {})
  }
}

/// Configuration for a standard image registry
#[typeshare]
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct StandardRegistryConfig {
  /// Specify the registry provider domain. Default: `docker.io`
  #[serde(default = "default_registry_domain")]
  pub domain: String,

  /// Specify an account to use with the registry.
  #[serde(default)]
  pub account: String,

  /// Optional. Specify an organization to push the image under.
  /// Empty string means no organization.
  #[serde(default)]
  pub organization: String,
}

fn default_registry_domain() -> String {
  String::from("docker.io")
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
