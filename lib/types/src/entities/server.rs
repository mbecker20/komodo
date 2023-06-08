use std::path::PathBuf;

use serde::{Serialize, Deserialize};
use typeshare::typeshare;

use crate::Timelength;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemInformation {
    pub name: Option<String>,
    pub os: Option<String>,
    pub kernel: Option<String>,
    pub core_count: Option<u32>,
    pub host_name: Option<String>,
    pub cpu_brand: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SystemStats {
    #[serde(default)]
    pub system_load: f64,
    pub cpu_perc: f32,
    pub cpu_freq_mhz: f64,
    pub mem_used_gb: f64,  // in GB
    pub mem_total_gb: f64, // in GB
    pub disk: DiskUsage,
    #[serde(default)]
    pub cpus: Vec<SingleCpuUsage>,
    #[serde(default)]
    pub networks: Vec<SystemNetwork>,
    #[serde(default)]
    pub components: Vec<SystemComponent>,
    #[serde(default)]
    pub processes: Vec<SystemProcess>,
    pub polling_rate: Timelength,
    pub refresh_ts: u128,
    pub refresh_list_ts: u128,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemProcess {
    pub pid: u32,
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub exe: String,
    pub cmd: Vec<String>,
    #[serde(default)]
    pub start_time: f64,
    pub cpu_perc: f32,
    pub mem_mb: f64,
    pub disk_read_kb: f64,
    pub disk_write_kb: f64,
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
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DiskUsage {
    pub used_gb: f64,  // in GB
    pub total_gb: f64, // in GB
    pub read_kb: f64,  // in kB
    pub write_kb: f64, // in kB
    #[serde(default)]
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
pub struct SingleCpuUsage {
    pub name: String,
    pub usage: f32,
}