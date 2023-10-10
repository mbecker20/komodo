use derive_builder::Builder;
use derive_variants::EnumVariants;
use mungos::{derive::MungosIndexed, mongodb::bson::doc};
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

use super::{
    resource::{Resource, ResourceListItem},
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
    pub state: DockerContainerState,
    pub status: Option<String>,
    pub image: String,
    pub server_id: String,
    pub build_id: Option<String>,
}

#[typeshare]
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Builder,
    Partial,
    MungosIndexed,
)]
#[partial_derive(Serialize, Deserialize, Debug, Clone)]
#[skip_serializing_none]
#[partial_from]
pub struct DeploymentConfig {
    #[serde(default)]
    #[builder(default)]
    #[index]
    pub server_id: String,

    #[serde(default = "default_send_alerts")]
    #[builder(default = "default_send_alerts()")]
    #[partial_default(default_send_alerts())]
    pub send_alerts: bool,

    #[serde(default)]
    #[builder(default)]
    pub image: DeploymentImage,

    #[serde(default)]
    #[builder(default)]
    pub skip_secret_interp: bool,

    #[serde(default)]
    #[builder(default)]
    pub redeploy_on_build: bool,

    #[serde(default = "default_term_signal_labels")]
    #[builder(default = "default_term_signal_labels()")]
    #[partial_default(default_term_signal_labels())]
    pub term_signal_labels: Vec<TerminationSignalLabel>,

    #[serde(default)]
    #[builder(default)]
    pub termination_signal: TerminationSignal,

    #[serde(default = "default_termination_timeout")]
    #[builder(default = "default_termination_timeout()")]
    #[partial_default(default_termination_timeout())]
    pub termination_timeout: i32,

    #[serde(default)]
    #[builder(default)]
    pub ports: Vec<Conversion>,

    #[serde(default)]
    #[builder(default)]
    pub volumes: Vec<Conversion>,

    #[serde(default)]
    #[builder(default)]
    pub environment: Vec<EnvironmentVar>,

    #[serde(default = "default_network")]
    #[builder(default = "default_network()")]
    #[partial_default(default_network())]
    pub network: String,

    #[serde(default)]
    #[builder(default)]
    pub restart: RestartMode,

    #[serde(default)]
    #[builder(default)]
    pub process_args: String, // empty is no post image

    #[serde(default)]
    #[builder(default)]
    pub container_user: String, // empty is no container user

    #[serde(default)]
    #[builder(default)]
    pub extra_args: Vec<String>,

    #[serde(default)]
    #[builder(default)]
    pub docker_account: String, // the username of the dockerhub account. empty if no account.
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

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Clone, MungosIndexed, EnumVariants,
)]
#[variant_derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    Display,
    EnumString
)]
#[serde(tag = "type", content = "params")]
pub enum DeploymentImage {
    Image { image: String },
    Build { build_id: String, version: Version },
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
    Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
pub struct Conversion {
    pub local: String,
    pub container: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContainerSummary {
    pub name: String,
    pub id: String,
    pub image: String,
    pub state: DockerContainerState,
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
    MungosIndexed,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
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
    MungosIndexed,
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
    MungosIndexed,
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

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeploymentActionState {
    pub deploying: bool,
    pub stopping: bool,
    pub starting: bool,
    pub removing: bool,
    pub updating: bool,
    pub renaming: bool,
    pub deleting: bool,
}
