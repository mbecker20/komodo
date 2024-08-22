use monitor_client::entities::{
  deployment::{
    conversions_deserializer, term_labels_deserializer, Conversion,
    DeploymentImage, RestartMode, TerminationSignalLabel,
  },
  env_vars_deserializer, EnvironmentVar, TerminationSignal,
};
use serde::{Deserialize, Serialize};

use super::{build::ImageRegistry, resource::Resource};

pub type Deployment = Resource<DeploymentConfig, ()>;

impl From<Deployment>
  for monitor_client::entities::deployment::Deployment
{
  fn from(value: Deployment) -> Self {
    monitor_client::entities::deployment::Deployment {
      id: value.id,
      name: value.name,
      description: value.description,
      updated_at: value.updated_at,
      tags: value.tags,
      info: (),
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

  /// The image which the deployment deploys.
  /// Can either be a user inputted image, or a Monitor build.
  #[serde(default)]
  pub image: DeploymentImage,

  /// Configure the registry used to pull the image from the registry.
  /// Used with `docker login`.
  ///
  /// When using attached build as image source:
  ///  - If the field is `None` variant, will use the same ImageRegistry config as the build.
  ///  - Otherwise, it must match the variant of the ImageRegistry build config.
  ///  - Only the account is used, the organization is not needed here
  #[serde(default)]
  pub image_registry: ImageRegistry,

  /// Whether to skip secret interpolation into the deployment environment variables.
  #[serde(default)]
  pub skip_secret_interp: bool,

  /// Whether to redeploy the deployment whenever the attached build finishes.
  #[serde(default)]
  pub redeploy_on_build: bool,

  /// Whether to send ContainerStateChange alerts for this deployment.
  #[serde(default = "default_send_alerts")]
  pub send_alerts: bool,

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

  /// The default termination signal to use to stop the deployment. Defaults to SigTerm (default docker signal).
  #[serde(default)]
  pub termination_signal: TerminationSignal,

  /// The termination timeout.
  #[serde(default = "default_termination_timeout")]
  pub termination_timeout: i32,

  /// Extra args which are interpolated into the `docker run` command,
  /// and affect the container configuration.
  #[serde(default)]
  pub extra_args: Vec<String>,

  /// Labels attached to various termination signal options.
  /// Used to specify different shutdown functionality depending on the termination signal.
  #[serde(
    default = "default_term_signal_labels",
    deserialize_with = "term_labels_deserializer"
  )]
  pub term_signal_labels: Vec<TerminationSignalLabel>,

  /// The container port mapping.
  /// Irrelevant if container network is `host`.
  /// Maps ports on host to ports on container.
  #[serde(default, deserialize_with = "conversions_deserializer")]
  pub ports: Vec<Conversion>,

  /// The container volume mapping.
  /// Maps files / folders on host to files / folders in container.
  #[serde(default, deserialize_with = "conversions_deserializer")]
  pub volumes: Vec<Conversion>,

  /// The environment variables passed to the container.
  #[serde(default, deserialize_with = "env_vars_deserializer")]
  pub environment: Vec<EnvironmentVar>,

  /// The docker labels given to the container.
  #[serde(default, deserialize_with = "env_vars_deserializer")]
  pub labels: Vec<EnvironmentVar>,
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

impl From<DeploymentConfig>
  for monitor_client::entities::deployment::DeploymentConfig
{
  fn from(value: DeploymentConfig) -> Self {
    monitor_client::entities::deployment::DeploymentConfig {
      server_id: value.server_id,
      image: value.image,
      image_registry_account: match value.image_registry {
        ImageRegistry::None(_)
        | ImageRegistry::AwsEcr(_)
        | ImageRegistry::Custom(_) => String::new(),
        ImageRegistry::DockerHub(params) => params.account,
        ImageRegistry::Ghcr(params) => params.account,
      },
      skip_secret_interp: value.skip_secret_interp,
      redeploy_on_build: value.redeploy_on_build,
      send_alerts: value.send_alerts,
      network: value.network,
      restart: value.restart,
      command: value.command,
      termination_signal: value.termination_signal,
      termination_timeout: value.termination_timeout,
      extra_args: value.extra_args,
      term_signal_labels: value.term_signal_labels,
      ports: value.ports,
      volumes: value.volumes,
      environment: value.environment,
      labels: value.labels,
    }
  }
}
