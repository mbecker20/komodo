use derive_builder::Builder;
use derive_variants::EnumVariants;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use typeshare::typeshare;

use super::resource::{
  AddFilters, Resource, ResourceListItem, ResourceQuery,
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
  pub provider: String,
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
pub enum BuilderConfig {
  /// Use a connected server an image builder.
  Server(ServerBuilderConfig),

  /// Use EC2 instances spawned on demand as an image builder.
  Aws(AwsBuilderConfig),
}

#[typeshare(serialized_as = "Partial<ServerBuilderConfig>")]
pub type _PartialServerBuilderConfig = PartialServerBuilderConfig;

#[typeshare(serialized_as = "Partial<AwsBuilderConfig>")]
pub type _PartialAwsBuilderConfig = PartialAwsBuilderConfig;

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
pub enum PartialBuilderConfig {
  Server(_PartialServerBuilderConfig),
  Aws(_PartialAwsBuilderConfig),
}

impl From<PartialBuilderConfig> for BuilderConfig {
  fn from(value: PartialBuilderConfig) -> BuilderConfig {
    match value {
      PartialBuilderConfig::Server(server) => {
        BuilderConfig::Server(server.into())
      }
      PartialBuilderConfig::Aws(builder) => {
        BuilderConfig::Aws(builder.into())
      }
    }
  }
}

impl BuilderConfig {
  pub fn merge_partial(
    self,
    partial: PartialBuilderConfig,
  ) -> BuilderConfig {
    match partial {
      PartialBuilderConfig::Server(partial) => match self {
        BuilderConfig::Server(config) => {
          let config = ServerBuilderConfig {
            server_id: partial.server_id.unwrap_or(config.server_id),
          };
          BuilderConfig::Server(config)
        }
        _ => BuilderConfig::Server(partial.into()),
      },
      PartialBuilderConfig::Aws(partial) => match self {
        BuilderConfig::Aws(config) => {
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
            github_accounts: partial
              .github_accounts
              .unwrap_or(config.github_accounts),
            docker_accounts: partial
              .docker_accounts
              .unwrap_or(config.docker_accounts),
          };
          BuilderConfig::Aws(config)
        }
        _ => BuilderConfig::Aws(partial.into()),
      },
    }
  }
}

/// Configuration for a monitor server builder.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[skip_serializing_none]
#[partial_from]
pub struct ServerBuilderConfig {
  /// The server id of the builder
  #[serde(alias = "server")]
  #[partial_attr(serde(alias = "server"))]
  pub server_id: String,
}

/// Configuration for an AWS builder.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[skip_serializing_none]
#[partial_from]
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

  /// The port periphery will be running on
  #[serde(default = "default_port")]
  #[builder(default = "default_port()")]
  #[partial_default(default_port())]
  pub port: i32,

  /// The EC2 ami id to create.
  /// The ami should have the periphery client configured to start on startup,
  /// and should have the necessary github / dockerhub accounts configured.
  pub ami_id: String,
  /// The subnet id to create the instance in.
  pub subnet_id: String,
  /// The security group ids to attach to the instance.
  /// This should include a security group to allow core inbound access to the periphery port.
  pub security_group_ids: Vec<String>,
  /// The key pair name to attach to the instance
  pub key_pair_name: String,
  /// Whether to assign the instance a public IP address.
  /// Likely needed for the instance to be able to reach the open internet.
  pub assign_public_ip: bool,
  /// Whether core should use the public IP address to communicate with periphery on the builder.
  /// If false, core will communicate with the instance using the private IP.
  pub use_public_ip: bool,

  /// Which github accounts (usernames) are available on the AMI
  #[serde(default)]
  pub github_accounts: Vec<String>,
  /// Which dockerhub accounts (usernames) are available on the AMI
  #[serde(default)]
  pub docker_accounts: Vec<String>,
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

#[typeshare]
pub type BuilderQuery = ResourceQuery<BuilderQuerySpecifics>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuilderQuerySpecifics {}

impl AddFilters for BuilderQuerySpecifics {}
