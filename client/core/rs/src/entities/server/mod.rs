use derive_builder::Builder;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::resource::{
  AddFilters, Resource, ResourceListItem, ResourceQuery,
};

pub mod docker_image;
pub mod docker_network;
pub mod stats;

#[typeshare]
pub type Server = Resource<ServerConfig, ()>;

#[typeshare]
pub type ServerListItem = ResourceListItem<ServerListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServerListItemInfo {
  /// The server's status.
  pub status: ServerStatus,
  /// Region of the server.
  pub region: String,
  /// Whether server is configured to send unreachable alerts.
  pub send_unreachable_alerts: bool,
  /// Whether server is configured to send cpu alerts.
  pub send_cpu_alerts: bool,
  /// Whether server is configured to send mem alerts.
  pub send_mem_alerts: bool,
  /// Whether server is configured to send disk alerts.
  pub send_disk_alerts: bool,
}

#[typeshare(serialized_as = "Partial<ServerConfig>")]
pub type _PartialServerConfig = PartialServerConfig;

/// Server configuration.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[skip_serializing_none]
#[partial_from]
pub struct ServerConfig {
  /// The http address of the periphery client.
  /// Example: http://localhost:8120
  pub address: String,

  /// Whether a server is enabled.
  /// If a server is disabled,
  /// you won't be able to perform any actions on it or see deployment's status.
  /// default: true
  #[serde(default = "default_enabled")]
  #[builder(default = "default_enabled()")]
  #[partial_default(default_enabled())]
  pub enabled: bool,

  /// Whether to monitor any server stats beyond passing health check.
  /// default: true
  #[serde(default = "default_stats_monitoring")]
  #[builder(default = "default_stats_monitoring()")]
  #[partial_default(default_stats_monitoring())]
  pub stats_monitoring: bool,

  /// Whether to trigger 'docker image prune -a -f' every 24 hours.
  /// default: true
  #[serde(default = "default_auto_prune")]
  #[builder(default = "default_auto_prune()")]
  #[partial_default(default_auto_prune())]
  pub auto_prune: bool,

  /// Whether to send alerts about the servers reachability
  #[serde(default = "default_send_alerts")]
  #[builder(default = "default_send_alerts()")]
  #[partial_default(default_send_alerts())]
  pub send_unreachable_alerts: bool,

  /// Whether to send alerts about the servers CPU status
  #[serde(default = "default_send_alerts")]
  #[builder(default = "default_send_alerts()")]
  #[partial_default(default_send_alerts())]
  pub send_cpu_alerts: bool,

  /// Whether to send alerts about the servers MEM status
  #[serde(default = "default_send_alerts")]
  #[builder(default = "default_send_alerts()")]
  #[partial_default(default_send_alerts())]
  pub send_mem_alerts: bool,

  /// Whether to send alerts about the servers DISK status
  #[serde(default = "default_send_alerts")]
  #[builder(default = "default_send_alerts()")]
  #[partial_default(default_send_alerts())]
  pub send_disk_alerts: bool,

  /// An optional region label
  #[serde(default)]
  #[builder(default)]
  pub region: String,

  /// The percentage threshhold which triggers WARNING state for CPU.
  #[serde(default = "default_cpu_warning")]
  #[builder(default = "default_cpu_warning()")]
  #[partial_default(default_cpu_warning())]
  pub cpu_warning: f32,

  /// The percentage threshhold which triggers CRITICAL state for CPU.
  #[serde(default = "default_cpu_critical")]
  #[builder(default = "default_cpu_critical()")]
  #[partial_default(default_cpu_critical())]
  pub cpu_critical: f32,

  /// The percentage threshhold which triggers WARNING state for MEM.
  #[serde(default = "default_mem_warning")]
  #[builder(default = "default_mem_warning()")]
  #[partial_default(default_mem_warning())]
  pub mem_warning: f64,

  /// The percentage threshhold which triggers CRITICAL state for MEM.
  #[serde(default = "default_mem_critical")]
  #[builder(default = "default_mem_critical()")]
  #[partial_default(default_mem_critical())]
  pub mem_critical: f64,

  /// The percentage threshhold which triggers WARNING state for DISK.
  #[serde(default = "default_disk_warning")]
  #[builder(default = "default_disk_warning()")]
  #[partial_default(default_disk_warning())]
  pub disk_warning: f64,

  /// The percentage threshhold which triggers CRITICAL state for DISK.
  #[serde(default = "default_disk_critical")]
  #[builder(default = "default_disk_critical()")]
  #[partial_default(default_disk_critical())]
  pub disk_critical: f64,
}

impl ServerConfig {
  pub fn builder() -> ServerConfigBuilder {
    ServerConfigBuilder::default()
  }
}

fn default_enabled() -> bool {
  true
}

fn default_stats_monitoring() -> bool {
  true
}

fn default_auto_prune() -> bool {
  true
}

fn default_send_alerts() -> bool {
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

/// Current pending actions on the server.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServerActionState {
  /// Server currently pruning networks
  pub pruning_networks: bool,
  /// Server currently pruning containers
  pub pruning_containers: bool,
  /// Server currently pruning images
  pub pruning_images: bool,
  /// Server currently stopping all containers.
  pub stopping_containers: bool,
}

#[typeshare]
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
pub enum ServerStatus {
  /// Server is unreachable.
  #[default]
  NotOk,
  /// Server health check passing.
  Ok,
  /// Server is disabled.
  Disabled,
}

/// Server-specific query
#[typeshare]
pub type ServerQuery = ResourceQuery<ServerQuerySpecifics>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ServerQuerySpecifics {}

impl AddFilters for ServerQuerySpecifics {}
