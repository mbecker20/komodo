use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{builder::AwsBuilderConfig, update::Update};

use super::MonitorWriteRequest;

/// Launch an EC2 instance with the specified config.
/// Response: [Update].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(Update)]
pub struct LaunchServer {
  /// The name of the created server.
  pub name: String,
  /// The configuration used to launch the server.
  pub config: LaunchServerConfig,
}

/// The cloud specific config.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "params")]
pub enum LaunchServerConfig {
  /// Launch a server on AWS.
  Aws(LaunchAwsServerConfig),
}

/// Aws EC2 instance config.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LaunchAwsServerConfig {
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
}

/// For information on AWS volumes, see
/// `<https://docs.aws.amazon.com/ebs/latest/userguide/ebs-volume-types.html>`.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl LaunchAwsServerConfig {
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
    }
  }
}
