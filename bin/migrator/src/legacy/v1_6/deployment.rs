use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{resource::Resource, EnvironmentVar, Version};

pub type Deployment = Resource<DeploymentConfig, ()>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeploymentConfig {
  /// The id of server the deployment is deployed on.
  #[serde(default, alias = "server")]
  pub server_id: String,

  /// Whether to send ContainerStateChange alerts for this deployment.
  #[serde(default = "default_send_alerts")]
  pub send_alerts: bool,

  /// The image which the deployment deploys.
  /// Can either be a user inputted image, or a Monitor build.
  #[serde(default)]
  pub image: DeploymentImage,

  /// Override the account used to pull the image from the registry.
  /// If it is empty, will use the same account as the build.
  #[serde(default)]
  pub docker_account: String,

  /// Whether to skip secret interpolation into the deployment environment variables.
  #[serde(default)]
  pub skip_secret_interp: bool,

  /// Whether to redeploy the deployment whenever the attached build finishes.
  #[serde(default)]
  pub redeploy_on_build: bool,

  /// Labels attached to various termination signal options.
  /// Used to specify different shutdown functionality depending on the termination signal.
  #[serde(default = "default_term_signal_labels")]
  pub term_signal_labels: Vec<TerminationSignalLabel>,

  /// The default termination signal to use to stop the deployment. Defaults to SigTerm (default docker signal).
  #[serde(default)]
  pub termination_signal: TerminationSignal,

  /// The termination timeout.
  #[serde(default = "default_termination_timeout")]
  pub termination_timeout: i32,

  /// The container port mapping.
  /// Irrelevant if container network is `host`.
  /// Maps ports on host to ports on container.
  #[serde(default)]
  pub ports: Vec<Conversion>,

  /// The container volume mapping.
  /// Maps files / folders on host to files / folders in container.
  #[serde(default)]
  pub volumes: Vec<Conversion>,

  /// The environment variables passed to the container.
  #[serde(default)]
  pub environment: Vec<EnvironmentVar>,

  /// The docker labels given to the container.
  #[serde(default)]
  pub labels: Vec<EnvironmentVar>,

  /// The network attached to the container.
  /// Default is `host`.
  #[serde(default = "default_network")]
  pub network: String,

  /// The restart mode given to the container.
  #[serde(default)]
  pub restart: RestartMode,

  /// This is interpolated at the end of the `docker run` command,
  /// which means they are either passed to the containers inner process,
  /// or replaces the container command, depending on use of ENTRYPOINT or CMD in dockerfile.
  /// Empty is no command.
  #[serde(default)]
  pub command: String,

  /// Extra args which are interpolated into the `docker run` command,
  /// and affect the container configuration.
  #[serde(default)]
  pub extra_args: Vec<String>,
}

fn default_send_alerts() -> bool {
  true
}

fn default_term_signal_labels() -> Vec<TerminationSignalLabel> {
  vec![TerminationSignalLabel::default()]
}

fn default_termination_timeout() -> i32 {
  10
}

fn default_network() -> String {
  String::from("host")
}

impl Default for DeploymentConfig {
  fn default() -> Self {
    Self {
      server_id: Default::default(),
      send_alerts: default_send_alerts(),
      image: Default::default(),
      docker_account: Default::default(),
      skip_secret_interp: Default::default(),
      redeploy_on_build: Default::default(),
      term_signal_labels: default_term_signal_labels(),
      termination_signal: Default::default(),
      termination_timeout: default_termination_timeout(),
      ports: Default::default(),
      volumes: Default::default(),
      environment: Default::default(),
      labels: Default::default(),
      network: default_network(),
      restart: Default::default(),
      command: Default::default(),
      extra_args: Default::default(),
    }
  }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type", content = "params")]
pub enum DeploymentImage {
  /// Deploy any external image.
  Image {
    /// The docker image, can be from any registry that works with docker and that the host server can reach.
    #[serde(default)]
    image: String,
  },

  /// Deploy a monitor build.
  Build {
    /// The id of the build
    #[serde(default, alias = "build")]
    build_id: String,
    /// Use a custom / older version of the image produced by the build.
    /// if version is 0.0.0, this means `latest` image.
    #[serde(default)]
    version: Version,
  },
}

impl Default for DeploymentImage {
  fn default() -> Self {
    Self::Image {
      image: Default::default(),
    }
  }
}

#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
pub struct Conversion {
  /// reference on the server.
  pub local: String,
  /// reference in the container.
  pub container: String,
}

/// A summary of a docker container on a server.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContainerSummary {
  /// Name of the container.
  pub name: String,
  /// Id of the container.
  pub id: String,
  /// The image the container is based on.
  pub image: String,
  /// The docker labels on the container.
  pub labels: HashMap<String, String>,
  /// The state of the container, like `running` or `not_deployed`
  pub state: DeploymentState,
  /// The status string of the docker container.
  pub status: Option<String>,
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

/// Variants de/serialized from/to snake_case.
///
/// Eg.
/// - NotDeployed -> not_deployed
/// - Restarting -> restarting
/// - Running -> running.
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
pub enum DeploymentState {
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

#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq,
)]
pub struct TerminationSignalLabel {
  pub signal: TerminationSignal,
  pub label: String,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct DeploymentActionState {
  pub deploying: bool,
  pub stopping: bool,
  pub starting: bool,
  pub removing: bool,
  pub renaming: bool,
}
