use std::collections::HashMap;

use anyhow::{anyhow, Context};
use chrono::{DateTime, LocalResult, SecondsFormat, TimeZone, Utc};
use serde::{Deserialize, Serialize};

pub mod traits;

mod action;
mod alert;
mod build;
mod config;
mod deployment;
mod group;
mod periphery_command;
mod procedure;
mod server;
mod update;
mod user;

pub use action::*;
pub use alert::*;
pub use build::*;
pub use config::*;
pub use deployment::*;
pub use group::*;
pub use periphery_command::*;
pub use procedure::*;
pub use server::*;
pub use update::*;
pub use user::*;

pub type PermissionsMap = HashMap<String, PermissionLevel>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CloneArgs {
  pub name: String,
  pub repo: Option<String>,
  pub branch: Option<String>,
  pub on_clone: Option<Command>,
  pub on_pull: Option<Command>,
  pub github_account: Option<GithubUsername>,
}

#[derive(
  Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq,
)]
pub struct Command {
  #[serde(default)]
  pub path: String,
  #[serde(default)]
  pub command: String,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
pub struct EnvironmentVar {
  pub variable: String,
  pub value: String,
}

impl From<EnvironmentVar>
  for monitor_types::entities::EnvironmentVar
{
  fn from(value: EnvironmentVar) -> Self {
    Self {
      variable: value.variable,
      value: value.value,
    }
  }
}

#[derive(Deserialize, Debug)]
pub struct UserCredentials {
  pub username: String,
  pub password: String,
}

#[derive(
  Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone, Copy,
)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
  Github,
  Docker,
}

#[derive(
  Serialize,
  Deserialize,
  Debug,
  Default,
  PartialEq,
  Hash,
  Eq,
  Clone,
  Copy,
)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
  // do nothing
  #[default]
  None,

  // server
  CreateServer,
  UpdateServer,
  DeleteServer,
  PruneImagesServer,
  PruneContainersServer,
  PruneNetworksServer,
  RenameServer,

  // build
  CreateBuild,
  UpdateBuild,
  DeleteBuild,
  BuildBuild,

  // deployment
  CreateDeployment,
  UpdateDeployment,
  DeleteDeployment,
  DeployContainer,
  StopContainer,
  StartContainer,
  RemoveContainer,
  PullDeployment,
  RecloneDeployment,
  RenameDeployment,

  // procedure
  CreateProcedure,
  UpdateProcedure,
  DeleteProcedure,

  // command
  CreateCommand,
  UpdateCommand,
  DeleteCommand,
  RunCommand,

  // group
  CreateGroup,
  UpdateGroup,
  DeleteGroup,

  // user
  ModifyUserEnabled,
  ModifyUserCreateServerPermissions,
  ModifyUserCreateBuildPermissions,
  ModifyUserPermissions,

  // github webhook automation
  AutoBuild,
  AutoPull,
}

#[derive(
  Serialize,
  Deserialize,
  Debug,
  Hash,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Default,
)]
#[serde(rename_all = "snake_case")]
pub enum PermissionLevel {
  #[default]
  None,
  Read,
  Execute,
  Update,
}

impl Default for &PermissionLevel {
  fn default() -> Self {
    &PermissionLevel::None
  }
}

impl From<PermissionLevel>
  for monitor_types::entities::PermissionLevel
{
  fn from(value: PermissionLevel) -> Self {
    use monitor_types::entities::PermissionLevel::*;
    match value {
      PermissionLevel::None => None,
      PermissionLevel::Read => Read,
      PermissionLevel::Execute => Execute,
      PermissionLevel::Update => Update,
    }
  }
}

#[derive(
  Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone, Copy,
)]
#[serde(rename_all = "snake_case")]
pub enum PermissionsTarget {
  Server,
  Deployment,
  Build,
  Procedure,
  Group,
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
pub enum Timelength {
  #[serde(rename = "1-sec")]
  OneSecond,
  #[serde(rename = "5-sec")]
  FiveSeconds,
  #[serde(rename = "10-sec")]
  TenSeconds,
  #[serde(rename = "15-sec")]
  FifteenSeconds,
  #[serde(rename = "30-sec")]
  ThirtySeconds,
  #[default]
  #[serde(rename = "1-min")]
  OneMinute,
  #[serde(rename = "2-min")]
  TwoMinutes,
  #[serde(rename = "5-min")]
  FiveMinutes,
  #[serde(rename = "10-min")]
  TenMinutes,
  #[serde(rename = "15-min")]
  FifteenMinutes,
  #[serde(rename = "30-min")]
  ThirtyMinutes,
  #[serde(rename = "1-hr")]
  OneHour,
  #[serde(rename = "2-hr")]
  TwoHours,
  #[serde(rename = "6-hr")]
  SixHours,
  #[serde(rename = "8-hr")]
  EightHours,
  #[serde(rename = "12-hr")]
  TwelveHours,
  #[serde(rename = "1-day")]
  OneDay,
  #[serde(rename = "3-day")]
  ThreeDay,
  #[serde(rename = "1-wk")]
  OneWeek,
  #[serde(rename = "2-wk")]
  TwoWeeks,
  #[serde(rename = "30-day")]
  ThirtyDays,
}

pub fn unix_from_monitor_ts(ts: &str) -> anyhow::Result<i64> {
  Ok(
    DateTime::parse_from_rfc3339(ts)
      .context("failed to parse rfc3339 timestamp")?
      .timestamp_millis(),
  )
}

pub fn monitor_ts_from_unix(ts: i64) -> anyhow::Result<String> {
  match Utc.timestamp_millis_opt(ts) {
    LocalResult::Single(dt) => {
      Ok(dt.to_rfc3339_opts(SecondsFormat::Millis, false))
    }
    LocalResult::None => {
      Err(anyhow!("out of bounds timestamp passed"))
    }
    _ => unreachable!(),
  }
}
