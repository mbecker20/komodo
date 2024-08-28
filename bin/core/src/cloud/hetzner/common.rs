use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerServerResponse {
  pub server: HetznerServer,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerServer {
  pub id: i64,
  pub name: String,
  pub primary_disk_size: f64,
  pub image: Option<HetznerImage>,
  pub private_net: Vec<HetznerPrivateNet>,
  pub public_net: HetznerPublicNet,
  pub server_type: HetznerServerTypeDetails,
  pub status: HetznerServerStatus,
  #[serde(default)]
  pub volumes: Vec<i64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerServerTypeDetails {
  pub architecture: String,
  pub cores: i64,
  pub cpu_type: String,
  pub description: String,
  pub disk: f64,
  pub id: i64,
  pub memory: f64,
  pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerPrivateNet {
  pub alias_ips: Vec<String>,
  pub ip: String,
  pub mac_address: String,
  pub network: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerPublicNet {
  #[serde(default)]
  pub firewalls: Vec<HetznerFirewall>,
  pub floating_ips: Vec<i64>,
  pub ipv4: Option<HetznerIpv4>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerFirewall {
  pub id: i64,
  pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerIpv4 {
  pub id: Option<i64>,
  pub blocked: bool,
  pub dns_ptr: String,
  pub ip: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerImage {
  pub id: i64,
  pub description: String,
  pub name: Option<String>,
  pub os_flavor: String,
  pub os_version: Option<String>,
  pub rapid_deploy: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerActionResponse {
  pub action: HetznerAction,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerAction {
  pub command: String,
  pub error: Option<HetznerError>,
  pub finished: Option<String>,
  pub id: i64,
  pub progress: i32,
  pub resources: Vec<HetznerResource>,
  pub started: String,
  pub status: HetznerActionStatus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerError {
  pub code: String,
  pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerResource {
  pub id: i64,
  #[serde(rename = "type")]
  pub ty: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerVolumeResponse {
  pub volume: HetznerVolume,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerVolume {
  /// Name of the Resource. Must be unique per Project.
  pub name: String,
  /// Point in time when the Resource was created (in ISO-8601 format).
  pub created: String,
  /// Filesystem of the Volume if formatted on creation, null if not formatted on creation
  pub format: Option<HetznerVolumeFormat>,
  /// ID of the Volume.
  pub id: i64,
  /// User-defined labels ( key/value pairs) for the Resource
  pub labels: HashMap<String, String>,
  /// Device path on the file system for the Volume
  pub linux_device: String,
  /// Protection configuration for the Resource.
  pub protection: HetznerProtection,
  /// ID of the Server the Volume is attached to, null if it is not attached at all
  pub server: Option<i64>,
  /// Size in GB of the  Volume
  pub size: i64,
  /// Current status of the Volume. Allowed: `creating`, `available`
  pub status: HetznerVolumeStatus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerProtection {
  /// Prevent the Resource from being deleted.
  pub delete: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerDatacenterResponse {
  pub datacenters: Vec<HetznerDatacenterDetails>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HetznerDatacenterDetails {
  pub id: i64,
  pub name: String,
  pub location: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HetznerLocation {
  #[serde(rename = "nbg1")]
  Nuremberg1,
  #[serde(rename = "hel1")]
  Helsinki1,
  #[serde(rename = "fsn1")]
  Falkenstein1,
  #[serde(rename = "ash")]
  Ashburn,
  #[serde(rename = "hil")]
  Hillsboro,
  #[serde(rename = "sin")]
  Singapore,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum HetznerDatacenter {
  #[serde(rename = "nbg1-dc3")]
  Nuremberg1Dc3,
  #[serde(rename = "hel1-dc2")]
  Helsinki1Dc2,
  #[serde(rename = "fsn1-dc14")]
  Falkenstein1Dc14,
  #[serde(rename = "ash-dc1")]
  AshburnDc1,
  #[serde(rename = "hil-dc1")]
  HillsboroDc1,
  #[serde(rename = "sin-dc1")]
  SingaporeDc1,
}

impl From<HetznerDatacenter> for HetznerLocation {
  fn from(value: HetznerDatacenter) -> Self {
    match value {
      HetznerDatacenter::Nuremberg1Dc3 => HetznerLocation::Nuremberg1,
      HetznerDatacenter::Helsinki1Dc2 => HetznerLocation::Helsinki1,
      HetznerDatacenter::Falkenstein1Dc14 => {
        HetznerLocation::Falkenstein1
      }
      HetznerDatacenter::AshburnDc1 => HetznerLocation::Ashburn,
      HetznerDatacenter::HillsboroDc1 => HetznerLocation::Hillsboro,
      HetznerDatacenter::SingaporeDc1 => HetznerLocation::Singapore,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HetznerVolumeFormat {
  Xfs,
  Ext4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HetznerVolumeStatus {
  Creating,
  Available,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HetznerServerStatus {
  Running,
  Initializing,
  Starting,
  Stopping,
  Off,
  Deleting,
  Migrating,
  Rebuilding,
  Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HetznerActionStatus {
  Running,
  Success,
  Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[allow(clippy::enum_variant_names)]
pub enum HetznerServerType {
  // Shared
  #[serde(rename = "cpx11")]
  SharedAmd2Core2Ram40Disk,
  #[serde(rename = "cax11")]
  SharedArm2Core4Ram40Disk,
  #[serde(rename = "cx22")]
  SharedIntel2Core4Ram40Disk,
  #[serde(rename = "cpx21")]
  SharedAmd3Core4Ram80Disk,
  #[serde(rename = "cax21")]
  SharedArm4Core8Ram80Disk,
  #[serde(rename = "cx32")]
  SharedIntel4Core8Ram80Disk,
  #[serde(rename = "cpx31")]
  SharedAmd4Core8Ram160Disk,
  #[serde(rename = "cax31")]
  SharedArm8Core16Ram160Disk,
  #[serde(rename = "cx42")]
  SharedIntel8Core16Ram160Disk,
  #[serde(rename = "cpx41")]
  SharedAmd8Core16Ram240Disk,
  #[serde(rename = "cax41")]
  SharedArm16Core32Ram320Disk,
  #[serde(rename = "cx52")]
  SharedIntel16Core32Ram320Disk,
  #[serde(rename = "cpx51")]
  SharedAmd16Core32Ram360Disk,
  // Dedicated
  #[serde(rename = "ccx13")]
  DedicatedAmd2Core8Ram80Disk,
  #[serde(rename = "ccx23")]
  DedicatedAmd4Core16Ram160Disk,
  #[serde(rename = "ccx33")]
  DedicatedAmd8Core32Ram240Disk,
  #[serde(rename = "ccx43")]
  DedicatedAmd16Core64Ram360Disk,
  #[serde(rename = "ccx53")]
  DedicatedAmd32Core128Ram600Disk,
  #[serde(rename = "ccx63")]
  DedicatedAmd48Core192Ram960Disk,
}
