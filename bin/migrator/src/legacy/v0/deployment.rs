use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

use crate::legacy::v0::unix_from_monitor_ts;

use super::{Command, EnvironmentVar, PermissionsMap, Version};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Deployment {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: String,

  pub name: String, // must be formatted to be compat with docker

  #[serde(default)]
  pub description: String,

  pub server_id: String,

  #[serde(default)]
  pub permissions: PermissionsMap,

  #[serde(default)]
  pub skip_secret_interp: bool,

  pub docker_run_args: DockerRunArgs,

  #[serde(default = "default_term_signal_labels")]
  pub term_signal_labels: Vec<TerminationSignalLabel>,

  #[serde(default)]
  pub termination_signal: TerminationSignal,

  #[serde(default = "default_termination_timeout")]
  pub termination_timeout: i32,

  pub build_id: Option<String>,

  #[serde(default)]
  pub redeploy_on_build: bool,

  pub build_version: Option<Version>,

  // deployment repo related
  pub repo: Option<String>,

  pub branch: Option<String>,

  pub github_account: Option<String>,

  pub on_clone: Option<Command>,

  pub on_pull: Option<Command>,

  pub repo_mount: Option<Conversion>,

  #[serde(default, skip_serializing_if = "String::is_empty")]
  pub created_at: String,
  #[serde(default)]
  pub updated_at: String,
}

fn default_termination_timeout() -> i32 {
  10
}

fn default_term_signal_labels() -> Vec<TerminationSignalLabel> {
  vec![TerminationSignalLabel::default()]
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeploymentWithContainerState {
  pub deployment: Deployment,
  pub state: DockerContainerState,
  pub container: Option<BasicContainerInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeploymentActionState {
  pub deploying: bool,
  pub stopping: bool,
  pub starting: bool,
  pub removing: bool,
  pub pulling: bool,
  pub recloning: bool,
  pub updating: bool,
  pub renaming: bool,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq,
)]
pub struct TerminationSignalLabel {
  pub signal: TerminationSignal,
  pub label: String,
}

impl From<TerminationSignalLabel>
  for monitor_client::entities::deployment::TerminationSignalLabel
{
  fn from(value: TerminationSignalLabel) -> Self {
    Self {
      signal: value.signal.into(),
      label: value.label,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DockerRunArgs {
  pub image: String,

  #[serde(default)]
  pub ports: Vec<Conversion>,

  #[serde(default)]
  pub volumes: Vec<Conversion>,

  #[serde(default)]
  pub environment: Vec<EnvironmentVar>,

  #[serde(default = "default_network")]
  pub network: String,

  #[serde(default)]
  pub restart: RestartMode,

  pub post_image: Option<String>,

  pub container_user: Option<String>,

  #[serde(default)]
  pub extra_args: Vec<String>,

  pub docker_account: Option<String>, // the username of the dockerhub account
}

impl Default for DockerRunArgs {
  fn default() -> DockerRunArgs {
    DockerRunArgs {
      network: "host".to_string(),
      image: Default::default(),
      ports: Default::default(),
      volumes: Default::default(),
      environment: Default::default(),
      restart: Default::default(),
      post_image: Default::default(),
      container_user: Default::default(),
      extra_args: Default::default(),
      docker_account: Default::default(),
    }
  }
}

fn default_network() -> String {
  String::from("host")
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicContainerInfo {
  pub name: String,
  pub id: String,
  pub image: String,
  pub state: DockerContainerState,
  pub status: Option<String>,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
pub struct Conversion {
  pub local: String,
  pub container: String,
}

impl From<Conversion>
  for monitor_client::entities::deployment::Conversion
{
  fn from(value: Conversion) -> Self {
    Self {
      local: value.local,
      container: value.container,
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DockerContainerStats {
  #[serde(alias = "Name")]
  pub name: String,
  #[serde(alias = "CPUPerc")]
  pub cpu_perc: String,
  #[serde(alias = "MemPerc")]
  pub mem_perc: String,
  #[serde(alias = "MemUsage")]
  pub mem_usage: String,
  #[serde(alias = "NetIO")]
  pub net_io: String,
  #[serde(alias = "BlockIO")]
  pub block_io: String,
  #[serde(alias = "PIDs")]
  pub pids: String,
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
pub enum DockerContainerState {
  #[default]
  Unknown,
  NotDeployed,
  Created,
  Restarting,
  Running,
  Removing,
  Paused,
  Exited,
  Dead,
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
pub enum RestartMode {
  #[default]
  #[serde(rename = "no")]
  NoRestart,
  #[serde(rename = "on-failure")]
  OnFailure,
  #[serde(rename = "always")]
  Always,
  #[serde(rename = "unless-stopped")]
  UnlessStopped,
}

impl From<RestartMode>
  for monitor_client::entities::deployment::RestartMode
{
  fn from(value: RestartMode) -> Self {
    use monitor_client::entities::deployment::RestartMode::*;
    match value {
      RestartMode::NoRestart => NoRestart,
      RestartMode::OnFailure => OnFailure,
      RestartMode::Always => Always,
      RestartMode::UnlessStopped => UnlessStopped,
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
#[serde(rename_all = "UPPERCASE")]
#[allow(clippy::enum_variant_names)]
pub enum TerminationSignal {
  #[serde(alias = "1")]
  SigHup,
  #[serde(alias = "2")]
  SigInt,
  #[serde(alias = "3")]
  SigQuit,
  #[default]
  #[serde(alias = "15")]
  SigTerm,
}

impl From<TerminationSignal>
  for monitor_client::entities::deployment::TerminationSignal
{
  fn from(value: TerminationSignal) -> Self {
    use monitor_client::entities::deployment::TerminationSignal::*;
    match value {
      TerminationSignal::SigHup => SigHup,
      TerminationSignal::SigInt => SigInt,
      TerminationSignal::SigQuit => SigQuit,
      TerminationSignal::SigTerm => SigTerm,
    }
  }
}

impl TryFrom<Deployment>
  for monitor_client::entities::deployment::Deployment
{
  type Error = anyhow::Error;
  fn try_from(value: Deployment) -> Result<Self, Self::Error> {
    let image = if let Some(build_id) = value.build_id {
      monitor_client::entities::deployment::DeploymentImage::Build {
        build_id,
        version: value.build_version.unwrap_or_default().into(),
      }
    } else {
      monitor_client::entities::deployment::DeploymentImage::Image {
        image: value.docker_run_args.image,
      }
    };
    let deployment = Self {
      id: value.id,
      name: value.name,
      description: value.description,
      // permissions: value
      //   .permissions
      //   .into_iter()
      //   .map(|(id, p)| (id, p.into()))
      //   .collect(),
      updated_at: unix_from_monitor_ts(&value.updated_at)?,
      tags: Vec::new(),
      info: (),
      config:
        monitor_client::entities::deployment::DeploymentConfig {
          server_id: value.server_id,
          send_alerts: true,
          image,
          skip_secret_interp: value.skip_secret_interp,
          redeploy_on_build: value.redeploy_on_build,
          term_signal_labels: value
            .term_signal_labels
            .into_iter()
            .map(|t| t.into())
            .collect(),
          termination_signal: value.termination_signal.into(),
          termination_timeout: value.termination_timeout,
          ports: value
            .docker_run_args
            .ports
            .into_iter()
            .map(|p| p.into())
            .collect(),
          volumes: value
            .docker_run_args
            .volumes
            .into_iter()
            .map(|v| v.into())
            .collect(),
          environment: value
            .docker_run_args
            .environment
            .into_iter()
            .map(|e| e.into())
            .collect(),
          network: value.docker_run_args.network,
          restart: value.docker_run_args.restart.into(),
          command: value
            .docker_run_args
            .post_image
            .unwrap_or_default(),
          extra_args: value.docker_run_args.extra_args,
          docker_account: value
            .docker_run_args
            .docker_account
            .unwrap_or_default(),
          labels: Default::default(),
        },
    };
    Ok(deployment)
  }
}
