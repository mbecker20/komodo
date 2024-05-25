use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::{
  HetznerAction, HetznerLocation, HetznerVolume, HetznerVolumeFormat,
};

#[derive(Debug, Clone, Serialize)]
pub struct CreateVolumeBody {
  /// Name of the volume
  pub name: String,
  /// Auto-mount Volume after attach. server must be provided.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub automount: Option<bool>,
  /// Format Volume after creation. One of: xfs, ext4
  #[serde(skip_serializing_if = "Option::is_none")]
  pub format: Option<HetznerVolumeFormat>,
  /// User-defined labels (key-value pairs) for the Resource
  pub labels: HashMap<String, String>,
  /// Location to create the Volume in (can be omitted if Server is specified)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub location: Option<HetznerLocation>,
  /// Server to which to attach the Volume once it's created (Volume will be created in the same Location as the server)
  #[serde(skip_serializing_if = "Option::is_none")]
  pub server: Option<i64>,
  /// Size of the Volume in GB
  pub size: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateVolumeResponse {
  pub action: HetznerAction,
  pub next_actions: Vec<HetznerAction>,
  pub volume: HetznerVolume,
}
