use derive_builder::Builder;
use derive_variants::EnumVariants;
use partial_derive2::{Diff, MaybeNone, Partial, PartialDiff};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use typeshare::typeshare;

use super::{
  config::{DockerRegistry, GitProvider},
  resource::{AddFilters, Resource, ResourceListItem, ResourceQuery},
  MergePartial,
};

#[typeshare]
pub type Builder = Resource<BuilderConfig, ()>;

#[typeshare]
pub type BuilderListItem = ResourceListItem<BuilderListItemInfo>;

#[typeshare(serialized_as = "Partial<BuilderConfig>")]
pub type _PartialBuilderConfig = PartialBuilderConfig;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuilderListItemInfo {
  /// 'Server' or 'Aws'
  pub builder_type: String,
  /// If 'Server': the server id
  /// If 'Aws': the instance type (eg. c5.xlarge)
  pub instance_type: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, EnumVariants)]
#[variant_derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  Display,
  EnumString
)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::large_enum_variant)]
pub enum BuilderConfig {
  /// Use a connected server an image builder.
  Server(ServerBuilderConfig),

  /// Use EC2 instances spawned on demand as an image builder.
  Aws(AwsBuilderConfig),
}

impl Default for BuilderConfig {
  fn default() -> Self {
    Self::Aws(Default::default())
  }
}

/// Partial representation of [BuilderConfig]
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, EnumVariants)]
#[variant_derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  Display,
  EnumString
)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::large_enum_variant)]
pub enum PartialBuilderConfig {
  Server(Option<_PartialServerBuilderConfig>),
  Aws(Option<_PartialAwsBuilderConfig>),
}

impl Default for PartialBuilderConfig {
  fn default() -> Self {
    Self::Aws(Default::default())
  }
}

impl MaybeNone for PartialBuilderConfig {
  fn is_none(&self) -> bool {
    match self {
      PartialBuilderConfig::Server(config) => config.is_none(),
      PartialBuilderConfig::Aws(config) => config.is_none(),
    }
  }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuilderConfigDiff {
  Server(ServerBuilderConfigDiff),
  Aws(AwsBuilderConfigDiff),
}

impl From<BuilderConfigDiff> for PartialBuilderConfig {
  fn from(value: BuilderConfigDiff) -> Self {
    match value {
      BuilderConfigDiff::Server(diff) => {
        PartialBuilderConfig::Server(Some(diff.into()))
      }
      BuilderConfigDiff::Aws(diff) => {
        PartialBuilderConfig::Aws(Some(diff.into()))
      }
    }
  }
}

impl Diff for BuilderConfigDiff {
  fn iter_field_diffs(
    &self,
  ) -> impl Iterator<Item = partial_derive2::FieldDiff> {
    match self {
      BuilderConfigDiff::Server(diff) => {
        diff.iter_field_diffs().collect::<Vec<_>>().into_iter()
      }
      BuilderConfigDiff::Aws(diff) => {
        diff.iter_field_diffs().collect::<Vec<_>>().into_iter()
      }
    }
  }
}

impl PartialDiff<PartialBuilderConfig, BuilderConfigDiff>
  for BuilderConfig
{
  fn partial_diff(
    &self,
    partial: PartialBuilderConfig,
  ) -> BuilderConfigDiff {
    match self {
      BuilderConfig::Server(original) => match partial {
        PartialBuilderConfig::Server(partial) => {
          BuilderConfigDiff::Server(
            original.partial_diff(partial.unwrap_or_default()),
          )
        }
        PartialBuilderConfig::Aws(partial) => {
          let default = AwsBuilderConfig::default();
          BuilderConfigDiff::Aws(
            default.partial_diff(partial.unwrap_or_default()),
          )
        }
      },
      BuilderConfig::Aws(original) => match partial {
        PartialBuilderConfig::Aws(partial) => BuilderConfigDiff::Aws(
          original.partial_diff(partial.unwrap_or_default()),
        ),
        PartialBuilderConfig::Server(partial) => {
          let default = ServerBuilderConfig::default();
          BuilderConfigDiff::Server(
            default.partial_diff(partial.unwrap_or_default()),
          )
        }
      },
    }
  }
}

impl MaybeNone for BuilderConfigDiff {
  fn is_none(&self) -> bool {
    match self {
      BuilderConfigDiff::Server(config) => config.is_none(),
      BuilderConfigDiff::Aws(config) => config.is_none(),
    }
  }
}

impl From<PartialBuilderConfig> for BuilderConfig {
  fn from(value: PartialBuilderConfig) -> BuilderConfig {
    match value {
      PartialBuilderConfig::Server(server) => {
        BuilderConfig::Server(server.unwrap_or_default().into())
      }
      PartialBuilderConfig::Aws(builder) => {
        BuilderConfig::Aws(builder.unwrap_or_default().into())
      }
    }
  }
}

impl From<BuilderConfig> for PartialBuilderConfig {
  fn from(value: BuilderConfig) -> Self {
    match value {
      BuilderConfig::Server(config) => {
        PartialBuilderConfig::Server(Some(config.into()))
      }
      BuilderConfig::Aws(config) => {
        PartialBuilderConfig::Aws(Some(config.into()))
      }
    }
  }
}

impl MergePartial for BuilderConfig {
  type Partial = PartialBuilderConfig;
  fn merge_partial(
    self,
    partial: PartialBuilderConfig,
  ) -> BuilderConfig {
    match partial {
      PartialBuilderConfig::Server(partial) => match self {
        BuilderConfig::Server(config) => {
          let partial = partial.unwrap_or_default();
          let config = ServerBuilderConfig {
            server_id: partial.server_id.unwrap_or(config.server_id),
          };
          BuilderConfig::Server(config)
        }
        _ => {
          BuilderConfig::Server(partial.unwrap_or_default().into())
        }
      },
      PartialBuilderConfig::Aws(partial) => match self {
        BuilderConfig::Aws(config) => {
          let partial = partial.unwrap_or_default();
          let config = AwsBuilderConfig {
            region: partial.region.unwrap_or(config.region),
            instance_type: partial
              .instance_type
              .unwrap_or(config.instance_type),
            volume_gb: partial.volume_gb.unwrap_or(config.volume_gb),
            ami_id: partial.ami_id.unwrap_or(config.ami_id),
            subnet_id: partial.subnet_id.unwrap_or(config.subnet_id),
            security_group_ids: partial
              .security_group_ids
              .unwrap_or(config.security_group_ids),
            key_pair_name: partial
              .key_pair_name
              .unwrap_or(config.key_pair_name),
            assign_public_ip: partial
              .assign_public_ip
              .unwrap_or(config.assign_public_ip),
            use_public_ip: partial
              .use_public_ip
              .unwrap_or(config.use_public_ip),
            port: partial.port.unwrap_or(config.port),
            use_https: partial.use_https.unwrap_or(config.use_https),
            user_data: partial.user_data.unwrap_or(config.user_data),
            git_providers: partial
              .git_providers
              .unwrap_or(config.git_providers),
            docker_registries: partial
              .docker_registries
              .unwrap_or(config.docker_registries),
            secrets: partial.secrets.unwrap_or(config.secrets),
          };
          BuilderConfig::Aws(config)
        }
        _ => BuilderConfig::Aws(partial.unwrap_or_default().into()),
      },
    }
  }
}

#[typeshare(serialized_as = "Partial<ServerBuilderConfig>")]
pub type _PartialServerBuilderConfig = PartialServerBuilderConfig;

/// Configuration for a Komodo Server Builder.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Builder, Partial,
)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[partial(skip_serializing_none, from, diff)]
pub struct ServerBuilderConfig {
  /// The server id of the builder
  #[serde(alias = "server")]
  #[partial_attr(serde(alias = "server"))]
  pub server_id: String,
}

#[typeshare(serialized_as = "Partial<AwsBuilderConfig>")]
pub type _PartialAwsBuilderConfig = PartialAwsBuilderConfig;

/// Configuration for an AWS builder.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[partial(skip_serializing_none, from, diff)]
pub struct AwsBuilderConfig {
  /// The AWS region to create the instance in
  #[serde(default = "aws_default_region")]
  #[builder(default = "aws_default_region()")]
  #[partial_default(aws_default_region())]
  pub region: String,

  /// The instance type to create for the build
  #[serde(default = "aws_default_instance_type")]
  #[builder(default = "aws_default_instance_type()")]
  #[partial_default(aws_default_instance_type())]
  pub instance_type: String,

  /// The size of the builder volume in gb
  #[serde(default = "aws_default_volume_gb")]
  #[builder(default = "aws_default_volume_gb()")]
  #[partial_default(aws_default_volume_gb())]
  pub volume_gb: i32,

  /// The port periphery will be running on.
  /// Default: `8120`
  #[serde(default = "default_port")]
  #[builder(default = "default_port()")]
  #[partial_default(default_port())]
  pub port: i32,

  #[serde(default = "default_use_https")]
  #[builder(default = "default_use_https()")]
  #[partial_default(default_use_https())]
  pub use_https: bool,

  /// The EC2 ami id to create.
  /// The ami should have the periphery client configured to start on startup,
  /// and should have the necessary github / dockerhub accounts configured.
  #[serde(default)]
  #[builder(default)]
  pub ami_id: String,
  /// The subnet id to create the instance in.
  #[serde(default)]
  #[builder(default)]
  pub subnet_id: String,
  /// The key pair name to attach to the instance
  #[serde(default)]
  #[builder(default)]
  pub key_pair_name: String,
  /// Whether to assign the instance a public IP address.
  /// Likely needed for the instance to be able to reach the open internet.
  #[serde(default)]
  #[builder(default)]
  pub assign_public_ip: bool,
  /// Whether core should use the public IP address to communicate with periphery on the builder.
  /// If false, core will communicate with the instance using the private IP.
  #[serde(default)]
  #[builder(default)]
  pub use_public_ip: bool,
  /// The security group ids to attach to the instance.
  /// This should include a security group to allow core inbound access to the periphery port.
  #[serde(default)]
  #[builder(default)]
  pub security_group_ids: Vec<String>,
  /// The user data to deploy the instance with.
  #[serde(default)]
  #[builder(default)]
  pub user_data: String,

  /// Which git providers are available on the AMI
  #[serde(default)]
  #[builder(default)]
  pub git_providers: Vec<GitProvider>,
  /// Which docker registries are available on the AMI.
  #[serde(default)]
  #[builder(default)]
  pub docker_registries: Vec<DockerRegistry>,
  /// Which secrets are available on the AMI.
  #[serde(default)]
  #[builder(default)]
  pub secrets: Vec<String>,
}

impl Default for AwsBuilderConfig {
  fn default() -> Self {
    Self {
      region: aws_default_region(),
      instance_type: aws_default_instance_type(),
      volume_gb: aws_default_volume_gb(),
      port: default_port(),
      use_https: default_use_https(),
      ami_id: Default::default(),
      subnet_id: Default::default(),
      security_group_ids: Default::default(),
      key_pair_name: Default::default(),
      assign_public_ip: Default::default(),
      use_public_ip: Default::default(),
      user_data: Default::default(),
      git_providers: Default::default(),
      docker_registries: Default::default(),
      secrets: Default::default(),
    }
  }
}

impl AwsBuilderConfig {
  pub fn builder() -> AwsBuilderConfigBuilder {
    AwsBuilderConfigBuilder::default()
  }
}

fn aws_default_region() -> String {
  String::from("us-east-1")
}

fn aws_default_instance_type() -> String {
  String::from("c5.2xlarge")
}

fn aws_default_volume_gb() -> i32 {
  20
}

fn default_port() -> i32 {
  8120
}

fn default_use_https() -> bool {
  true
}

#[typeshare]
pub type BuilderQuery = ResourceQuery<BuilderQuerySpecifics>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuilderQuerySpecifics {}

impl AddFilters for BuilderQuerySpecifics {}
