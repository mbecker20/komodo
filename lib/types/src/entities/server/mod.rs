use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use mungos::MungosIndexed;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{I64, i64_is_zero};

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

    #[serde(default, skip_serializing_if = "i64_is_zero")]
    #[builder(setter(skip))]
    pub created_at: I64,

    #[serde(default)]
    #[builder(setter(skip))]
    pub updated_at: I64,

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

impl From<PartialServerConfig> for ServerConfig {
    fn from(value: PartialServerConfig) -> ServerConfig {
        ServerConfig {
            address: value.address.unwrap_or_default(),
            enabled: value.enabled.unwrap_or(default_enabled()),
            auto_prune: value.auto_prune.unwrap_or(default_auto_prune()),
            region: value.region.unwrap_or_default(),
            cpu_alert: value.cpu_alert.unwrap_or(default_cpu_alert()),
            mem_alert: value.mem_alert.unwrap_or(default_mem_alert()),
            disk_alert: value.disk_alert.unwrap_or(default_disk_alert()),
            to_notify: value.to_notify.unwrap_or_default()
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServerActionState {
    pub pruning_networks: bool,
    pub pruning_containers: bool,
    pub pruning_images: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone, Copy, Default)]
pub enum ServerStatus {
    #[default]
    NotOk,
    Ok,
    Disabled,
}
