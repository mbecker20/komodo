use std::path::PathBuf;

use derive_variants::EnumVariants;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{MongoId, I64};

use super::{
  _Serror, deployment::DeploymentState, server::stats::SeverityLevel,
  update::ResourceTarget,
};

/// Representation of an alert in the system.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(
  feature = "mongo",
  derive(mongo_indexed::derive::MongoIndexed)
)]
#[cfg_attr(feature = "mongo", doc_index({ "data.type": 1 }))]
#[cfg_attr(feature = "mongo", doc_index({ "target.type": 1 }))]
#[cfg_attr(feature = "mongo", doc_index({ "target.id": 1 }))]
pub struct Alert {
  /// The Mongo ID of the alert.
  /// This field is de/serialized from/to JSON as
  /// `{ "_id": { "$oid": "..." }, ...(rest of serialized Alert) }`
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "bson::serde_helpers::hex_string_as_object_id"
  )]
  pub id: MongoId,

  /// Unix timestamp in milliseconds the alert was opened
  #[cfg_attr(feature = "mongo", index)]
  pub ts: I64,

  /// Whether the alert is already resolved
  #[cfg_attr(feature = "mongo", index)]
  pub resolved: bool,

  /// The severity of the alert
  #[cfg_attr(feature = "mongo", index)]
  pub level: SeverityLevel,

  /// The target of the alert
  pub target: ResourceTarget,

  /// The type of alert, eg ServerUnreachable, ServerMem, ContainerStateChange
  pub variant: AlertDataVariant,

  /// The data attached to the alert
  pub data: AlertData,

  /// The timestamp of alert resolution
  pub resolved_ts: Option<I64>,
}

/// The variants of data related to the alert.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, EnumVariants)]
#[variant_derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash
)]
#[serde(tag = "type", content = "data")]
pub enum AlertData {
  /// A server could not be reached.
  ServerUnreachable {
    /// The id of the server
    id: String,
    /// The name of the server
    name: String,
    /// The region of the server
    region: Option<String>,
    /// The error data
    err: Option<_Serror>,
  },

  /// A server has high CPU usage.
  ServerCpu {
    /// The id of the server
    id: String,
    /// The name of the server
    name: String,
    /// The region of the server
    region: Option<String>,
    /// The cpu usage percentage
    percentage: f64,
  },

  /// A server has high memory usage.
  ServerMem {
    /// The id of the server
    id: String,
    /// The name of the server
    name: String,
    /// The region of the server
    region: Option<String>,
    /// The used memory
    used_gb: f64,
    /// The total memory
    total_gb: f64,
  },

  /// A server has high disk usage.
  ServerDisk {
    /// The id of the server
    id: String,
    /// The name of the server
    name: String,
    /// The region of the server
    region: Option<String>,
    /// The mount path of the disk
    path: PathBuf,
    /// The used portion of the disk in GB
    used_gb: f64,
    /// The total size of the disk in GB
    total_gb: f64,
  },

  /// A container's state has changed unexpectedly.
  ContainerStateChange {
    /// The id of the deployment
    id: String,
    /// The name of the deployment
    name: String,
    /// The server id of server deployment is on
    server_id: String,
    /// The server name
    server_name: String,
    /// The previous container state
    from: DeploymentState,
    /// The current container state
    to: DeploymentState,
  },

  /// An AWS builder failed to terminate.
  AwsBuilderTerminationFailed {
    /// The id of the aws instance which failed to terminate
    instance_id: String,
  },
  None {},
}

impl Default for AlertData {
  fn default() -> Self {
    AlertData::None {}
  }
}

#[allow(clippy::derivable_impls)]
impl Default for AlertDataVariant {
  fn default() -> Self {
    AlertDataVariant::None
  }
}
