use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use diff::Diff;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

use crate::{diff::*, Command, EnvironmentVar, PermissionsMap, Version};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Diff, Builder)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct Deployment {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    #[builder(setter(skip))]
    pub id: String,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub name: String, // must be formatted to be compat with docker

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub server_id: String,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing)]))]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[diff(attr(#[serde(skip_serializing_if = "docker_run_args_diff_no_change")]))]
    pub docker_run_args: DockerRunArgs,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub build_id: Option<String>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub build_version: Option<Version>,

    // deployment repo related
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub repo: Option<String>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub branch: Option<String>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub github_account: Option<String>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub on_clone: Option<Command>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub on_pull: Option<Command>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub repo_mount: Option<Conversion>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[diff(attr(#[serde(skip)]))]
    #[builder(setter(skip))]
    pub created_at: String,
    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    #[builder(setter(skip))]
    pub updated_at: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DeploymentWithContainerState {
    pub deployment: Deployment,
    pub state: DockerContainerState,
    pub container: Option<BasicContainerInfo>,
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
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Diff, Builder)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub struct DockerRunArgs {
    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub image: String,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "vec_diff_no_change")]))]
    pub ports: Vec<Conversion>,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "vec_diff_no_change")]))]
    pub volumes: Vec<Conversion>,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "vec_diff_no_change")]))]
    pub environment: Vec<EnvironmentVar>,

    #[serde(default = "default_network")]
    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub network: String,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "restart_mode_diff_no_change")]))]
    pub restart: RestartMode,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub post_image: Option<String>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub container_user: Option<String>,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "vec_diff_no_change")]))]
    pub extra_args: Vec<String>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
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

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicContainerInfo {
    pub name: String,
    pub id: String,
    pub state: DockerContainerState,
    pub status: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Diff)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub struct Conversion {
    pub local: String,
    pub container: String,
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
#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DockerContainerState {
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

impl Default for DockerContainerState {
    fn default() -> Self {
        Self::Unknown
    }
}

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy, Diff,
)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub enum RestartMode {
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

impl Default for RestartMode {
    fn default() -> RestartMode {
        RestartMode::NoRestart
    }
}
