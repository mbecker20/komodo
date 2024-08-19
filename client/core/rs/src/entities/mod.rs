use std::str::FromStr;

use anyhow::Context;
use async_timing_util::unix_timestamp_ms;
use build::StandardRegistryConfig;
use clap::Parser;
use config::core::AwsEcrConfig;
use derive_empty_traits::EmptyTraits;
use serde::{
  de::{
    value::{MapAccessDeserializer, SeqAccessDeserializer},
    Visitor,
  },
  Deserialize, Deserializer, Serialize,
};
use serror::Serror;
use strum::{AsRefStr, Display, EnumString};
use typeshare::typeshare;

/// Subtypes of [Alert][alert::Alert].
pub mod alert;
/// Subtypes of [Alerter][alerter::Alerter].
pub mod alerter;
/// Subtypes of [ApiKey][api_key::ApiKey].
pub mod api_key;
/// Subtypes of [Build][build::Build].
pub mod build;
/// Subtypes of [Builder][builder::Builder].
pub mod builder;
/// [core config][config::core] and [periphery config][config::periphery]
pub mod config;
/// Subtypes of [Deployment][deployment::Deployment].
pub mod deployment;
/// Networks, Images, Containers.
pub mod docker;
/// Subtypes of [LogConfig][logger::LogConfig].
pub mod logger;
/// Subtypes of [Permission][permission::Permission].
pub mod permission;
/// Subtypes of [Procedure][procedure::Procedure].
pub mod procedure;
/// Subtypes of [ProviderAccount][provider::ProviderAccount]
pub mod provider;
/// Subtypes of [Repo][repo::Repo].
pub mod repo;
/// Subtypes of [Resource][resource::Resource].
pub mod resource;
/// Subtypes of [Server][server::Server].
pub mod server;
/// Subtypes of [ServerTemplate][server_template::ServerTemplate].
pub mod server_template;
/// Subtypes of [Stack][stack::Stack]
pub mod stack;
/// Subtypes for server stats reporting.
pub mod stats;
/// Subtypes of [ResourceSync][sync::ResourceSync]
pub mod sync;
/// Subtypes of [Tag][tag::Tag].
pub mod tag;
/// Subtypes of [ResourcesToml][toml::ResourcesToml].
pub mod toml;
/// Subtypes of [Update][update::Update].
pub mod update;
/// Subtypes of [User][user::User].
pub mod user;
/// Subtypes of [UserGroup][user_group::UserGroup].
pub mod user_group;
/// Subtypes of [Variable][variable::Variable]
pub mod variable;

#[typeshare(serialized_as = "number")]
pub type I64 = i64;
#[typeshare(serialized_as = "number")]
pub type U64 = u64;
#[typeshare(serialized_as = "number")]
pub type Usize = usize;
#[typeshare(serialized_as = "any")]
pub type MongoDocument = bson::Document;
#[typeshare(serialized_as = "any")]
pub type JsonValue = serde_json::Value;
#[typeshare(serialized_as = "MongoIdObj")]
pub type MongoId = String;
#[typeshare(serialized_as = "__Serror")]
pub type _Serror = Serror;

/// Represents an empty json object: `{}`
#[typeshare]
#[derive(
  Debug,
  Clone,
  Default,
  PartialEq,
  Serialize,
  Deserialize,
  Parser,
  EmptyTraits,
)]
pub struct NoData {}

pub trait MergePartial: Sized {
  type Partial;
  fn merge_partial(self, partial: Self::Partial) -> Self;
}

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
        image_name,
        image_registry,
        ..
      },
    ..
  }: &build::Build,
  aws_ecr: impl FnOnce(&String) -> Option<AwsEcrConfig>,
) -> anyhow::Result<String> {
  let name = if image_name.is_empty() {
    to_monitor_name(name)
  } else {
    to_monitor_name(image_name)
  };
  let name = match image_registry {
    build::ImageRegistry::None(_) => name,
    build::ImageRegistry::AwsEcr(label) => {
      let AwsEcrConfig {
        region, account_id, ..
      } = aws_ecr(label).with_context(|| {
        format!("didn't find aws ecr config for registry {label}")
      })?;
      format!("{account_id}.dkr.ecr.{region}.amazonaws.com/{name}")
    }
    build::ImageRegistry::Standard(StandardRegistryConfig {
      domain,
      account,
      organization,
    }) => {
      if !organization.is_empty() {
        let org = organization.to_lowercase();
        format!("{domain}/{org}/{name}")
      } else if !account.is_empty() {
        format!("{domain}/{account}/{name}")
      } else {
        name
      }
    }
  };
  Ok(name)
}

pub fn to_monitor_name(name: &str) -> String {
  name.to_lowercase().replace([' ', '.'], "_")
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
#[derive(Serialize, Debug, Clone, Copy, Default, PartialEq)]
pub struct Version {
  pub major: i32,
  pub minor: i32,
  pub patch: i32,
}

impl<'de> Deserialize<'de> for Version {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    #[derive(Deserialize)]
    struct VersionInner {
      major: i32,
      minor: i32,
      patch: i32,
    }

    impl From<VersionInner> for Version {
      fn from(
        VersionInner {
          major,
          minor,
          patch,
        }: VersionInner,
      ) -> Self {
        Version {
          major,
          minor,
          patch,
        }
      }
    }

    struct VersionVisitor;

    impl<'de> Visitor<'de> for VersionVisitor {
      type Value = Version;
      fn expecting(
        &self,
        formatter: &mut std::fmt::Formatter,
      ) -> std::fmt::Result {
        write!(
          formatter,
          "version string or object | example: '0.2.4' or {{ \"major\": 0, \"minor\": 2, \"patch\": 4, }}"
        )
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        v.try_into()
          .map_err(|e| serde::de::Error::custom(format!("{e:#}")))
      }

      fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
      where
        A: serde::de::MapAccess<'de>,
      {
        Ok(
          VersionInner::deserialize(MapAccessDeserializer::new(map))?
            .into(),
        )
      }
    }

    deserializer.deserialize_any(VersionVisitor)
  }
}

impl std::fmt::Display for Version {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "{}.{}.{}",
      self.major, self.minor, self.patch
    ))
  }
}

impl TryFrom<&str> for Version {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let mut split = value.split('.');
    let major = split
      .next()
      .context("must provide at least major version")?
      .parse::<i32>()
      .context("major version must be integer")?;
    let minor = split
      .next()
      .map(|minor| minor.parse::<i32>())
      .transpose()
      .context("minor version must be integer")?
      .unwrap_or_default();
    let patch = split
      .next()
      .map(|patch| patch.parse::<i32>())
      .transpose()
      .context("patch version must be integer")?
      .unwrap_or_default();
    Ok(Version {
      major,
      minor,
      patch,
    })
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
  Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize,
)]
pub struct EnvironmentVar {
  pub variable: String,
  pub value: String,
}

pub fn environment_vars_to_string(vars: &[EnvironmentVar]) -> String {
  vars
    .iter()
    .map(|EnvironmentVar { variable, value }| {
      format!("{variable}={value}")
    })
    .collect::<Vec<_>>()
    .join("\n")
}

pub fn environment_vars_from_str(
  value: &str,
) -> anyhow::Result<Vec<EnvironmentVar>> {
  let trimmed = value.trim();
  if trimmed.is_empty() {
    return Ok(Vec::new());
  }
  let res = trimmed
    .split('\n')
    .map(|line| line.trim())
    .enumerate()
    .filter(|(_, line)| {
      !line.is_empty()
        && !line.starts_with('#')
        && !line.starts_with("//")
    })
    .map(|(i, line)| {
      let (variable, value) = line
        .split_once('=')
        .with_context(|| format!("line {i} missing assignment (=)"))
        .map(|(variable, value)| {
          (variable.trim().to_string(), value.trim().to_string())
        })?;
      anyhow::Ok(EnvironmentVar { variable, value })
    })
    .collect::<anyhow::Result<Vec<_>>>()?;
  Ok(res)
}

pub fn env_vars_deserializer<'de, D>(
  deserializer: D,
) -> Result<Vec<EnvironmentVar>, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(EnvironmentVarVisitor)
}

pub fn option_env_vars_deserializer<'de, D>(
  deserializer: D,
) -> Result<Option<Vec<EnvironmentVar>>, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(OptionEnvVarVisitor)
}

struct EnvironmentVarVisitor;

impl<'de> Visitor<'de> for EnvironmentVarVisitor {
  type Value = Vec<EnvironmentVar>;

  fn expecting(
    &self,
    formatter: &mut std::fmt::Formatter,
  ) -> std::fmt::Result {
    write!(formatter, "string or Vec<EnvironmentVar>")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    environment_vars_from_str(v)
      .map_err(|e| serde::de::Error::custom(format!("{e:#}")))
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::SeqAccess<'de>,
  {
    #[derive(Deserialize)]
    struct EnvironmentVarInner {
      variable: String,
      value: String,
    }

    impl From<EnvironmentVarInner> for EnvironmentVar {
      fn from(value: EnvironmentVarInner) -> Self {
        Self {
          variable: value.variable,
          value: value.value,
        }
      }
    }

    let res = Vec::<EnvironmentVarInner>::deserialize(
      SeqAccessDeserializer::new(seq),
    )?
    .into_iter()
    .map(Into::into)
    .collect();
    Ok(res)
  }
}

struct OptionEnvVarVisitor;

impl<'de> Visitor<'de> for OptionEnvVarVisitor {
  type Value = Option<Vec<EnvironmentVar>>;

  fn expecting(
    &self,
    formatter: &mut std::fmt::Formatter,
  ) -> std::fmt::Result {
    write!(formatter, "null or string or Vec<EnvironmentVar>")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    EnvironmentVarVisitor.visit_str(v).map(Some)
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::SeqAccess<'de>,
  {
    EnvironmentVarVisitor.visit_seq(seq).map(Some)
  }

  fn visit_none<E>(self) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(None)
  }

  fn visit_unit<E>(self) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(None)
  }
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LatestCommit {
  pub hash: String,
  pub message: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CloneArgs {
  /// Resource name (eg Build name, Repo name)
  pub name: String,
  /// Git provider domain. Default: `github.com`
  pub provider: Option<String>,
  /// Use https (vs http).
  pub https: bool,
  /// Full repo identifier. <namespace>/<repo_name>
  pub repo: Option<String>,
  /// Git Branch. Default: `main`
  pub branch: Option<String>,
  /// Specific commit hash. Optional
  pub commit: Option<String>,
  /// The clone destination path
  pub destination: Option<String>,
  /// Command to run after the repo has been cloned
  pub on_clone: Option<SystemCommand>,
  /// Command to run after the repo has been pulled
  pub on_pull: Option<SystemCommand>,
  /// Configure the account used to access repo (if private)
  pub account: Option<String>,
}

impl From<&self::build::Build> for CloneArgs {
  fn from(build: &self::build::Build) -> CloneArgs {
    CloneArgs {
      name: build.name.clone(),
      repo: optional_string(&build.config.repo),
      branch: optional_string(&build.config.branch),
      commit: optional_string(&build.config.commit),
      destination: None,
      on_clone: build.config.pre_build.clone().into_option(),
      on_pull: None,
      provider: optional_string(&build.config.git_provider),
      https: build.config.git_https,
      account: optional_string(&build.config.git_account),
    }
  }
}

impl From<&self::repo::Repo> for CloneArgs {
  fn from(repo: &self::repo::Repo) -> CloneArgs {
    CloneArgs {
      name: repo.name.clone(),
      repo: optional_string(&repo.config.repo),
      branch: optional_string(&repo.config.branch),
      commit: optional_string(&repo.config.commit),
      destination: optional_string(&repo.config.path),
      on_clone: repo.config.on_clone.clone().into_option(),
      on_pull: repo.config.on_pull.clone().into_option(),
      provider: optional_string(&repo.config.git_provider),
      https: repo.config.git_https,
      account: optional_string(&repo.config.git_account),
    }
  }
}

impl From<&self::sync::ResourceSync> for CloneArgs {
  fn from(sync: &self::sync::ResourceSync) -> Self {
    CloneArgs {
      name: sync.name.clone(),
      repo: optional_string(&sync.config.repo),
      branch: optional_string(&sync.config.branch),
      commit: optional_string(&sync.config.commit),
      destination: None,
      on_clone: None,
      on_pull: None,
      provider: optional_string(&sync.config.git_provider),
      https: sync.config.git_https,
      account: optional_string(&sync.config.git_account),
    }
  }
}

impl From<&self::stack::Stack> for CloneArgs {
  fn from(stack: &self::stack::Stack) -> Self {
    CloneArgs {
      name: stack.name.clone(),
      repo: optional_string(&stack.config.repo),
      branch: optional_string(&stack.config.branch),
      commit: optional_string(&stack.config.commit),
      destination: None,
      on_clone: None,
      on_pull: None,
      provider: optional_string(&stack.config.git_provider),
      https: stack.config.git_https,
      account: optional_string(&stack.config.git_account),
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

impl TryInto<async_timing_util::Timelength> for Timelength {
  type Error = anyhow::Error;
  fn try_into(
    self,
  ) -> Result<async_timing_util::Timelength, Self::Error> {
    async_timing_util::Timelength::from_str(&self.to_string())
      .context("failed to parse timelength?")
  }
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
  Default,
  Display,
  EnumString,
  AsRefStr,
)]
pub enum Operation {
  // do nothing
  #[default]
  None,

  // server
  CreateServer,
  UpdateServer,
  DeleteServer,
  RenameServer,
  PruneImages,
  PruneContainers,
  PruneNetworks,
  CreateNetwork,
  DeleteNetwork,
  StopAllContainers,

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
  Deploy,
  StartContainer,
  RestartContainer,
  PauseContainer,
  UnpauseContainer,
  StopContainer,
  RemoveContainer,
  RenameDeployment,

  // repo
  CreateRepo,
  UpdateRepo,
  DeleteRepo,
  CloneRepo,
  PullRepo,
  BuildRepo,
  CancelRepoBuild,

  // alerter
  CreateAlerter,
  UpdateAlerter,
  DeleteAlerter,

  // procedure
  CreateProcedure,
  UpdateProcedure,
  DeleteProcedure,
  RunProcedure,

  // server template
  CreateServerTemplate,
  UpdateServerTemplate,
  DeleteServerTemplate,
  LaunchServer,

  // sync
  CreateResourceSync,
  UpdateResourceSync,
  DeleteResourceSync,
  RunSync,

  // stack
  CreateStack,
  UpdateStack,
  RenameStack,
  DeleteStack,
  RefreshStackCache,
  DeployStack,
  StartStack,
  RestartStack,
  PauseStack,
  UnpauseStack,
  StopStack,
  DestroyStack,

  // stack (service)
  StartStackService,
  RestartStackService,
  PauseStackService,
  UnpauseStackService,
  StopStackService,

  // variable
  CreateVariable,
  UpdateVariableValue,
  DeleteVariable,

  // git provider
  CreateGitProviderAccount,
  UpdateGitProviderAccount,
  DeleteGitProviderAccount,

  // docker registry
  CreateDockerRegistryAccount,
  UpdateDockerRegistryAccount,
  DeleteDockerRegistryAccount,
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
