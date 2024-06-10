use std::collections::HashMap;

use anyhow::Context;
use bson::{doc, Document};
use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use derive_variants::EnumVariants;
use partial_derive2::Partial;
use serde::{
  de::{value::SeqAccessDeserializer, Visitor},
  Deserialize, Deserializer, Serialize,
};
use strum::{Display, EnumString};
use typeshare::typeshare;

use super::{
  build::ImageRegistry,
  resource::{Resource, ResourceListItem, ResourceQuery},
  EnvironmentVar, Version,
};

#[typeshare]
pub type Deployment = Resource<DeploymentConfig, ()>;

#[typeshare]
pub type DeploymentListItem =
  ResourceListItem<DeploymentListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeploymentListItemInfo {
  /// The state of the deployment / underlying docker container.
  pub state: DeploymentState,
  /// The status of the docker container (eg. up 12 hours, exited 5 minutes ago.)
  pub status: Option<String>,
  /// The image attached to the deployment.
  pub image: String,
  /// The server that deployment sits on.
  pub server_id: String,
  /// An attached monitor build, if it exists.
  pub build_id: Option<String>,
}

#[typeshare(serialized_as = "Partial<DeploymentConfig>")]
pub type _PartialDeploymentConfig = PartialDeploymentConfig;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone)]
#[partial(skip_serializing_none, from, diff)]
pub struct DeploymentConfig {
  /// The id of server the deployment is deployed on.
  #[serde(default, alias = "server")]
  #[partial_attr(serde(alias = "server"))]
  #[builder(default)]
  pub server_id: String,

  /// Whether to send ContainerStateChange alerts for this deployment.
  #[serde(default = "default_send_alerts")]
  #[builder(default = "default_send_alerts()")]
  #[partial_default(default_send_alerts())]
  pub send_alerts: bool,

  /// The image which the deployment deploys.
  /// Can either be a user inputted image, or a Monitor build.
  #[serde(default)]
  #[builder(default)]
  pub image: DeploymentImage,

  /// Configure the registry used to pull the image from the registry.
  /// Used with `docker login`.
  ///
  /// When using attached build as image source:
  ///  - If the field is `None` variant, will use the same ImageRegistry config as the build.
  ///  - Otherwise, it must match the variant of the ImageRegistry build config.
  ///  - Only the account is used, the organization is not needed here
  #[serde(default)]
  #[builder(default)]
  pub image_registry: ImageRegistry,

  /// Whether to skip secret interpolation into the deployment environment variables.
  #[serde(default)]
  #[builder(default)]
  pub skip_secret_interp: bool,

  /// Whether to redeploy the deployment whenever the attached build finishes.
  #[serde(default)]
  #[builder(default)]
  pub redeploy_on_build: bool,

  /// Labels attached to various termination signal options.
  /// Used to specify different shutdown functionality depending on the termination signal.
  #[serde(
    default = "default_term_signal_labels",
    deserialize_with = "term_labels_deserializer"
  )]
  #[partial_attr(serde(
    deserialize_with = "option_term_labels_deserializer"
  ))]
  #[builder(default = "default_term_signal_labels()")]
  #[partial_default(default_term_signal_labels())]
  pub term_signal_labels: Vec<TerminationSignalLabel>,

  /// The default termination signal to use to stop the deployment. Defaults to SigTerm (default docker signal).
  #[serde(default)]
  #[builder(default)]
  pub termination_signal: TerminationSignal,

  /// The termination timeout.
  #[serde(default = "default_termination_timeout")]
  #[builder(default = "default_termination_timeout()")]
  #[partial_default(default_termination_timeout())]
  pub termination_timeout: i32,

  /// The container port mapping.
  /// Irrelevant if container network is `host`.
  /// Maps ports on host to ports on container.
  #[serde(default, deserialize_with = "conversions_deserializer")]
  #[partial_attr(serde(
    deserialize_with = "option_conversions_deserializer"
  ))]
  #[builder(default)]
  pub ports: Vec<Conversion>,

  /// The container volume mapping.
  /// Maps files / folders on host to files / folders in container.
  #[serde(default, deserialize_with = "conversions_deserializer")]
  #[partial_attr(serde(
    deserialize_with = "option_conversions_deserializer"
  ))]
  #[builder(default)]
  pub volumes: Vec<Conversion>,

  /// The environment variables passed to the container.
  #[serde(
    default,
    deserialize_with = "super::env_vars_deserializer"
  )]
  #[partial_attr(serde(
    deserialize_with = "super::option_env_vars_deserializer"
  ))]
  #[builder(default)]
  pub environment: Vec<EnvironmentVar>,

  /// The docker labels given to the container.
  #[serde(
    default,
    deserialize_with = "super::env_vars_deserializer"
  )]
  #[partial_attr(serde(
    deserialize_with = "super::option_env_vars_deserializer"
  ))]
  #[builder(default)]
  pub labels: Vec<EnvironmentVar>,

  /// The network attached to the container.
  /// Default is `host`.
  #[serde(default = "default_network")]
  #[builder(default = "default_network()")]
  #[partial_default(default_network())]
  pub network: String,

  /// The restart mode given to the container.
  #[serde(default)]
  #[builder(default)]
  pub restart: RestartMode,

  /// This is interpolated at the end of the `docker run` command,
  /// which means they are either passed to the containers inner process,
  /// or replaces the container command, depending on use of ENTRYPOINT or CMD in dockerfile.
  /// Empty is no command.
  #[serde(default)]
  #[builder(default)]
  pub command: String,

  /// Extra args which are interpolated into the `docker run` command,
  /// and affect the container configuration.
  #[serde(default)]
  #[builder(default)]
  pub extra_args: Vec<String>,
}

impl DeploymentConfig {
  pub fn builder() -> DeploymentConfigBuilder {
    DeploymentConfigBuilder::default()
  }
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
      image_registry: Default::default(),
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

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, PartialEq, EnumVariants,
)]
#[variant_derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  PartialEq,
  Eq,
  Display,
  EnumString
)]
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

#[typeshare]
#[derive(
  Debug, Clone, Default, PartialEq, Serialize, Deserialize,
)]
pub struct Conversion {
  /// reference on the server.
  pub local: String,
  /// reference in the container.
  pub container: String,
}

pub fn conversions_from_str(
  value: &str,
) -> anyhow::Result<Vec<Conversion>> {
  let res = value
    .split('\n')
    .map(|line| line.trim())
    .enumerate()
    .filter(|(_, line)| !line.starts_with('#'))
    .map(|(i, line)| {
      let mut split = line.split('=');
      let local = split
        .next()
        .with_context(|| {
          format!("line {i} does not have 'local' key")
        })?
        .trim()
        .to_string();
      // remove trailing comments
      let mut container_split = split
        .next()
        .with_context(|| {
          format!("line {i} does not have 'container' key")
        })?
        .split('#');
      let container = container_split
        .next()
        .with_context(|| {
          format!("line {i} does not have 'container' key")
        })?
        .trim()
        .to_string();
      anyhow::Ok(Conversion { local, container })
    })
    .collect::<anyhow::Result<Vec<_>>>()?;
  Ok(res)
}

pub fn conversions_deserializer<'de, D>(
  deserializer: D,
) -> Result<Vec<Conversion>, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(ConversionVisitor)
}

pub fn option_conversions_deserializer<'de, D>(
  deserializer: D,
) -> Result<Option<Vec<Conversion>>, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(OptionConversionVisitor)
}

struct ConversionVisitor;

impl<'de> Visitor<'de> for ConversionVisitor {
  type Value = Vec<Conversion>;

  fn expecting(
    &self,
    formatter: &mut std::fmt::Formatter,
  ) -> std::fmt::Result {
    write!(formatter, "string or Vec<Conversion>")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    conversions_from_str(v)
      .map_err(|e| serde::de::Error::custom(format!("{e:#}")))
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::SeqAccess<'de>,
  {
    #[derive(Deserialize)]
    struct ConversionInner {
      local: String,
      container: String,
    }

    impl From<ConversionInner> for Conversion {
      fn from(value: ConversionInner) -> Self {
        Self {
          local: value.local,
          container: value.container,
        }
      }
    }

    let res = Vec::<ConversionInner>::deserialize(
      SeqAccessDeserializer::new(seq),
    )?
    .into_iter()
    .map(Into::into)
    .collect();
    Ok(res)
  }
}

struct OptionConversionVisitor;

impl<'de> Visitor<'de> for OptionConversionVisitor {
  type Value = Option<Vec<Conversion>>;

  fn expecting(
    &self,
    formatter: &mut std::fmt::Formatter,
  ) -> std::fmt::Result {
    write!(formatter, "null or string or Vec<Conversion>")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    ConversionVisitor.visit_str(v).map(Some)
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::SeqAccess<'de>,
  {
    ConversionVisitor.visit_seq(seq).map(Some)
  }

  fn visit_none<E>(self) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(None)
  }
}

/// A summary of a docker container on a server.
#[typeshare]
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

#[typeshare]
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
#[typeshare]
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
  Display,
  EnumString,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
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

#[typeshare]
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
  Display,
  EnumString,
)]
pub enum RestartMode {
  #[default]
  #[serde(rename = "no")]
  #[strum(serialize = "no")]
  NoRestart,
  #[serde(rename = "on-failure")]
  #[strum(serialize = "on-failure")]
  OnFailure,
  #[serde(rename = "always")]
  #[strum(serialize = "always")]
  Always,
  #[serde(rename = "unless-stopped")]
  #[strum(serialize = "unless-stopped")]
  UnlessStopped,
}

#[typeshare]
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
  Display,
  EnumString,
)]
#[serde(rename_all = "UPPERCASE")]
#[strum(serialize_all = "UPPERCASE")]
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

#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Default,
  PartialEq,
  Eq,
  Builder,
)]
pub struct TerminationSignalLabel {
  #[builder(default)]
  pub signal: TerminationSignal,
  #[builder(default)]
  pub label: String,
}

pub fn term_signal_labels_from_str(
  value: &str,
) -> anyhow::Result<Vec<TerminationSignalLabel>> {
  let res = value
    .split('\n')
    .map(|line| line.trim())
    .enumerate()
    .filter(|(_, line)| !line.starts_with('#'))
    .map(|(i, line)| {
      let mut split = line.split('=');
      let signal = split
        .next()
        .with_context(|| format!("line {i} does not have signal"))?
        .trim()
        .parse::<TerminationSignal>()
        .with_context(|| {
          format!("line {i} does not have valid signal")
        })?;
      // remove trailing comments
      let mut label_split = split
        .next()
        .with_context(|| format!("line {i} does not have label"))?
        .split('#');
      let label = label_split
        .next()
        .with_context(|| format!("line {i} does not have label"))?
        .trim()
        .to_string();
      anyhow::Ok(TerminationSignalLabel { signal, label })
    })
    .collect::<anyhow::Result<Vec<_>>>()?;
  Ok(res)
}

pub fn term_labels_deserializer<'de, D>(
  deserializer: D,
) -> Result<Vec<TerminationSignalLabel>, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(TermSignalLabelVisitor)
}

pub fn option_term_labels_deserializer<'de, D>(
  deserializer: D,
) -> Result<Option<Vec<TerminationSignalLabel>>, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(OptionTermSignalLabelVisitor)
}

struct TermSignalLabelVisitor;

impl<'de> Visitor<'de> for TermSignalLabelVisitor {
  type Value = Vec<TerminationSignalLabel>;

  fn expecting(
    &self,
    formatter: &mut std::fmt::Formatter,
  ) -> std::fmt::Result {
    write!(formatter, "string or Vec<TerminationSignalLabel>")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    term_signal_labels_from_str(v)
      .map_err(|e| serde::de::Error::custom(format!("{e:#}")))
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::SeqAccess<'de>,
  {
    #[derive(Deserialize)]
    struct TermSignalLabelInner {
      signal: TerminationSignal,
      label: String,
    }

    impl From<TermSignalLabelInner> for TerminationSignalLabel {
      fn from(value: TermSignalLabelInner) -> Self {
        Self {
          signal: value.signal,
          label: value.label,
        }
      }
    }

    let res = Vec::<TermSignalLabelInner>::deserialize(
      SeqAccessDeserializer::new(seq),
    )?
    .into_iter()
    .map(Into::into)
    .collect();
    Ok(res)
  }
}

struct OptionTermSignalLabelVisitor;

impl<'de> Visitor<'de> for OptionTermSignalLabelVisitor {
  type Value = Option<Vec<TerminationSignalLabel>>;

  fn expecting(
    &self,
    formatter: &mut std::fmt::Formatter,
  ) -> std::fmt::Result {
    write!(formatter, "null or string or Vec<TerminationSignalLabel>")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    TermSignalLabelVisitor.visit_str(v).map(Some)
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::SeqAccess<'de>,
  {
    TermSignalLabelVisitor.visit_seq(seq).map(Some)
  }

  fn visit_none<E>(self) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(None)
  }
}

#[typeshare]
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct DeploymentActionState {
  pub deploying: bool,
  pub stopping: bool,
  pub starting: bool,
  pub removing: bool,
  pub renaming: bool,
}

#[typeshare]
pub type DeploymentQuery = ResourceQuery<DeploymentQuerySpecifics>;

#[typeshare]
#[derive(
  Debug, Clone, Default, Serialize, Deserialize, DefaultBuilder,
)]
pub struct DeploymentQuerySpecifics {
  #[serde(default)]
  pub server_ids: Vec<String>,

  #[serde(default)]
  pub build_ids: Vec<String>,
}

impl super::resource::AddFilters for DeploymentQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    if !self.server_ids.is_empty() {
      filters
        .insert("config.server_id", doc! { "$in": &self.server_ids });
    }
    if !self.build_ids.is_empty() {
      filters.insert("config.image.type", "Build");
      filters.insert(
        "config.image.params.build_id",
        doc! { "$in": &self.build_ids },
      );
    }
  }
}
