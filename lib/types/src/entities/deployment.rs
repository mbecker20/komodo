use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use mungos::MungosIndexed;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

use crate::{i64_is_zero, I64};

use super::{EnvironmentVar, PermissionsMap};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, MungosIndexed)]
pub struct Deployment {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    #[builder(setter(skip))]
    pub id: String,

    #[unique_index]
    pub name: String,

    #[serde(default)]
    #[builder(default)]
    pub description: String,

    #[serde(default)]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[serde(default, skip_serializing_if = "i64_is_zero")]
    #[builder(setter(skip))]
    pub created_at: I64,

    #[serde(default)]
    #[builder(setter(skip))]
    pub updated_at: I64,

    pub config: DeploymentConfig,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial, MungosIndexed)]
#[partial_derive(Serialize, Deserialize, Debug, Clone)]
#[skip_serializing_none]
pub struct DeploymentConfig {
    #[serde(default)]
    #[builder(default)]
    #[index]
    pub server_id: String,

    #[serde(default)]
    #[builder(default)]
    #[index]
    pub build_id: String,

    #[serde(default)]
    #[builder(default)]
    pub image: String,

    #[serde(default)]
    #[builder(default)]
    pub skip_secret_interp: bool,

    #[serde(default)]
    #[builder(default)]
    pub redeploy_on_build: bool,

    #[serde(default = "default_term_signal_labels")]
    #[builder(default = "default_term_signal_labels()")]
    pub term_signal_labels: Vec<TerminationSignalLabel>,

    #[serde(default)]
    #[builder(default)]
    pub termination_signal: TerminationSignal,

    #[serde(default = "default_termination_timeout")]
    #[builder(default = "default_termination_timeout()")]
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
    pub network: String,

    #[serde(default)]
    #[builder(default)]
    pub restart: RestartMode,

    #[serde(default)]
    #[builder(default)]
    pub post_image: String, // empty is no post image

    #[serde(default)]
    #[builder(default)]
    pub container_user: String, // empty is no container user

    #[serde(default)]
    #[builder(default)]
    pub extra_args: Vec<String>,

    #[serde(default)]
    #[builder(default)]
    pub docker_account: String, // the username of the dockerhub account. empty if no account.

    #[serde(default)]
    #[builder(default)]
    pub tags: Vec<String>,
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

impl From<PartialDeploymentConfig> for DeploymentConfig {
    fn from(value: PartialDeploymentConfig) -> DeploymentConfig {
        DeploymentConfig {
            server_id: value.server_id.unwrap_or_default(),
            build_id: value.build_id.unwrap_or_default(),
            image: value.image.unwrap_or_default(),
            skip_secret_interp: value.skip_secret_interp.unwrap_or_default(),
            redeploy_on_build: value.redeploy_on_build.unwrap_or_default(),
            term_signal_labels: value
                .term_signal_labels
                .unwrap_or(default_term_signal_labels()),
            termination_signal: value.termination_signal.unwrap_or_default(),
            termination_timeout: value
                .termination_timeout
                .unwrap_or(default_termination_timeout()),
            ports: value.ports.unwrap_or_default(),
            volumes: value.volumes.unwrap_or_default(),
            environment: value.environment.unwrap_or_default(),
            network: value.network.unwrap_or(default_network()),
            restart: value.restart.unwrap_or_default(),
            post_image: value.post_image.unwrap_or_default(),
            container_user: value.container_user.unwrap_or_default(),
            extra_args: value.extra_args.unwrap_or_default(),
            docker_account: value.docker_account.unwrap_or_default(),
            tags: value.tags.unwrap_or_default(),
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Conversion {
    pub local: String,
    pub container: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicContainerInfo {
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
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Builder)]
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
    pub pulling: bool,
    pub recloning: bool,
    pub updating: bool,
    pub renaming: bool,
}
