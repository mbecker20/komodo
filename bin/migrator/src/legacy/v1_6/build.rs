use monitor_client::entities::{
  build::{ImageRegistry, StandardRegistryConfig},
  NoData,
};
use serde::{Deserialize, Serialize};

use super::{
  resource::Resource, EnvironmentVar, SystemCommand, Version,
};

pub type Build = Resource<BuildConfig, BuildInfo>;

impl From<Build> for monitor_client::entities::build::Build {
  fn from(value: Build) -> Self {
    monitor_client::entities::build::Build {
      id: value.id,
      name: value.name,
      description: value.description,
      updated_at: value.updated_at,
      tags: value.tags,
      info: monitor_client::entities::build::BuildInfo {
        last_built_at: value.info.last_built_at,
        built_hash: None,
        built_message: None,
        latest_hash: None,
        latest_message: None,
      },
      base_permission: Default::default(),
      config: value.config.into(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildInfo {
  pub last_built_at: i64,
}

/// The build configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
  /// Which builder is used to build the image.
  #[serde(default, alias = "builder")]
  pub builder_id: String,

  /// Whether to skip secret interpolation in the build_args.
  #[serde(default)]
  pub skip_secret_interp: bool,

  /// The current version of the build.
  #[serde(default)]
  pub version: Version,

  /// The Github repo used as the source of the build.
  #[serde(default)]
  pub repo: String,

  /// The branch of the repo.
  #[serde(default = "default_branch")]
  pub branch: String,

  /// Optionally set a specific commit hash.
  #[serde(default)]
  pub commit: String,

  /// The github account used to clone (used to access private repos).
  /// Empty string is public clone (only public repos).
  #[serde(default)]
  pub github_account: String,

  /// The dockerhub account used to push the image to dockerhub.
  /// Empty string means no dockerhub push (server local build).
  #[serde(default)]
  pub docker_account: String,

  /// The docker organization which the image should be pushed under.
  /// Empty string means no organization.
  #[serde(default)]
  pub docker_organization: String,

  /// The optional command run after repo clone and before docker build.
  #[serde(default)]
  pub pre_build: SystemCommand,

  /// The path of the docker build context relative to the root of the repo.
  /// Default: "." (the root of the repo).
  #[serde(default = "default_build_path")]
  pub build_path: String,

  /// The path of the dockerfile relative to the build path.
  #[serde(default = "default_dockerfile_path")]
  pub dockerfile_path: String,

  /// Docker build arguments
  #[serde(default)]
  pub build_args: Vec<EnvironmentVar>,

  /// Docker labels
  #[serde(default)]
  pub labels: Vec<EnvironmentVar>,

  /// Any extra docker cli arguments to be included in the build command
  #[serde(default)]
  pub extra_args: Vec<String>,

  /// Whether to use buildx to build (eg `docker buildx build ...`)
  #[serde(default)]
  pub use_buildx: bool,

  /// Whether incoming webhooks actually trigger action.
  #[serde(default = "default_webhook_enabled")]
  pub webhook_enabled: bool,
}

impl From<BuildConfig>
  for monitor_client::entities::build::BuildConfig
{
  fn from(value: BuildConfig) -> Self {
    monitor_client::entities::build::BuildConfig {
      builder_id: value.builder_id,
      skip_secret_interp: value.skip_secret_interp,
      version: monitor_client::entities::Version {
        major: value.version.major,
        minor: value.version.minor,
        patch: value.version.patch,
      },
      image_name: Default::default(),
      image_tag: Default::default(),
      git_provider: String::from("github.com"),
      git_https: true,
      repo: value.repo,
      branch: value.branch,
      commit: value.commit,
      git_account: value.github_account,
      pre_build: monitor_client::entities::SystemCommand {
        path: value.pre_build.path,
        command: value.pre_build.command,
      },
      build_path: value.build_path,
      dockerfile_path: value.dockerfile_path,
      build_args: value
        .build_args
        .into_iter()
        .map(Into::into)
        .collect(),
      secret_args: Default::default(),
      labels: value.labels.into_iter().map(Into::into).collect(),
      extra_args: value.extra_args,
      use_buildx: value.use_buildx,
      image_registry: if value.docker_account.is_empty() {
        ImageRegistry::None(NoData {})
      } else {
        ImageRegistry::Standard(StandardRegistryConfig {
          domain: String::from("docker.io"),
          account: value.docker_account,
          organization: value.docker_organization,
        })
      },
      webhook_enabled: value.webhook_enabled,
      webhook_secret: Default::default(),
    }
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
