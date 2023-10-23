use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

use super::{
  Build, Deployment, Group, Operation, Procedure, Server, Version,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Update {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: String,
  pub target: UpdateTarget,
  pub operation: Operation,
  pub logs: Vec<Log>,
  pub start_ts: String,
  pub end_ts: Option<String>,
  pub status: UpdateStatus,
  pub success: bool,
  pub operator: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub version: Option<Version>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Log {
  pub stage: String,
  pub command: String,
  pub stdout: String,
  pub stderr: String,
  pub success: bool,
  pub start_ts: String,
  pub end_ts: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(tag = "type", content = "id")]
pub enum UpdateTarget {
  #[default]
  System,
  Build(String),
  Deployment(String),
  Server(String),
  Procedure(String),
  Group(String),
  Command(String),
}

impl From<&Build> for UpdateTarget {
  fn from(build: &Build) -> Self {
    Self::Build(build.id.clone())
  }
}

impl From<&Build> for Option<UpdateTarget> {
  fn from(build: &Build) -> Self {
    Some(UpdateTarget::Build(build.id.clone()))
  }
}

impl From<&Deployment> for UpdateTarget {
  fn from(deployment: &Deployment) -> Self {
    Self::Deployment(deployment.id.clone())
  }
}

impl From<&Deployment> for Option<UpdateTarget> {
  fn from(deployment: &Deployment) -> Self {
    Some(UpdateTarget::Deployment(deployment.id.clone()))
  }
}

impl From<&Server> for UpdateTarget {
  fn from(server: &Server) -> Self {
    Self::Server(server.id.clone())
  }
}

impl From<&Server> for Option<UpdateTarget> {
  fn from(server: &Server) -> Self {
    Some(UpdateTarget::Server(server.id.clone()))
  }
}

impl From<&Procedure> for UpdateTarget {
  fn from(procedure: &Procedure) -> Self {
    Self::Procedure(procedure.id.clone())
  }
}

impl From<&Procedure> for Option<UpdateTarget> {
  fn from(procedure: &Procedure) -> Self {
    Some(UpdateTarget::Procedure(procedure.id.clone()))
  }
}

impl From<&Group> for UpdateTarget {
  fn from(group: &Group) -> Self {
    Self::Group(group.id.clone())
  }
}

impl From<&Group> for Option<UpdateTarget> {
  fn from(group: &Group) -> Self {
    Some(UpdateTarget::Group(group.id.clone()))
  }
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
pub enum UpdateStatus {
  Queued,
  InProgress,
  #[default]
  Complete,
}
