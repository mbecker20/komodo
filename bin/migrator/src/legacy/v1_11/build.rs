use komodo_client::entities::{
  build::StandardRegistryConfig, EnvironmentVar, NoData,
  SystemCommand, Version, I64,
};
use serde::{Deserialize, Serialize};

use super::resource::Resource;

pub type Build = Resource<BuildConfig, BuildInfo>;

impl From<Build> for komodo_client::entities::build::Build {
  fn from(value: Build) -> Self {
    komodo_client::entities::build::Build {
      id: value.id,
      name: value.name,
      description: value.description,
      updated_at: value.updated_at,
      tags: value.tags,
      info: komodo_client::entities::build::BuildInfo {
        last_built_at: value.info.last_built_at,
        built_hash: None,
        built_message: None,
        latest_hash: None,
        latest_message: None,
      },
      config: value.config.into(),
      base_permission: Default::default(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildInfo {
  pub last_built_at: I64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
  /// Which builder is used to build the image.
  #[serde(default, alias = "builder")]
  pub builder_id: String,

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

  /// The optional command run after repo clone and before docker build.
  #[serde(default)]
  pub pre_build: SystemCommand,

  /// Configuration for the registry to push the built image to.
  #[serde(default)]
  pub image_registry: ImageRegistry,

  /// The path of the docker build context relative to the root of the repo.
  /// Default: "." (the root of the repo).
  #[serde(default = "default_build_path")]
  pub build_path: String,

  /// The path of the dockerfile relative to the build path.
  #[serde(default = "default_dockerfile_path")]
  pub dockerfile_path: String,

  /// Whether to skip secret interpolation in the build_args.
  #[serde(default)]
  pub skip_secret_interp: bool,

  /// Whether to use buildx to build (eg `docker buildx build ...`)
  #[serde(default)]
  pub use_buildx: bool,

  /// Whether incoming webhooks actually trigger action.
  #[serde(default = "default_webhook_enabled")]
  pub webhook_enabled: bool,

  /// Any extra docker cli arguments to be included in the build command
  #[serde(default)]
  pub extra_args: Vec<String>,

  /// Docker build arguments.
  ///
  /// These values are visible in the final image by running `docker inspect`.
  #[serde(
    default,
    deserialize_with = "komodo_client::entities::env_vars_deserializer"
  )]
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
    deserialize_with = "komodo_client::entities::env_vars_deserializer"
  )]
  pub secret_args: Vec<EnvironmentVar>,

  /// Docker labels
  #[serde(
    default,
    deserialize_with = "komodo_client::entities::env_vars_deserializer"
  )]
  pub labels: Vec<EnvironmentVar>,
}

impl From<BuildConfig>
  for komodo_client::entities::build::BuildConfig
{
  fn from(value: BuildConfig) -> Self {
    komodo_client::entities::build::BuildConfig {
      builder_id: value.builder_id,
      skip_secret_interp: value.skip_secret_interp,
      version: komodo_client::entities::Version {
        major: value.version.major,
        minor: value.version.minor,
        patch: value.version.patch,
      },
      auto_increment_version: true,
      image_name: Default::default(),
      image_tag: Default::default(),
      git_provider: String::from("github.com"),
      git_https: true,
      repo: value.repo,
      branch: value.branch,
      commit: value.commit,
      git_account: value.github_account,
      pre_build: komodo_client::entities::SystemCommand {
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
      webhook_enabled: value.webhook_enabled,
      webhook_secret: Default::default(),
      image_registry: value.image_registry.into(),
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
  /// Todo. Will point to a custom "Registry" resource by id
  Custom(String),
}

impl Default for ImageRegistry {
  fn default() -> Self {
    Self::None(NoData {})
  }
}

impl From<ImageRegistry>
  for komodo_client::entities::build::ImageRegistry
{
  fn from(value: ImageRegistry) -> Self {
    match value {
      ImageRegistry::None(_) | ImageRegistry::Custom(_) => {
        komodo_client::entities::build::ImageRegistry::None(NoData {})
      }
      ImageRegistry::DockerHub(params) => {
        komodo_client::entities::build::ImageRegistry::Standard(
          StandardRegistryConfig {
            domain: String::from("docker.io"),
            account: params.account,
            organization: params.organization,
          },
        )
      }
      ImageRegistry::Ghcr(params) => {
        komodo_client::entities::build::ImageRegistry::Standard(
          StandardRegistryConfig {
            domain: String::from("ghcr.io"),
            account: params.account,
            organization: params.organization,
          },
        )
      }
      ImageRegistry::AwsEcr(label) => {
        komodo_client::entities::build::ImageRegistry::AwsEcr(label)
      }
    }
  }
}

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
