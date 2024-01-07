use monitor_client::entities::update::ResourceTarget;
use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

use super::{
  unix_from_monitor_ts, Build, Deployment, Group, Operation,
  Procedure, Server, Version,
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

impl TryFrom<Update> for monitor_client::entities::update::Update {
  type Error = anyhow::Error;
  fn try_from(value: Update) -> Result<Self, Self::Error> {
    let target: Option<ResourceTarget> = value.target.into();
    let update = Self {
      id: value.id,
      operation: value.operation.into(),
      start_ts: unix_from_monitor_ts(&value.start_ts)?,
      success: value.success,
      operator: value.operator,
      target: target.unwrap_or_default(),
      logs: value
        .logs
        .into_iter()
        .map(|log| log.try_into())
        .collect::<anyhow::Result<
        Vec<monitor_client::entities::update::Log>,
      >>()?,
      end_ts: value
        .end_ts
        .and_then(|ts| unix_from_monitor_ts(&ts).ok()),
      status: value.status.into(),
      version: value.version.map(|v| v.into()).unwrap_or_default(),
    };
    Ok(update)
  }
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

impl TryFrom<Log> for monitor_client::entities::update::Log {
  type Error = anyhow::Error;
  fn try_from(value: Log) -> Result<Self, Self::Error> {
    Ok(Self {
      stage: value.stage,
      command: value.command,
      stdout: value.stdout,
      stderr: value.stderr,
      success: value.success,
      start_ts: unix_from_monitor_ts(&value.start_ts)?,
      end_ts: unix_from_monitor_ts(&value.end_ts)?,
    })
  }
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

impl From<UpdateTarget>
  for Option<monitor_client::entities::update::ResourceTarget>
{
  fn from(value: UpdateTarget) -> Self {
    use monitor_client::entities::update::ResourceTarget::*;
    match value {
      UpdateTarget::System => Some(System("system".to_string())),
      UpdateTarget::Build(id) => Some(Build(id)),
      UpdateTarget::Deployment(id) => Some(Deployment(id)),
      UpdateTarget::Server(id) => Some(Server(id)),
      UpdateTarget::Procedure(_) => None,
      UpdateTarget::Group(_) => None,
      UpdateTarget::Command(id) => None,
    }
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

impl From<UpdateStatus>
  for monitor_client::entities::update::UpdateStatus
{
  fn from(value: UpdateStatus) -> Self {
    use monitor_client::entities::update::UpdateStatus::*;
    match value {
      UpdateStatus::Queued => Queued,
      UpdateStatus::InProgress => InProgress,
      UpdateStatus::Complete => Complete,
    }
  }
}
