use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use serde::{Deserialize, Serialize};
use serror::Serror;
use strum::{Display, EnumString};
use typeshare::typeshare;

pub mod alert;
pub mod alerter;
pub mod api_key;
pub mod build;
pub mod builder;
pub mod deployment;
pub mod permission;
pub mod procedure;
pub mod repo;
pub mod resource;
pub mod server;
pub mod tag;
pub mod update;
pub mod user;

#[typeshare(serialized_as = "number")]
pub type I64 = i64;
#[typeshare(serialized_as = "number")]
pub type U64 = u64;
#[typeshare(serialized_as = "any")]
pub type MongoDocument = mungos::mongodb::bson::Document;
#[typeshare(serialized_as = "MongoIdObj")]
pub type MongoId = String;
#[typeshare(serialized_as = "__Serror")]
pub type _Serror = Serror;

pub fn all_logs_success(logs: &[update::Log]) -> bool {
  for log in logs {
    if !log.success {
      return false;
    }
  }
  true
}

pub fn optional_string(string: &str) -> Option<String> {
  if string.is_empty() {
    None
  } else {
    Some(string.to_string())
  }
}

pub fn get_image_name(
  build::Build {
    name,
    config:
      build::BuildConfig {
        docker_organization,
        docker_account,
        ..
      },
    ..
  }: &build::Build,
) -> String {
  let name = to_monitor_name(name);
  if !docker_organization.is_empty() {
    format!("{docker_organization}/{name}")
  } else if !docker_account.is_empty() {
    format!("{docker_account}/{name}")
  } else {
    name
  }
}

pub fn to_monitor_name(name: &str) -> String {
  name.to_lowercase().replace(' ', "_")
}

pub fn monitor_timestamp() -> i64 {
  unix_timestamp_ms() as i64
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MongoIdObj {
  #[serde(rename = "$oid")]
  pub oid: String,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct __Serror {
  pub error: String,
  pub trace: Vec<String>,
}

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq,
)]
pub struct SystemCommand {
  #[serde(default)]
  pub path: String,
  #[serde(default)]
  pub command: String,
}

impl SystemCommand {
  pub fn command(&self) -> Option<String> {
    if self.is_none() {
      None
    } else {
      Some(format!("cd {} && {}", self.path, self.command))
    }
  }

  pub fn into_option(self) -> Option<SystemCommand> {
    if self.is_none() {
      None
    } else {
      Some(self)
    }
  }

  pub fn is_none(&self) -> bool {
    self.path.is_empty() || self.command.is_empty()
  }
}

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
pub struct Version {
  pub major: i32,
  pub minor: i32,
  pub patch: i32,
}

impl ToString for Version {
  fn to_string(&self) -> String {
    format!("{}.{}.{}", self.major, self.minor, self.patch)
  }
}

impl TryFrom<&str> for Version {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let vals = value
      .split('.')
      .map(|v| {
        anyhow::Ok(
          v.parse().context("failed at parsing value into i32")?,
        )
      })
      .collect::<anyhow::Result<Vec<i32>>>()?;
    let version = Version {
      major: *vals
        .first()
        .ok_or(anyhow!("must include at least major version"))?,
      minor: *vals.get(1).unwrap_or(&0),
      patch: *vals.get(2).unwrap_or(&0),
    };
    Ok(version)
  }
}

impl Version {
  pub fn increment(&mut self) {
    self.patch += 1;
  }

  pub fn is_none(&self) -> bool {
    self.major == 0 && self.minor == 0 && self.patch == 0
  }
}

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
pub struct EnvironmentVar {
  pub variable: String,
  pub value: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CloneArgs {
  pub name: String,
  pub repo: Option<String>,
  pub branch: Option<String>,
  pub on_clone: Option<SystemCommand>,
  pub on_pull: Option<SystemCommand>,
  pub github_account: Option<String>,
}

impl From<&self::build::Build> for CloneArgs {
  fn from(build: &self::build::Build) -> CloneArgs {
    CloneArgs {
      name: build.name.clone(),
      repo: optional_string(&build.config.repo),
      branch: optional_string(&build.config.branch),
      on_clone: build.config.pre_build.clone().into_option(),
      on_pull: None,
      github_account: optional_string(&build.config.github_account),
    }
  }
}

impl From<&self::repo::Repo> for CloneArgs {
  fn from(repo: &self::repo::Repo) -> CloneArgs {
    CloneArgs {
      name: repo.name.clone(),
      repo: optional_string(&repo.config.repo),
      branch: optional_string(&repo.config.branch),
      on_clone: repo.config.on_clone.clone().into_option(),
      on_pull: repo.config.on_pull.clone().into_option(),
      github_account: optional_string(&repo.config.github_account),
    }
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
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Timelength {
  #[serde(rename = "1-sec")]
  #[strum(serialize = "1-sec")]
  OneSecond,
  #[serde(rename = "5-sec")]
  #[strum(serialize = "5-sec")]
  FiveSeconds,
  #[serde(rename = "10-sec")]
  #[strum(serialize = "10-sec")]
  TenSeconds,
  #[serde(rename = "15-sec")]
  #[strum(serialize = "15-sec")]
  FifteenSeconds,
  #[serde(rename = "30-sec")]
  #[strum(serialize = "30-sec")]
  ThirtySeconds,
  #[default]
  #[serde(rename = "1-min")]
  #[strum(serialize = "1-min")]
  OneMinute,
  #[serde(rename = "2-min")]
  #[strum(serialize = "2-min")]
  TwoMinutes,
  #[serde(rename = "5-min")]
  #[strum(serialize = "5-min")]
  FiveMinutes,
  #[serde(rename = "10-min")]
  #[strum(serialize = "10-min")]
  TenMinutes,
  #[serde(rename = "15-min")]
  #[strum(serialize = "15-min")]
  FifteenMinutes,
  #[serde(rename = "30-min")]
  #[strum(serialize = "30-min")]
  ThirtyMinutes,
  #[serde(rename = "1-hr")]
  #[strum(serialize = "1-hr")]
  OneHour,
  #[serde(rename = "2-hr")]
  #[strum(serialize = "2-hr")]
  TwoHours,
  #[serde(rename = "6-hr")]
  #[strum(serialize = "6-hr")]
  SixHours,
  #[serde(rename = "8-hr")]
  #[strum(serialize = "8-hr")]
  EightHours,
  #[serde(rename = "12-hr")]
  #[strum(serialize = "12-hr")]
  TwelveHours,
  #[serde(rename = "1-day")]
  #[strum(serialize = "1-day")]
  OneDay,
  #[serde(rename = "3-day")]
  #[strum(serialize = "3-day")]
  ThreeDay,
  #[serde(rename = "1-wk")]
  #[strum(serialize = "1-wk")]
  OneWeek,
  #[serde(rename = "2-wk")]
  #[strum(serialize = "2-wk")]
  TwoWeeks,
  #[serde(rename = "30-day")]
  #[strum(serialize = "30-day")]
  ThirtyDays,
}

#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Default,
  Display,
  EnumString,
  PartialEq,
  Hash,
  Eq,
  Clone,
  Copy,
)]
pub enum Operation {
  // do nothing
  #[default]
  None,

  // server
  LaunchServer,
  CreateServer,
  UpdateServer,
  DeleteServer,
  RenameServer,
  PruneImagesServer,
  PruneContainersServer,
  PruneNetworksServer,
  CreateNetwork,
  DeleteNetwork,

  // build
  CreateBuild,
  UpdateBuild,
  DeleteBuild,
  RunBuild,
  CancelBuild,

  // builder
  CreateBuilder,
  UpdateBuilder,
  DeleteBuilder,

  // deployment
  CreateDeployment,
  UpdateDeployment,
  DeleteDeployment,
  DeployContainer,
  StopContainer,
  StopAllContainers,
  StartContainer,
  RemoveContainer,
  RenameDeployment,

  // repo
  CreateRepo,
  UpdateRepo,
  DeleteRepo,
  CloneRepo,
  PullRepo,

  // alerter
  CreateAlerter,
  UpdateAlerter,
  DeleteAlerter,

  // procedure
  CreateProcedure,
  UpdateProcedure,
  DeleteProcedure,
  RunProcedure,

  // user
  UpdateUserPermissions,
  UpdateUserPermissionsOnTarget,
}

#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Default,
  Display,
  EnumString,
  PartialEq,
  Hash,
  Eq,
  Clone,
  Copy,
)]
pub enum SearchCombinator {
  #[default]
  Or,
  And,
}
