use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use mungos::MungosIndexed;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{i64_is_zero, I64};

use super::PermissionsMap;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, MungosIndexed)]
pub struct Builder {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    #[builder(setter(skip))]
    pub id: String,

    #[unique_index]
    pub name: String,

    #[serde(default)]
    #[builder(default)]
    pub description: String,

    #[serde(default)]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[serde(default, skip_serializing_if = "i64_is_zero")]
    #[builder(setter(skip))]
    pub created_at: I64,

    #[serde(default)]
    #[builder(setter(skip))]
    pub updated_at: I64,

    pub config: BuilderConfig,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, MungosIndexed)]
#[serde(tag = "type", content = "params")]
pub enum BuilderConfig {
    AwsBuilder(AwsBuilder),
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, MungosIndexed)]
#[serde(tag = "type", content = "params")]
pub enum PartialBuilderConfig {
    AwsBuilder(PartialAwsBuilder),
}

impl From<PartialBuilderConfig> for BuilderConfig {
    fn from(value: PartialBuilderConfig) -> BuilderConfig {
        match value {
            PartialBuilderConfig::AwsBuilder(builder) => BuilderConfig::AwsBuilder(builder.into()),
        }
    }
}

impl BuilderConfig {
    pub fn merge_partial(self, partial: PartialBuilderConfig) -> BuilderConfig {
        match partial {
            PartialBuilderConfig::AwsBuilder(partial) => match self {
                BuilderConfig::AwsBuilder(config) => {
                    let config = AwsBuilder {
                        region: partial.region.unwrap_or(config.region),
                        instance_type: partial.instance_type.unwrap_or(config.instance_type),
                        volume_gb: partial.volume_gb.unwrap_or(config.volume_gb),
                        ami_id: partial.ami_id.unwrap_or(config.ami_id),
                        subnet_id: partial.subnet_id.unwrap_or(config.subnet_id),
                        security_group_ids: partial
                            .security_group_ids
                            .unwrap_or(config.security_group_ids),
                        key_pair_name: partial.key_pair_name.unwrap_or(config.key_pair_name),
                        assign_public_ip: partial
                            .assign_public_ip
                            .unwrap_or(config.assign_public_ip),
                    };
                    BuilderConfig::AwsBuilder(config)
                } // _ => BuilderConfig::AwsBuilder(partial.into()),
            },
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone)]
#[skip_serializing_none]
pub struct AwsBuilder {
    #[serde(default = "aws_default_region")]
    #[builder(default = "aws_default_region()")]
    pub region: String,

    #[serde(default = "aws_default_instance_type")]
    #[builder(default = "aws_default_instance_type()")]
    pub instance_type: String,

    #[serde(default = "aws_default_volume_gb")]
    #[builder(default = "aws_default_volume_gb()")]
    pub volume_gb: i32,

    pub ami_id: String,
    pub subnet_id: String,
    pub security_group_ids: Vec<String>,
    pub key_pair_name: String,
    pub assign_public_ip: bool,
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

impl From<PartialAwsBuilder> for AwsBuilder {
    fn from(value: PartialAwsBuilder) -> AwsBuilder {
        AwsBuilder {
            region: value.region.unwrap_or(aws_default_region()),
            instance_type: value.instance_type.unwrap_or(aws_default_instance_type()),
            volume_gb: value.volume_gb.unwrap_or(aws_default_volume_gb()),
            ami_id: value.ami_id.unwrap_or_default(),
            subnet_id: value.subnet_id.unwrap_or_default(),
            security_group_ids: value.security_group_ids.unwrap_or_default(),
            key_pair_name: value.key_pair_name.unwrap_or_default(),
            assign_public_ip: value.assign_public_ip.unwrap_or_default(),
        }
    }
}
