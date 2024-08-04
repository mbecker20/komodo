use async_timing_util::unix_timestamp_ms;
use derive_variants::{EnumVariants, ExtractVariant};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use typeshare::typeshare;

use crate::entities::{
  all_logs_success, monitor_timestamp, MongoId, Operation, I64,
};

use super::{
  alerter::Alerter, build::Build, builder::Builder,
  deployment::Deployment, procedure::Procedure, repo::Repo,
  server::Server, server_template::ServerTemplate, stack::Stack,
  sync::ResourceSync, Version,
};

/// Represents an action performed by Monitor.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(
  feature = "mongo",
  derive(mongo_indexed::derive::MongoIndexed)
)]
#[cfg_attr(feature = "mongo", doc_index({ "target.type": 1 }))]
#[cfg_attr(feature = "mongo", sparse_doc_index({ "target.id": 1 }))]
pub struct Update {
  /// The Mongo ID of the update.
  /// This field is de/serialized from/to JSON as
  /// `{ "_id": { "$oid": "..." }, ...(rest of serialized Update) }`
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "bson::serde_helpers::hex_string_as_object_id"
  )]
  pub id: MongoId,

  /// The operation performed
  #[cfg_attr(feature = "mongo", index)]
  pub operation: Operation,

  /// The time the operation started
  #[cfg_attr(feature = "mongo", index)]
  pub start_ts: I64,

  /// Whether the operation was successful
  #[cfg_attr(feature = "mongo", index)]
  pub success: bool,

  /// The user id that triggered the update.
  ///
  /// Also can take these values for operations triggered automatically:
  /// - `Procedure`: The operation was triggered as part of a procedure run
  /// - `Github`: The operation was triggered by a github webhook
  /// - `Auto Redeploy`: The operation (always `Deploy`) was triggered by an attached build finishing.
  #[cfg_attr(feature = "mongo", index)]
  pub operator: String,

  /// The target resource to which this update refers
  pub target: ResourceTarget,
  /// Logs produced as the operation is performed
  pub logs: Vec<Log>,
  /// The time the operation completed.
  pub end_ts: Option<I64>,
  /// The status of the update
  /// - `Queued`
  /// - `InProgress`
  /// - `Complete`
  pub status: UpdateStatus,
  /// An optional version on the update, ie build version or deployed version.
  #[serde(default, skip_serializing_if = "Version::is_none")]
  pub version: Version,
  /// Some unstructured, operation specific data. Not for general usage.
  #[serde(default, skip_serializing_if = "String::is_empty")]
  pub other_data: String,
}

impl Update {
  pub fn push_simple_log(
    &mut self,
    stage: &str,
    msg: impl Into<String>,
  ) {
    self.logs.push(Log::simple(stage, msg.into()));
  }

  pub fn push_error_log(
    &mut self,
    stage: &str,
    msg: impl Into<String>,
  ) {
    self.logs.push(Log::error(stage, msg.into()));
  }

  pub fn in_progress(&mut self) {
    self.status = UpdateStatus::InProgress;
  }

  pub fn finalize(&mut self) {
    self.success = all_logs_success(&self.logs);
    self.end_ts = Some(monitor_timestamp());
    self.status = UpdateStatus::Complete;
  }
}

/// Minimal representation of an action performed by Monitor.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateListItem {
  /// The id of the update
  pub id: String,
  /// Which operation was run
  pub operation: Operation,
  /// The starting time of the operation
  pub start_ts: I64,
  /// Whether the operation was successful
  pub success: bool,
  /// The username of the user performing update
  pub username: String,
  /// The user id that triggered the update.
  ///
  /// Also can take these values for operations triggered automatically:
  /// - `Procedure`: The operation was triggered as part of a procedure run
  /// - `Github`: The operation was triggered by a github webhook
  /// - `Auto Redeploy`: The operation (always `Deploy`) was triggered by an attached build finishing.
  pub operator: String,
  /// The target resource to which this update refers
  pub target: ResourceTarget,
  /// The status of the update
  /// - `Queued`
  /// - `InProgress`
  /// - `Complete`
  pub status: UpdateStatus,
  /// An optional version on the update, ie build version or deployed version.
  #[serde(default, skip_serializing_if = "Version::is_none")]
  pub version: Version,
  /// Some unstructured, operation specific data. Not for general usage.
  #[serde(default, skip_serializing_if = "String::is_empty")]
  pub other_data: String,
}

/// Represents the output of some command being run
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Log {
  /// A label for the log
  pub stage: String,
  /// The command which was executed
  pub command: String,
  /// The output of the command in the standard channel
  pub stdout: String,
  /// The output of the command in the error channel
  pub stderr: String,
  /// Whether the command run was successful
  pub success: bool,
  /// The start time of the command execution
  pub start_ts: I64,
  /// The end time of the command execution
  pub end_ts: I64,
}

impl Log {
  pub fn simple(stage: &str, msg: String) -> Log {
    let ts = unix_timestamp_ms() as i64;
    Log {
      stage: stage.to_string(),
      stdout: msg,
      success: true,
      start_ts: ts,
      end_ts: ts,
      ..Default::default()
    }
  }

  pub fn error(stage: &str, msg: String) -> Log {
    let ts = unix_timestamp_ms() as i64;
    Log {
      stage: stage.to_string(),
      stderr: msg,
      start_ts: ts,
      end_ts: ts,
      success: false,
      ..Default::default()
    }
  }
}

/// Used to reference a specific resource across all resource types
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
  EnumVariants,
)]
#[variant_derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Hash,
  Serialize,
  Deserialize,
  Display,
  EnumString,
  AsRefStr
)]
#[serde(tag = "type", content = "id")]
pub enum ResourceTarget {
  System(String),
  Build(String),
  Builder(String),
  Deployment(String),
  Server(String),
  Repo(String),
  Alerter(String),
  Procedure(String),
  ServerTemplate(String),
  ResourceSync(String),
  Stack(String),
}

impl ResourceTarget {
  pub fn extract_variant_id(
    &self,
  ) -> (ResourceTargetVariant, &String) {
    let id = match &self {
      ResourceTarget::System(id) => id,
      ResourceTarget::Build(id) => id,
      ResourceTarget::Builder(id) => id,
      ResourceTarget::Deployment(id) => id,
      ResourceTarget::Server(id) => id,
      ResourceTarget::Repo(id) => id,
      ResourceTarget::Alerter(id) => id,
      ResourceTarget::Procedure(id) => id,
      ResourceTarget::ServerTemplate(id) => id,
      ResourceTarget::ResourceSync(id) => id,
      ResourceTarget::Stack(id) => id,
    };
    (self.extract_variant(), id)
  }

  pub fn system() -> ResourceTarget {
    Self::System("system".to_string())
  }
}

impl Default for ResourceTarget {
  fn default() -> Self {
    ResourceTarget::system()
  }
}

impl From<&Build> for ResourceTarget {
  fn from(build: &Build) -> Self {
    Self::Build(build.id.clone())
  }
}

impl From<&Deployment> for ResourceTarget {
  fn from(deployment: &Deployment) -> Self {
    Self::Deployment(deployment.id.clone())
  }
}

impl From<&Server> for ResourceTarget {
  fn from(server: &Server) -> Self {
    Self::Server(server.id.clone())
  }
}

impl From<&Repo> for ResourceTarget {
  fn from(repo: &Repo) -> Self {
    Self::Repo(repo.id.clone())
  }
}

impl From<&Builder> for ResourceTarget {
  fn from(builder: &Builder) -> Self {
    Self::Builder(builder.id.clone())
  }
}

impl From<&Alerter> for ResourceTarget {
  fn from(alerter: &Alerter) -> Self {
    Self::Alerter(alerter.id.clone())
  }
}

impl From<&Procedure> for ResourceTarget {
  fn from(procedure: &Procedure) -> Self {
    Self::Procedure(procedure.id.clone())
  }
}

impl From<&ServerTemplate> for ResourceTarget {
  fn from(server_template: &ServerTemplate) -> Self {
    Self::ServerTemplate(server_template.id.clone())
  }
}

impl From<&ResourceSync> for ResourceTarget {
  fn from(resource_sync: &ResourceSync) -> Self {
    Self::ResourceSync(resource_sync.id.clone())
  }
}

impl From<&Stack> for ResourceTarget {
  fn from(resource_sync: &Stack) -> Self {
    Self::Stack(resource_sync.id.clone())
  }
}

/// An update's status
#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Display,
  EnumString,
  PartialEq,
  Hash,
  Eq,
  Clone,
  Copy,
  Default,
)]
pub enum UpdateStatus {
  /// The run is in the system but hasn't started yet
  Queued,
  /// The run is currently running
  InProgress,
  /// The run is complete
  #[default]
  Complete,
}
