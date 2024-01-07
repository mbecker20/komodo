use std::path::PathBuf;

use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

use super::{unix_from_monitor_ts, PermissionsMap, Timelength};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: String,

  pub name: String,

  #[serde(default)]
  pub description: String,

  pub address: String,

  #[serde(default)]
  pub permissions: PermissionsMap,

  pub enabled: bool,

  #[serde(default)]
  pub to_notify: Vec<String>, // slack users to notify

  #[serde(default)]
  pub auto_prune: bool,

  #[serde(default = "default_cpu_alert")]
  pub cpu_alert: f32,

  #[serde(default = "default_mem_alert")]
  pub mem_alert: f64,

  #[serde(default = "default_disk_alert")]
  pub disk_alert: f64,

  #[serde(default)]
  pub stats_interval: Timelength,

  pub region: Option<String>,

  pub instance_id: Option<String>,

  #[serde(default, skip_serializing_if = "String::is_empty")]
  pub created_at: String,
  #[serde(default)]
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
      auto_prune: true,
      to_notify: Default::default(),
      cpu_alert: default_cpu_alert(),
      mem_alert: default_mem_alert(),
      disk_alert: default_disk_alert(),
      stats_interval: Default::default(),
      region: Default::default(),
      instance_id: Default::default(),
      description: Default::default(),
      created_at: Default::default(),
      updated_at: Default::default(),
    }
  }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerWithStatus {
  pub server: Server,
  pub status: ServerStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServerActionState {
  pub pruning_networks: bool,
  pub pruning_containers: bool,
  pub pruning_images: bool,
}

#[derive(
  Serialize,
  Deserialize,
  Debug,
  PartialEq,
  Hash,
  Eq,
  Clone,
  Copy,
  Default,
)]
#[serde(rename_all = "snake_case")]
pub enum ServerStatus {
  Ok,
  #[default]
  NotOk,
  Disabled,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct SystemStatsQuery {
  #[serde(default)]
  pub cpus: bool,
  #[serde(default)]
  pub disks: bool,
  #[serde(default)]
  pub networks: bool,
  #[serde(default)]
  pub components: bool,
  #[serde(default)]
  pub processes: bool,
}

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleCpuUsage {
  pub name: String,
  pub usage: f32,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DiskUsage {
  pub used_gb: f64,  // in GB
  pub total_gb: f64, // in GB
  pub read_kb: f64,  // in kB
  pub write_kb: f64, // in kB
  #[serde(default)]
  pub disks: Vec<SingleDiskUsage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleDiskUsage {
  pub mount: PathBuf,
  pub used_gb: f64,  // in GB
  pub total_gb: f64, // in GB
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemNetwork {
  pub name: String,
  pub recieved_kb: f64,    // in kB
  pub transmitted_kb: f64, // in kB
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemComponent {
  pub label: String,
  pub temp: f32,
  pub max: f32,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub critical: Option<f32>,
}

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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SystemStatsRecord {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: String,
  pub server_id: String,
  pub ts: f64, // unix ts milliseconds
  #[serde(default)]
  pub system_load: f64,
  pub cpu_perc: f32, // in %
  #[serde(default)]
  pub cpu_freq_mhz: f64, // in MHz
  pub mem_used_gb: f64, // in GB
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
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoricalStatsQuery {
  #[serde(default = "default_interval")]
  pub interval: Timelength,
  #[serde(default = "default_limit")]
  pub limit: f64,
  #[serde(default)]
  pub page: f64,
  #[serde(default)]
  pub networks: bool,
  #[serde(default)]
  pub components: bool,
}

impl Default for HistoricalStatsQuery {
  fn default() -> Self {
    HistoricalStatsQuery {
      interval: default_interval(),
      limit: default_limit(),
      page: Default::default(),
      networks: Default::default(),
      components: Default::default(),
    }
  }
}

fn default_interval() -> Timelength {
  Timelength::OneHour
}

fn default_limit() -> f64 {
  100.0
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemInformation {
  pub name: Option<String>,
  pub os: Option<String>,
  pub kernel: Option<String>,
  pub core_count: Option<u32>,
  pub host_name: Option<String>,
  pub cpu_brand: String,
}

impl TryFrom<Server> for monitor_client::entities::server::Server {
  type Error = anyhow::Error;
  fn try_from(value: Server) -> Result<Self, Self::Error> {
    let server = Self {
      id: value.id,
      name: value.name,
      description: value.description,
      permissions: value
        .permissions
        .into_iter()
        .map(|(id, p)| (id, p.into()))
        .collect(),
      updated_at: unix_from_monitor_ts(&value.updated_at)?,
      tags: Vec::new(),
      info: (),
      config: monitor_client::entities::server::ServerConfig {
        address: value.address,
        enabled: value.enabled,
        auto_prune: value.auto_prune,
        send_unreachable_alerts: true,
        send_cpu_alerts: true,
        send_mem_alerts: true,
        send_disk_alerts: true,
        send_temp_alerts: true,
        region: value.region.unwrap_or_default(),
        cpu_warning: value.cpu_alert,
        cpu_critical: value.cpu_alert,
        mem_warning: value.mem_alert,
        mem_critical: value.mem_alert,
        disk_warning: value.disk_alert,
        disk_critical: value.disk_alert,
      },
    };
    Ok(server)
  }
}
