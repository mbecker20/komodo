use std::path::PathBuf;

use async_timing_util::Timelength;
use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use diff::Diff;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

use crate::{diff::*, PermissionsMap};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Diff, Builder)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct Server {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    #[builder(setter(skip))]
    pub id: String,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub name: String,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub address: String,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "hashmap_diff_no_change")]))]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub enabled: bool,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "vec_diff_no_change")]))]
    pub to_notify: Vec<String>, // slack users to notify

    #[serde(default = "default_cpu_alert")]
    pub cpu_alert: f64,
    #[serde(default = "default_mem_alert")]
    pub mem_alert: f64,
    #[serde(default = "default_disk_alert")]
    pub disk_alert: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub stats_interval: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub region: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub instance_id: Option<String>,

    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    #[builder(setter(skip))]
    pub created_at: String,
    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    #[builder(setter(skip))]
    pub updated_at: String,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            address: Default::default(),
            permissions: Default::default(),
            enabled: true,
            to_notify: Default::default(),
            cpu_alert: default_cpu_alert(),
            mem_alert: default_mem_alert(),
            disk_alert: default_disk_alert(),
            stats_interval: Default::default(),
            region: Default::default(),
            instance_id: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}

fn default_cpu_alert() -> f64 {
    50.0
}

fn default_mem_alert() -> f64 {
    75.0
}

fn default_disk_alert() -> f64 {
    75.0
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerWithStatus {
    pub server: Server,
    pub status: ServerStatus,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServerActionState {
    pub pruning_networks: bool,
    pub pruning_containers: bool,
    pub pruning_images: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ServerStatus {
    Ok,
    NotOk,
    Disabled,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemStats {
    pub cpu_perc: f32,     // in %
    pub mem_used_gb: f64,  // in GB
    pub mem_total_gb: f64, // in GB
    pub disk: DiskUsage,
    pub networks: Vec<SystemNetwork>,
    pub polling_rate: Timelength,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct DiskUsage {
    pub used_gb: f64,  // in GB
    pub total_gb: f64, // in GB
    pub read_kb: f64,  // in kB
    pub write_kb: f64, // in kB
    pub disks: Vec<SingleDiskUsage>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct SingleDiskUsage {
    pub mount: PathBuf,
    pub used_gb: f64,  // in GB
    pub total_gb: f64, // in GB
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemNetwork {
    pub name: String,
    pub recieved_kb: f64,    // in kB
    pub transmitted_kb: f64, // in kB
}
