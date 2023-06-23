use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use mungos::MungosIndexed;
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
#[derive(Serialize, Deserialize, Debug, Clone, Builder, MungosIndexed)]
pub struct AwsBuilder {
    #[serde(default = "default_region")]
    #[builder(default = "default_region()")]
    pub region: String,

    #[serde(default = "default_instance_type")]
    #[builder(default = "default_instance_type()")]
    pub instance_type: String,

    #[serde(default = "default_volume_gb")]
    #[builder(default = "default_volume_gb()")]
    pub volume_gb: i32,

    pub ami_id: String,
    pub subnet_id: String,
    pub security_group_ids: Vec<String>,
    pub key_pair_name: String,
    pub assign_public_ip: bool,
}

fn default_region() -> String {
    String::from("us-east-1")
}

fn default_instance_type() -> String {
    String::from("c5.2xlarge")
}

fn default_volume_gb() -> i32 {
    20
}
