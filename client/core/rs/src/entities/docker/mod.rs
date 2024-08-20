use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

pub mod container;
pub mod image;
pub mod network;
pub mod volume;

/// PortBinding represents a binding between a host IP address and a host port.
#[typeshare]
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct PortBinding {
  /// Host IP address that the container's port is mapped to.
  #[serde(rename = "HostIp")]
  pub host_ip: Option<String>,

  /// Host port number that the container's port is mapped to.
  #[serde(rename = "HostPort")]
  pub host_port: Option<String>,
}

/// Information about the storage driver used to store the container's and image's filesystem.
#[typeshare]
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct GraphDriverData {
  /// Name of the storage driver.
  #[serde(default, rename = "Name")]
  pub name: String,
  /// Low-level storage metadata, provided as key/value pairs.  This information is driver-specific, and depends on the storage-driver in use, and should be used for informational purposes only.
  #[serde(default, rename = "Data")]
  pub data: HashMap<String, String>,
}
