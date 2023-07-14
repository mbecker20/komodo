use derive_builder::Builder;
use mungos::{
    derive::{MungosIndexed, StringObjectId},
    mongodb::bson::serde_helpers::hex_string_as_object_id,
};
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{MongoId, I64};

use super::PermissionsMap;

pub mod docker_image;
pub mod docker_network;
pub mod stats;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, MungosIndexed, StringObjectId)]
pub struct Server {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    #[builder(setter(skip))]
    pub id: MongoId,

    #[unique_index]
    pub name: String,

    #[serde(default)]
    #[builder(default)]
    pub description: String,

    #[serde(default)]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[serde(default)]
    #[builder(setter(skip))]
    pub updated_at: I64,

    #[serde(default)]
    #[builder(default)]
    pub tags: Vec<String>,

    pub config: ServerConfig,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial, MungosIndexed)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[skip_serializing_none]
#[partial_from]
pub struct ServerConfig {
    pub address: String,

    #[serde(default = "default_enabled")]
    #[builder(default = "default_enabled()")]
    #[partial_default(default_enabled())]
    #[index]
    pub enabled: bool,

    #[serde(default = "default_auto_prune")]
    #[builder(default = "default_auto_prune()")]
    #[partial_default(default_auto_prune())]
    #[index]
    pub auto_prune: bool,

    #[serde(default)]
    #[builder(default)]
    pub region: String,

    #[serde(default = "default_cpu_warning")]
    #[builder(default = "default_cpu_warning()")]
    #[partial_default(default_cpu_warning())]
    pub cpu_warning: f32,

    #[serde(default = "default_cpu_critical")]
    #[builder(default = "default_cpu_critical()")]
    #[partial_default(default_cpu_critical())]
    pub cpu_critical: f32,

    #[serde(default = "default_mem_warning")]
    #[builder(default = "default_mem_warning()")]
    #[partial_default(default_mem_warning())]
    pub mem_warning: f64,

    #[serde(default = "default_mem_critical")]
    #[builder(default = "default_mem_critical()")]
    #[partial_default(default_mem_critical())]
    pub mem_critical: f64,

    #[serde(default = "default_disk_warning")]
    #[builder(default = "default_disk_warning()")]
    #[partial_default(default_disk_warning())]
    pub disk_warning: f64,

    #[serde(default = "default_disk_critical")]
    #[builder(default = "default_disk_critical()")]
    #[partial_default(default_disk_critical())]
    pub disk_critical: f64,

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

fn default_cpu_warning() -> f32 {
    90.0
}

fn default_cpu_critical() -> f32 {
    99.0
}

fn default_mem_warning() -> f64 {
    75.0
}

fn default_mem_critical() -> f64 {
    95.0
}

fn default_disk_warning() -> f64 {
    75.0
}

fn default_disk_critical() -> f64 {
    95.0
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
