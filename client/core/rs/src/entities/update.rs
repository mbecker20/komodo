use async_timing_util::unix_timestamp_ms;
use derive_variants::EnumVariants;
use mongo_indexed::derive::MongoIndexed;
use mungos::mongodb::bson::{
  doc, serde_helpers::hex_string_as_object_id,
};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use typeshare::typeshare;

use crate::entities::{
  all_logs_success, monitor_timestamp, MongoId, Operation, I64,
};

use super::{
  alerter::Alerter, build::Build, builder::Builder,
  deployment::Deployment, procedure::Procedure, repo::Repo,
  server::Server, server_template::ServerTemplate, Version,
};

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, MongoIndexed,
)]
#[doc_index({ "target.type": 1 })]
#[sparse_doc_index({ "target.id": 1 })]
pub struct Update {
  /// The Mongo ID of the update.
  /// This field is de/serialized from/to JSON as
  /// `{ "_id": { "$oid": "..." }, ...(rest of serialized Update) }`
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: MongoId,

  #[index]
  pub operation: Operation,

  #[index]
  pub start_ts: I64,

  #[index]
  pub success: bool,

  #[index]
  pub operator: String,

  pub target: ResourceTarget,
  pub logs: Vec<Log>,
  pub end_ts: Option<I64>,
  pub status: UpdateStatus,
  pub version: Version,
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

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateListItem {
  pub id: String,
  pub operation: Operation,
  pub start_ts: I64,
  pub success: bool,
  pub username: String,
  pub operator: String,
  pub target: ResourceTarget,
  pub status: UpdateStatus,
  pub version: Version,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Log {
  pub stage: String,
  pub command: String,
  pub stdout: String,
  pub stderr: String,
  pub success: bool,
  pub start_ts: I64,
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

#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  PartialEq,
  Eq,
  Hash,
  EnumVariants,
)]
#[variant_derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
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
}

impl ResourceTarget {
  pub fn extract_variant_id(
    &self,
  ) -> (ResourceTargetVariant, &String) {
    let variant: ResourceTargetVariant = self.into();
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
    };
    (variant, id)
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
  Queued,
  InProgress,
  #[default]
  Complete,
}
