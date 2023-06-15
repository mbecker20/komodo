use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use mungos::MungosIndexed;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::PermissionsMap;

pub mod docker_image;
pub mod docker_network;
pub mod stats;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, MungosIndexed)]
pub struct Server {
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

    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[builder(setter(skip))]
    pub created_at: String,

    #[serde(default)]
    #[builder(setter(skip))]
    pub updated_at: String,

    #[serde(default)]
    pub tags: Vec<String>,

    pub config: ServerConfig,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial, MungosIndexed)]
#[partial_derive(Serialize, Deserialize, Debug, Clone)]
#[skip_serializing_none]
pub struct ServerConfig {
    pub address: String,

    #[serde(default = "default_enabled")]
    #[builder(default = "default_enabled()")]
    #[index]
    pub enabled: bool,

    #[serde(default = "default_auto_prune")]
    #[builder(default = "default_auto_prune()")]
    #[index]
    pub auto_prune: bool,

    #[serde(default)]
    #[builder(default)]
    pub region: String,

    #[serde(default = "default_cpu_alert")]
    #[builder(default = "default_cpu_alert()")]
    pub cpu_alert: f32,

    #[serde(default = "default_mem_alert")]
    #[builder(default = "default_mem_alert()")]
    pub mem_alert: f64,

    #[serde(default = "default_disk_alert")]
    #[builder(default = "default_disk_alert()")]
    pub disk_alert: f64,

    #[serde(default)]
    #[builder(default)]
    pub to_notify: Vec<String>, // slack users to notify
}

fn default_enabled() -> bool {
    true
}

fn default_auto_prune() -> bool {
    true
}

fn default_cpu_alert() -> f32 {
    95.0
}

fn default_mem_alert() -> f64 {
    80.0
}

fn default_disk_alert() -> f64 {
    75.0
}
