use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use async_timing_util::Timelength;
use bson::serde_helpers::hex_string_as_object_id;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use diff::{Diff, HashMapDiff, OptionDiff, VecDiff};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub use bollard::service::{ImageSummary, Network};
use typeshare::typeshare;

pub mod traits;

pub const PERIPHERY_BUILDER_BUSY: &str = "builder is busy";

pub type GithubUsername = String;
pub type GithubToken = String;
pub type GithubAccounts = HashMap<GithubUsername, GithubToken>;

pub type DockerUsername = String;
pub type DockerToken = String;
pub type DockerAccounts = HashMap<DockerUsername, DockerToken>;

pub type SecretsMap = HashMap<String, String>; // these are used for injection into deployments run commands

#[typeshare]
pub type PermissionsMap = HashMap<String, PermissionLevel>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Diff)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct User {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub id: String,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub username: String,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub enabled: bool,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub admin: bool,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub create_server_permissions: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub avatar: Option<String>,

    // used with auth
    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "vec_diff_no_change")]))]
    pub secrets: Vec<ApiSecret>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub github_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub google_id: Option<String>,

    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    pub created_at: String,
    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    pub updated_at: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Diff)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct ApiSecret {
    pub name: String,
    pub hash: String,
    pub created_at: String,
    pub expires: Option<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Diff, Builder)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct Server {
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
    pub name: String,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub address: String,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "hashmap_diff_no_change")]))]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "vec_diff_no_change")]))]
    pub to_notify: Vec<String>, // slack users to notify

    #[serde(default = "default_cpu_alert")]
    pub cpu_alert: f64,
    #[serde(default = "default_mem_alert")]
    pub mem_alert: f64,
    #[serde(default = "default_disk_alert")]
    pub disk_alert: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub stats_interval: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub region: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub instance_id: Option<String>,

    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    #[builder(setter(skip))]
    pub created_at: String,
    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    #[builder(setter(skip))]
    pub updated_at: String,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            address: Default::default(),
            permissions: Default::default(),
            to_notify: Default::default(),
            cpu_alert: default_cpu_alert(),
            mem_alert: default_mem_alert(),
            disk_alert: default_disk_alert(),
            stats_interval: Default::default(),
            region: Default::default(),
            instance_id: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
        }
    }
}

fn default_cpu_alert() -> f64 {
    50.0
}

fn default_mem_alert() -> f64 {
    75.0
}

fn default_disk_alert() -> f64 {
    75.0
}

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
    #[diff(attr(#[serde(skip_serializing_if = "hashmap_diff_no_change")]))]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[diff(attr(#[serde(skip_serializing_if = "docker_run_args_diff_no_change")]))]
    pub docker_run_args: DockerRunArgs,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub build_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub build_version: Option<Version>,

    // deployment repo related
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub repo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub branch: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub github_account: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub on_clone: Option<Command>,

    #[serde(default)]
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
pub struct DeploymentWithContainer {
    pub deployment: Deployment,
    pub container: Option<BasicContainerInfo>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Diff, Builder)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct Build {
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
    pub name: String,

    #[diff(attr(#[serde(skip_serializing_if = "hashmap_diff_no_change")]))]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub server_id: String, // server which this image should be built on

    pub version: Version,

    // git related
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub repo: Option<String>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub branch: Option<String>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub github_account: Option<String>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub on_clone: Option<Command>,

    // build related
    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub pre_build: Option<Command>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub docker_build_args: Option<DockerBuildArgs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub docker_account: Option<String>,

    #[serde(default)]
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
pub struct Update {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    pub id: String,
    pub target: UpdateTarget,
    pub operation: Operation,
    pub logs: Vec<Log>,
    pub start_ts: String,
    pub end_ts: Option<String>,
    pub status: UpdateStatus,
    pub success: bool,
    pub operator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Diff, Builder)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct Procedure {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    #[builder(setter(skip))]
    pub id: String,
    pub name: String,
    pub stages: Vec<ProcedureStage>,
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    #[builder(setter(skip))]
    pub created_at: String,
    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    #[builder(setter(skip))]
    pub updated_at: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Diff)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct ProcedureStage {
    pub operation: ProcedureOperation,
    pub target_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Diff, Builder)]
#[diff(attr(#[derive(Debug, Serialize, PartialEq)]))]
pub struct DockerBuildArgs {
    pub build_path: String,
    pub dockerfile_path: Option<String>,
    pub build_args: Vec<EnvironmentVar>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Diff, Builder)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub struct DockerRunArgs {
    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub image: String,

    #[diff(attr(#[serde(skip_serializing_if = "vec_diff_no_change")]))]
    pub ports: Vec<Conversion>,

    #[diff(attr(#[serde(skip_serializing_if = "vec_diff_no_change")]))]
    pub volumes: Vec<Conversion>,

    #[diff(attr(#[serde(skip_serializing_if = "vec_diff_no_change")]))]
    pub environment: Vec<EnvironmentVar>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub network: Option<String>,

    #[diff(attr(#[serde(skip_serializing_if = "restart_mode_diff_no_change")]))]
    pub restart: RestartMode,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub post_image: Option<String>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub container_user: Option<String>,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub docker_account: Option<String>, // the username of the dockerhub account
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
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Log {
    pub stage: String,
    pub command: String,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub start_ts: String,
    pub end_ts: String,
}

impl Log {
    pub fn simple(stage: &str, msg: String) -> Log {
        let ts = monitor_timestamp();
        Log {
            stage: stage.to_string(),
            stdout: msg,
            success: true,
            start_ts: ts.clone(),
            end_ts: ts,
            ..Default::default()
        }
    }

    pub fn error(stage: &str, msg: String) -> Log {
        let ts = monitor_timestamp();
        Log {
            stage: stage.to_string(),
            stderr: msg,
            start_ts: ts.clone(),
            end_ts: ts,
            success: false,
            ..Default::default()
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Diff)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub struct Command {
    pub path: String,
    pub command: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Diff)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl Version {
    pub fn increment(&mut self) {
        self.patch += 1;
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Diff)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub struct Conversion {
    pub local: String,
    pub container: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Diff)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub struct EnvironmentVar {
    pub variable: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OauthCredentials {
    pub id: String,
    pub secret: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoreConfig {
    // port the core web server runs on
    #[serde(default = "default_core_port")]
    pub port: u16,

    // jwt config
    pub jwt_secret: String,
    #[serde(default = "default_jwt_valid_for")]
    pub jwt_valid_for: Timelength,

    // integration with slack app
    pub slack_url: Option<String>,

    // github integration
    pub github_webhook_secret: String,
    pub github_oauth: OauthCredentials,

    // mongo config
    pub mongo: MongoConfig,
}

fn default_core_port() -> u16 {
    9000
}

fn default_jwt_valid_for() -> Timelength {
    Timelength::OneWeek
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MongoConfig {
    pub uri: String,
    #[serde(default = "default_core_mongo_app_name")]
    pub app_name: String,
    #[serde(default = "default_core_mongo_db_name")]
    pub db_name: String,
}

fn default_core_mongo_app_name() -> String {
    "monitor_core".to_string()
}

fn default_core_mongo_db_name() -> String {
    "monitor".to_string()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PeripheryConfig {
    #[serde(default = "default_periphery_port")]
    pub port: u16,
    #[serde(default = "default_repo_dir")]
    pub repo_dir: String,
    #[serde(default)]
    pub secrets: SecretsMap,
    #[serde(default)]
    pub github_accounts: GithubAccounts,
    #[serde(default)]
    pub docker_accounts: DockerAccounts,
}

fn default_periphery_port() -> u16 {
    8000
}

fn default_repo_dir() -> String {
    "/repos".to_string()
}

#[typeshare]
#[derive(Deserialize, Debug)]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemStats {
    pub cpu_perc: f32,     // in %
    pub mem_used_gb: f64,  // in GB
    pub mem_total_gb: f64, // in GB
    pub disk: DiskUsage,
    pub networks: Vec<SystemNetwork>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct DiskUsage {
    pub used_gb: f64,  // in GB
    pub total_gb: f64, // in GB
    pub read_kb: f64,  // in kB
    pub write_kb: f64, // in kB
    pub disks: Vec<SingleDiskUsage>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct SingleDiskUsage {
    pub mount: PathBuf,
    pub used_gb: f64,  // in GB
    pub total_gb: f64, // in GB
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug)]
pub struct SystemNetwork {
    pub name: String,
    pub recieved_kb: f64,    // in kB
    pub transmitted_kb: f64, // in kB
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AccountType {
    Github,
    Docker,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "id")]
pub enum UpdateTarget {
    System,
    Build(String),
    Deployment(String),
    Server(String),
    Procedure(String),
}

impl Default for UpdateTarget {
    fn default() -> Self {
        UpdateTarget::System
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum UpdateStatus {
    Queued,
    InProgress,
    Complete,
}

impl Default for UpdateStatus {
    fn default() -> Self {
        UpdateStatus::Complete
    }
}

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy, Diff,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub enum Operation {
    // do nothing
    None,

    // server
    CreateServer,
    UpdateServer,
    DeleteServer,
    PruneImagesServer,
    PruneContainersServer,
    PruneNetworksServer,

    // build
    CreateBuild,
    UpdateBuild,
    DeleteBuild,
    BuildBuild,
    RecloneBuild,

    // deployment
    CreateDeployment,
    UpdateDeployment,
    DeleteDeployment,
    DeployContainer,
    StopContainer,
    StartContainer,
    RemoveContainer,
    PullDeployment,
    RecloneDeployment,

    // procedure
    CreateProcedure,
    UpdateProcedure,
    DeleteProcedure,
}

impl Default for Operation {
    fn default() -> Self {
        Operation::None
    }
}

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy, Diff,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub enum ProcedureOperation {
    // do nothing
    None,

    // server
    PruneImagesServer,
    PruneContainersServer,
    PruneNetworksServer,

    // build
    BuildBuild,
    RecloneBuild,

    // deployment
    DeployContainer,
    StopContainer,
    StartContainer,
    RemoveContainer,
    PullDeployment,
    RecloneDeployment,

    // procedure
    RunProcedure,
}

impl Default for ProcedureOperation {
    fn default() -> Self {
        ProcedureOperation::None
    }
}

#[typeshare]
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Display,
    EnumString,
    Hash,
    Clone,
    Copy,
    Diff,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub enum PermissionLevel {
    None,
    Read,
    Write,
}

impl Default for PermissionLevel {
    fn default() -> Self {
        PermissionLevel::None
    }
}

impl Default for &PermissionLevel {
    fn default() -> Self {
        &PermissionLevel::None
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PermissionsTarget {
    Server,
    Deployment,
    Build,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DockerContainerState {
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

fn option_diff_no_change<T: Diff>(option_diff: &OptionDiff<T>) -> bool
where
    <T as Diff>::Repr: PartialEq,
{
    option_diff == &OptionDiff::NoChange || option_diff == &OptionDiff::None
}

fn vec_diff_no_change<T: Diff>(vec_diff: &VecDiff<T>) -> bool {
    vec_diff.0.is_empty()
}

fn hashmap_diff_no_change<T: Diff>(hashmap_diff: &HashMapDiff<String, T>) -> bool {
    hashmap_diff.altered.is_empty() && hashmap_diff.removed.is_empty()
}

fn docker_run_args_diff_no_change(dra: &DockerRunArgsDiff) -> bool {
    dra.image.is_none()
        && option_diff_no_change(&dra.container_user)
        && option_diff_no_change(&dra.docker_account)
        && option_diff_no_change(&dra.network)
        && option_diff_no_change(&dra.post_image)
        && vec_diff_no_change(&dra.environment)
        && vec_diff_no_change(&dra.ports)
        && vec_diff_no_change(&dra.volumes)
        && restart_mode_diff_no_change(&dra.restart)
}

fn restart_mode_diff_no_change(restart_mode: &RestartModeDiff) -> bool {
    restart_mode == &RestartModeDiff::NoChange
}

pub fn monitor_timestamp() -> String {
    Utc::now().to_rfc3339()
}

pub fn unix_from_monitor_ts(ts: &str) -> anyhow::Result<i64> {
    Ok(DateTime::parse_from_rfc3339(ts)
        .context("failed to parse rfc3339 timestamp")?
        .timestamp_millis())
}
