use monitor_client::entities::deployment::{
  Conversion, DeploymentImage, RestartMode, TerminationSignal,
  TerminationSignalLabel,
};
use serde::{Deserialize, Serialize};

use super::{resource::Resource, EnvironmentVar};

pub type Deployment = Resource<DeploymentConfig, ()>;

impl From<Deployment>
  for monitor_client::entities::deployment::Deployment
{
  fn from(value: Deployment) -> Self {
    Self {
      id: value.id,
      name: value.name,
      description: value.description,
      updated_at: value.updated_at,
      tags: value.tags,
      info: value.info,
      config: value.config.into(),
      base_permission: Default::default(),
    }
  }
}

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
  #[serde(default)]
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

impl From<DeploymentConfig>
  for monitor_client::entities::deployment::DeploymentConfig
{
  fn from(value: DeploymentConfig) -> Self {
    Self {
      server_id: value.server_id,
      send_alerts: value.send_alerts,
      image: value.image,
      image_registry_account: value.docker_account,
      skip_secret_interp: value.skip_secret_interp,
      redeploy_on_build: value.redeploy_on_build,
      term_signal_labels: value.term_signal_labels,
      termination_signal: value.termination_signal,
      termination_timeout: value.termination_timeout,
      ports: value.ports,
      volumes: value.volumes,
      environment: value
        .environment
        .into_iter()
        .map(Into::into)
        .collect(),
      labels: value.labels.into_iter().map(Into::into).collect(),
      network: value.network,
      restart: value.restart,
      command: value.command,
      extra_args: value.extra_args,
    }
  }
}

fn default_send_alerts() -> bool {
  true
}

fn default_termination_timeout() -> i32 {
  10
}

fn default_network() -> String {
  String::from("host")
}
