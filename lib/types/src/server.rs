use std::path::PathBuf;

use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use diff::Diff;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

use crate::{diff::*, PermissionsMap, Timelength};

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
    #[diff(attr(#[serde(skip_serializing_if = "f32_diff_no_change")]))]
    pub cpu_alert: f32,

    #[serde(default = "default_mem_alert")]
    #[diff(attr(#[serde(skip_serializing_if = "f64_diff_no_change")]))]
    pub mem_alert: f64,

    #[serde(default = "default_disk_alert")]
    #[diff(attr(#[serde(skip_serializing_if = "f64_diff_no_change")]))]
    pub disk_alert: f64,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "timelength_diff_no_change")]))]
    pub stats_interval: Timelength,

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

fn default_cpu_alert() -> f32 {
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
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct SystemStatsQuery {
    #[serde(default)]
    pub networks: bool,
    #[serde(default)]
    pub components: bool,
    #[serde(default)]
    pub processes: bool,
}

impl SystemStatsQuery {
    pub fn all() -> SystemStatsQuery {
        SystemStatsQuery {
            networks: true,
            components: true,
            processes: true,
        }
    }

    pub fn none() -> SystemStatsQuery {
        Default::default()
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SystemStats {
    pub cpu_perc: f32,     // in %
    pub mem_used_gb: f64,  // in GB
    pub mem_total_gb: f64, // in GB
    pub disk: DiskUsage,
    pub networks: Vec<SystemNetwork>,
    pub components: Vec<SystemComponent>,
    pub processes: Vec<SystemProcess>,
    pub polling_rate: Timelength,
    pub refresh_ts: u128,
    pub refresh_list_ts: u128,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DiskUsage {
    pub used_gb: f64,  // in GB
    pub total_gb: f64, // in GB
    pub read_kb: f64,  // in kB
    pub write_kb: f64, // in kB
    pub disks: Vec<SingleDiskUsage>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleDiskUsage {
    pub mount: PathBuf,
    pub used_gb: f64,  // in GB
    pub total_gb: f64, // in GB
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemNetwork {
    pub name: String,
    pub recieved_kb: f64,    // in kB
    pub transmitted_kb: f64, // in kB
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemComponent {
    pub label: String,
    pub temp: f32,
    pub max: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub critical: Option<f32>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemProcess {
    pub pid: u32,
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub exe: String,
    pub cmd: Vec<String>,
    pub cpu_perc: f32,
    pub mem_mb: f64,
    pub disk_read_kb: f64,
    pub disk_write_kb: f64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SystemStatsRecord {
    pub server_id: String,
    pub ts: i64,           // unix ts milliseconds
    pub cpu_perc: f32,     // in %
    pub mem_used_gb: f64,  // in GB
    pub mem_total_gb: f64, // in GB
    pub disk: DiskUsage,
    pub networks: Vec<SystemNetwork>,
    pub components: Vec<SystemComponent>,
    pub processes: Vec<SystemProcess>,
    pub polling_rate: Timelength,
}

impl SystemStatsRecord {
    pub fn from_stats(server_id: String, ts: i64, stats: SystemStats) -> SystemStatsRecord {
        SystemStatsRecord {
            server_id,
            ts,
            cpu_perc: stats.cpu_perc,
            mem_used_gb: stats.mem_used_gb,
            mem_total_gb: stats.mem_total_gb,
            disk: stats.disk,
            networks: stats.networks,
            components: stats.components,
            processes: stats.processes,
            polling_rate: stats.polling_rate,
        }
    }
}
