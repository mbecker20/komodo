use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct TerminationSignalLabel {
    pub signal: TerminationSignal,
    pub label: String,
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

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Conversion {
    pub local: String,
    pub container: String,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone, Copy, Default)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone, Copy, Default)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Hash, Eq, Clone, Copy, Default)]
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
