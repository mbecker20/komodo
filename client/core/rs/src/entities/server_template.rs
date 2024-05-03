use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use derive_variants::EnumVariants;
use mungos::mongodb::bson::{doc, Document};
use partial_derive2::{MaybeNone, Partial, PartialDiff};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use typeshare::typeshare;

use super::{
  builder::AwsBuilderConfig,
  resource::{AddFilters, Resource, ResourceListItem, ResourceQuery},
};

#[typeshare]
pub type ServerTemplate = Resource<ServerTemplateConfig, ()>;

#[typeshare]
pub type ServerTemplateListItem =
  ResourceListItem<ServerTemplateListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerTemplateListItemInfo {
  /// The cloud provider
  pub provider: String,
  /// The instance type, eg c5.2xlarge on for Aws templates
  pub instance_type: Option<String>,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, EnumVariants)]
#[variant_derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  Display,
  EnumString,
  AsRefStr
)]
#[serde(tag = "type", content = "params")]
pub enum ServerTemplateConfig {
  /// Template to launch an AWS EC2 instance
  Aws(AwsServerTemplateConfig),
}

impl PartialDiff<PartialServerTemplateConfig>
  for ServerTemplateConfig
{
  fn partial_diff(
    &self,
    partial: PartialServerTemplateConfig,
  ) -> PartialServerTemplateConfig {
    match self {
      ServerTemplateConfig::Aws(original) => match partial {
        PartialServerTemplateConfig::Aws(partial) => {
          PartialServerTemplateConfig::Aws(
            original.partial_diff(partial),
          )
        }
      },
    }
  }
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
  EnumString,
  AsRefStr
)]
#[serde(tag = "type", content = "params")]
pub enum PartialServerTemplateConfig {
  Aws(_PartialAwsServerTemplateConfig),
}

impl MaybeNone for PartialServerTemplateConfig {
  fn is_none(&self) -> bool {
    match self {
      PartialServerTemplateConfig::Aws(config) => config.is_none(),
    }
  }
}

impl From<PartialServerTemplateConfig> for ServerTemplateConfig {
  fn from(
    value: PartialServerTemplateConfig,
  ) -> ServerTemplateConfig {
    match value {
      PartialServerTemplateConfig::Aws(config) => {
        ServerTemplateConfig::Aws(config.into())
      }
    }
  }
}

impl From<ServerTemplateConfig> for PartialServerTemplateConfig {
  fn from(value: ServerTemplateConfig) -> Self {
    match value {
      ServerTemplateConfig::Aws(config) => {
        PartialServerTemplateConfig::Aws(config.into())
      }
    }
  }
}

impl ServerTemplateConfig {
  pub fn merge_partial(
    self,
    partial: PartialServerTemplateConfig,
  ) -> ServerTemplateConfig {
    match partial {
      PartialServerTemplateConfig::Aws(partial) => match self {
        ServerTemplateConfig::Aws(config) => {
          let config = AwsServerTemplateConfig {
            region: partial.region.unwrap_or(config.region),
            instance_type: partial
              .instance_type
              .unwrap_or(config.instance_type),
            volumes: partial.volumes.unwrap_or(config.volumes),
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
          };
          ServerTemplateConfig::Aws(config)
        }
      },
    }
  }
}

#[typeshare(serialized_as = "Partial<AwsServerTemplateConfig>")]
pub type _PartialAwsServerTemplateConfig =
  PartialAwsServerTemplateConfig;

/// Aws EC2 instance config.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Partial)]
#[partial_derive(Debug, Clone, Default, Serialize, Deserialize)]
#[partial(skip_serializing_none, from, diff)]
pub struct AwsServerTemplateConfig {
  /// The aws region to launch the server in, eg. us-east-1
  pub region: String,
  /// The instance type to launch, eg. c5.2xlarge
  pub instance_type: String,
  /// Specify the EBS volumes to attach.
  pub volumes: Vec<AwsVolume>,
  /// Specify the ami id to use. Must be set up to start the periphery binary on startup.
  pub ami_id: String,
  /// The subnet to assign to the instance.
  pub subnet_id: String,
  /// The security groups to give to the instance.
  pub security_group_ids: Vec<String>,
  /// The key pair name to give to the instance in case SSH access required.
  pub key_pair_name: String,
  /// Assign a public ip to the instance. Depending on how your network is
  /// setup, this may be required for the instance to reach the public internet.
  pub assign_public_ip: bool,
  /// Use the instances public ip as the address for the server.
  /// Could be used when build instances are created in another non-interconnected network to the core api.
  pub use_public_ip: bool,
  /// The port periphery will be running on in AMI.
  /// Default: `8120`
  #[serde(default = "default_port")]
  #[builder(default = "default_port()")]
  #[partial_default(default_port())]
  pub port: i32,
}

fn default_port() -> i32 {
  8120
}

/// For information on AWS volumes, see
/// `<https://docs.aws.amazon.com/ebs/latest/userguide/ebs-volume-types.html>`.
#[typeshare]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AwsVolume {
  /// The device name (for example, `/dev/sda1` or `xvdh`).
  pub device_name: String,
  /// The size of the volume in GB
  pub size_gb: i32,
  /// The type of volume, eg gp2, gp3, io1
  pub volume_type: Option<String>,
  /// The iops of the volume, or AWS default.
  pub iops: Option<i32>,
  /// The throughput of the volume, or AWS default.
  pub throughput: Option<i32>,
}

impl AwsServerTemplateConfig {
  pub fn from_builder_config(value: &AwsBuilderConfig) -> Self {
    Self {
      region: value.region.clone(),
      instance_type: value.instance_type.clone(),
      volumes: vec![AwsVolume {
        size_gb: value.volume_gb,
        device_name: "/dev/sda1".to_string(),
        volume_type: None,
        iops: None,
        throughput: None,
      }],
      ami_id: value.ami_id.clone(),
      subnet_id: value.subnet_id.clone(),
      security_group_ids: value.security_group_ids.clone(),
      key_pair_name: value.key_pair_name.clone(),
      assign_public_ip: value.assign_public_ip,
      use_public_ip: value.use_public_ip,
      port: value.port,
    }
  }
}

#[typeshare]
pub type ServerTemplateQuery =
  ResourceQuery<ServerTemplateQuerySpecifics>;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, DefaultBuilder,
)]
pub struct ServerTemplateQuerySpecifics {
  pub types: Vec<ServerTemplateConfigVariant>,
}

impl AddFilters for ServerTemplateQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    let types =
      self.types.iter().map(|t| t.as_ref()).collect::<Vec<_>>();
    if !self.types.is_empty() {
      filters.insert("config.type", doc! { "$in": types });
    }
  }
}
