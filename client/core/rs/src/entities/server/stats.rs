use std::{collections::HashMap, path::PathBuf};

use mongo_indexed::derive::MongoIndexed;
use mungos::mongodb::bson::Document;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use typeshare::typeshare;

use crate::entities::{Timelength, I64};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SystemInformation {
  pub name: Option<String>,
  pub os: Option<String>,
  pub kernel: Option<String>,
  pub core_count: Option<u32>,
  pub host_name: Option<String>,
  pub cpu_brand: String,
}

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, MongoIndexed,
)]
#[collection_name(Stats)]
pub struct SystemStatsRecord {
  #[index]
  pub ts: I64,
  #[index]
  pub sid: String,
  // basic stats
  pub cpu_perc: f32,
  pub mem_used_gb: f64,
  pub mem_total_gb: f64,
  pub disk_used_gb: f64,
  pub disk_total_gb: f64,
  pub disks: Vec<SingleDiskUsage>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SystemStats {
  pub cpu_perc: f32,
  pub mem_used_gb: f64,
  pub mem_total_gb: f64,
  pub disks: Vec<SingleDiskUsage>,

  // metadata
  pub polling_rate: Timelength,
  pub refresh_ts: I64,
  pub refresh_list_ts: I64,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SingleDiskUsage {
  pub mount: PathBuf,
  pub used_gb: f64,  // in GB
  pub total_gb: f64, // in GB
}

pub fn sum_disk_usage(disks: &[SingleDiskUsage]) -> TotalDiskUsage {
  disks
    .iter()
    .fold(TotalDiskUsage::default(), |mut total, disk| {
      total.used_gb += disk.used_gb;
      total.total_gb += disk.total_gb;
      total
    })
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct TotalDiskUsage {
  pub used_gb: f64,  // in GB
  pub total_gb: f64, // in GB
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SystemProcess {
  pub pid: u32,
  pub name: String,
  #[serde(default)]
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
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ServerHealth {
  pub cpu: SeverityLevel,
  pub mem: SeverityLevel,
  pub disks: HashMap<PathBuf, SeverityLevel>,
}

#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  Default,
  Display,
  EnumString,
  PartialEq,
  Eq,
)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
pub enum SeverityLevel {
  #[default]
  Ok,
  Warning,
  Critical,
}
