use std::collections::HashMap;

use derive_builder::Builder;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum::AsRefStr;
use typeshare::typeshare;

use crate::entities::I64;

#[typeshare(serialized_as = "Partial<HetznerServerTemplateConfig>")]
pub type _PartialHetznerServerTemplateConfig =
  PartialHetznerServerTemplateConfig;

/// Hetzner server config.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, Builder, Partial)]
#[partial_derive(Debug, Clone, Default, Serialize, Deserialize)]
#[partial(skip_serializing_none, from, diff)]
pub struct HetznerServerTemplateConfig {
  /// ID or name of the Image the Server is created from
  #[serde(default)]
  #[builder(default)]
  pub image: String,
  /// ID or name of Datacenter to create Server in
  #[serde(default)]
  #[builder(default)]
  pub datacenter: HetznerDatacenter,
  /// Network IDs which should be attached to the Server private network interface at the creation time
  #[serde(default)]
  #[builder(default)]
  pub private_network_ids: Vec<I64>,
  /// ID of the Placement Group the server should be in,
  /// Or 0 to not use placement group.
  #[serde(default)]
  #[builder(default)]
  pub placement_group: I64,
  /// Attach an IPv4 on the public NIC. If false, no IPv4 address will be attached.
  #[serde(default)]
  #[builder(default)]
  pub enable_public_ipv4: bool,
  /// Attach an IPv6 on the public NIC. If false, no IPv6 address will be attached.
  #[serde(default)]
  #[builder(default)]
  pub enable_public_ipv6: bool,
  /// The firewalls to attach to the instance
  #[serde(default)]
  #[builder(default)]
  pub firewall_ids: Vec<I64>,
  /// ID or name of the Server type this Server should be created with
  #[serde(default)]
  #[builder(default)]
  pub server_type: HetznerServerType,
  /// SSH key IDs ( integer ) or names ( string ) which should be injected into the Server at creation time
  #[serde(default)]
  #[builder(default)]
  pub ssh_keys: Vec<String>,
  /// Cloud-Init user data to use during Server creation. This field is limited to 32KiB.
  #[serde(default)]
  #[builder(default)]
  pub user_data: String,
  /// Connect to the instance using it's public ip.
  #[serde(default)]
  #[builder(default)]
  pub use_public_ip: bool,
  /// Labels for the server
  #[serde(default)]
  #[builder(default)]
  pub labels: HashMap<String, String>,
  /// Specs for volumes to attach
  #[serde(default)]
  #[builder(default)]
  pub volumes: Vec<HetznerVolumeSpecs>,
  /// The port periphery will be running on in AMI.
  /// Default: `8120`
  #[serde(default = "default_port")]
  #[builder(default = "default_port()")]
  #[partial_default(default_port())]
  pub port: i32,
}

impl HetznerServerTemplateConfig {
  pub fn builder() -> HetznerServerTemplateConfigBuilder {
    HetznerServerTemplateConfigBuilder::default()
  }
}

fn default_port() -> i32 {
  8120
}

impl Default for HetznerServerTemplateConfig {
  fn default() -> Self {
    Self {
      port: default_port(),
      image: Default::default(),
      datacenter: Default::default(),
      private_network_ids: Default::default(),
      placement_group: Default::default(),
      enable_public_ipv4: Default::default(),
      enable_public_ipv6: Default::default(),
      firewall_ids: Default::default(),
      server_type: Default::default(),
      ssh_keys: Default::default(),
      user_data: Default::default(),
      use_public_ip: Default::default(),
      labels: Default::default(),
      volumes: Default::default(),
    }
  }
}

#[typeshare]
#[derive(
  Debug, Clone, PartialEq, Serialize, Deserialize, Builder,
)]
pub struct HetznerVolumeSpecs {
  /// A name for the volume
  pub name: String,
  /// Size of the volume in GB
  pub size_gb: I64,
  /// The format for the volume
  #[serde(default)]
  #[builder(default)]
  pub format: HetznerVolumeFormat,
  /// Labels for the volume
  #[serde(default)]
  #[builder(default)]
  pub labels: HashMap<String, String>,
}

#[typeshare]
#[derive(
  Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize,
)]
pub enum HetznerVolumeFormat {
  #[default]
  Xfs,
  Ext4,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Serialize,
  Deserialize,
  AsRefStr,
)]
#[allow(clippy::enum_variant_names)]
pub enum HetznerServerType {
  SharedIntel1Core2Ram20Disk,
  // The lowest tier available at all locations
  #[default]
  SharedAmd2Core2Ram40Disk,
  SharedArm2Core4Ram40Disk,
  SharedIntel2Core4Ram40Disk,
  SharedAmd3Core4Ram80Disk,
  SharedArm4Core8Ram80Disk,
  SharedIntel2Core8Ram80Disk,
  SharedAmd4Core8Ram160Disk,
  SharedArm8Core16Ram160Disk,
  SharedIntel4Core16Ram160Disk,
  SharedAmd8Core16Ram240Disk,
  SharedArm16Core32Ram320Disk,
  SharedIntel8Core32Ram240Disk,
  SharedAmd16Core32Ram360Disk,
  DedicatedAmd2Core8Ram80Disk,
  DedicatedAmd4Core16Ram160Disk,
  DedicatedAmd8Core32Ram240Disk,
  DedicatedAmd16Core64Ram360Disk,
  DedicatedAmd32Core128Ram600Disk,
  DedicatedAmd48Core192Ram960Disk,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Serialize,
  Deserialize,
  AsRefStr,
)]
pub enum HetznerDatacenter {
  #[default]
  Nuremberg1Dc3,
  Helsinki1Dc2,
  Falkenstein1Dc14,
  AshburnDc1,
  HillsboroDc1,
}
