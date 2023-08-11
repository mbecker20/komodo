use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{builder::AwsBuilderConfig, update::Update};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Update)]
pub struct LaunchServer {
    pub name: String,
    pub config: LaunchServerConfig,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "params")]
pub enum LaunchServerConfig {
    Aws(LaunchAwsServerConfig),
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LaunchAwsServerConfig {
    pub region: String,
    pub instance_type: String,
    pub volumes: Vec<AwsVolume>,
    pub ami_id: String,
    pub subnet_id: String,
    pub security_group_ids: Vec<String>,
    pub key_pair_name: String,
    pub assign_public_ip: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AwsVolume {
    pub device_name: String,
    pub size_gb: i32,
    pub volume_type: Option<String>,
    pub iops: Option<i32>,
    pub throughput: Option<i32>,
}

impl From<&AwsBuilderConfig> for LaunchAwsServerConfig {
    fn from(value: &AwsBuilderConfig) -> Self {
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
        }
    }
}
