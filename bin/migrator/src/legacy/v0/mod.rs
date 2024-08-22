use std::collections::HashMap;

use anyhow::Context;
use chrono::DateTime;
use mungos::{init::MongoBuilder, mongodb::Collection};
use serde::{Deserialize, Serialize};

mod build;
mod config;
mod deployment;
mod server;
mod update;
mod user;

pub use build::*;
pub use config::*;
pub use deployment::*;
pub use server::*;
pub use update::*;
pub use user::*;

pub struct DbClient {
  pub users: Collection<User>,
  pub servers: Collection<Server>,
  pub deployments: Collection<Deployment>,
  pub builds: Collection<Build>,
  pub updates: Collection<Update>,
}

impl DbClient {
  pub async fn new(
    legacy_uri: &str,
    legacy_db_name: &str,
  ) -> DbClient {
    let client = MongoBuilder::default()
      .uri(legacy_uri)
      .build()
      .await
      .expect("failed to init legacy mongo client");
    let db = client.database(legacy_db_name);
    DbClient {
      users: db.collection("users"),
      servers: db.collection("servers"),
      deployments: db.collection("deployments"),
      builds: db.collection("builds"),
      updates: db.collection("updates"),
    }
  }
}

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
  for monitor_client::entities::EnvironmentVar
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

impl From<Operation> for monitor_client::entities::Operation {
  fn from(value: Operation) -> Self {
    use monitor_client::entities::Operation::*;
    match value {
      Operation::None => None,
      Operation::CreateServer => CreateServer,
      Operation::UpdateServer => UpdateServer,
      Operation::DeleteServer => DeleteServer,
      Operation::PruneImagesServer => PruneImages,
      Operation::PruneContainersServer => PruneContainers,
      Operation::PruneNetworksServer => PruneNetworks,
      Operation::RenameServer => RenameServer,
      Operation::CreateBuild => CreateBuild,
      Operation::UpdateBuild => UpdateBuild,
      Operation::DeleteBuild => DeleteBuild,
      Operation::BuildBuild => RunBuild,
      Operation::CreateDeployment => CreateDeployment,
      Operation::UpdateDeployment => UpdateDeployment,
      Operation::DeleteDeployment => DeleteDeployment,
      Operation::DeployContainer => Deploy,
      Operation::StopContainer => StopDeployment,
      Operation::StartContainer => StartDeployment,
      Operation::RemoveContainer => DestroyDeployment,
      Operation::PullDeployment => None,
      Operation::RecloneDeployment => None,
      Operation::RenameDeployment => RenameDeployment,
      Operation::CreateProcedure => None,
      Operation::UpdateProcedure => None,
      Operation::DeleteProcedure => None,
      Operation::CreateCommand => None,
      Operation::UpdateCommand => None,
      Operation::DeleteCommand => None,
      Operation::RunCommand => None,
      Operation::CreateGroup => None,
      Operation::UpdateGroup => None,
      Operation::DeleteGroup => None,
      Operation::ModifyUserEnabled => None,
      Operation::ModifyUserCreateServerPermissions => None,
      Operation::ModifyUserCreateBuildPermissions => None,
      Operation::ModifyUserPermissions => None,
      Operation::AutoBuild => RunBuild,
      Operation::AutoPull => PullRepo,
    }
  }
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
  for monitor_client::entities::permission::PermissionLevel
{
  fn from(value: PermissionLevel) -> Self {
    use monitor_client::entities::permission::PermissionLevel::*;
    match value {
      PermissionLevel::None => None,
      PermissionLevel::Read => Read,
      PermissionLevel::Execute => Execute,
      PermissionLevel::Update => Write,
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
