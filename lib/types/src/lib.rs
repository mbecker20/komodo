use std::{collections::HashMap, path::PathBuf};

use async_timing_util::{unix_timestamp_ms, Timelength};
use mungos::ObjectId;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub mod traits;

pub const PERIPHERY_BUILDER_BUSY: &str = "builder is busy";

pub type UserId = String;
pub type ServerId = String;
pub type DeploymentId = String;
pub type BuildId = String;

pub type GithubUsername = String;
pub type GithubToken = String;
pub type GithubAccounts = HashMap<GithubUsername, GithubToken>;

pub type DockerUsername = String;
pub type DockerToken = String;
pub type DockerAccounts = HashMap<DockerUsername, DockerToken>;

pub type SecretsMap = HashMap<String, String>; // these are used for injection into deployments run commands

pub type PermissionsMap = HashMap<UserId, PermissionLevel>;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub enabled: bool,
    pub admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,

    // used with auth
    #[serde(default)]
    pub secrets: Vec<ApiSecret>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ApiSecret {
    pub name: String,
    pub hash: String,
    pub created_at: i64,
    pub expires: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub address: String,
    #[serde(default)]
    pub permissions: PermissionsMap,
    #[serde(default)]
    pub to_notify: Vec<String>,
    #[serde(default = "default_cpu_alert")]
    pub cpu_alert: f64,
    #[serde(default = "default_mem_alert")]
    pub mem_alert: f64,
    #[serde(default = "default_disk_alert")]
    pub disk_alert: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_interval: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            id: None,
            name: String::new(),
            address: String::new(),
            permissions: HashMap::new(),
            to_notify: Vec::new(),
            cpu_alert: default_cpu_alert(),
            mem_alert: default_mem_alert(),
            disk_alert: default_disk_alert(),
            stats_interval: None,
            region: None,
            instance_id: None,
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Deployment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String, // must be formatted to be compat with docker
    pub server_id: ServerId,
    pub permissions: PermissionsMap,
    pub docker_run_args: DockerRunArgs,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_core: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_id: Option<BuildId>,

    // deployment repo related
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub github_account: Option<String>,
    pub on_clone: Option<Command>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Build {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub permissions: PermissionsMap,
    pub server_id: String, // server which this image should be built on
    pub version: Version,

    // git related
    pub repo: Option<String>,
    pub branch: Option<String>,
    pub github_account: Option<String>,
    pub on_clone: Option<Command>,

    // build related
    pub pre_build: Option<Command>,
    pub docker_build_args: Option<DockerBuildArgs>,
    pub docker_account: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildRecord {
    pub start_ts: i64,
    pub end_ts: i64,
    pub successful: bool,
    pub logs: Vec<Log>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Update {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub target: UpdateTarget,
    pub operation: Operation,
    pub log: Vec<Log>,
    pub start_ts: i64,
    pub end_ts: Option<i64>,
    pub status: UpdateStatus,
    pub is_error: bool,
    pub operator: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Procedure {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub procedure: Vec<Operation>,
    pub permissions: PermissionsMap,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DockerBuildArgs {
    pub build_path: String,
    pub dockerfile_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DockerRunArgs {
    pub image: String,
    pub ports: Vec<Conversion>,
    pub volumes: Vec<Conversion>,
    pub environment: Vec<EnvironmentVar>,
    pub network: Option<String>,
    pub restart: RestartMode,
    pub post_image: Option<String>,
    pub container_user: Option<String>,
    pub docker_account: Option<String>, // the username of the dockerhub account
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BasicContainerInfo {
    pub name: String,
    pub id: String,
    pub state: DockerContainerState,
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Log {
    pub stage: String,
    pub command: String,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub start_ts: i64,
    pub end_ts: i64,
}

impl Log {
    pub fn simple(msg: String) -> Log {
        let ts = unix_timestamp_ms() as i64;
        Log {
            stdout: msg,
            success: true,
            start_ts: ts,
            end_ts: ts,
            ..Default::default()
        }
    }

    pub fn error(msg: String) -> Log {
        let ts = unix_timestamp_ms() as i64;
        Log {
            stderr: msg,
            start_ts: ts,
            end_ts: ts,
            success: false,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command {
    pub path: String,
    pub command: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}", self.major, self.minor)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Conversion {
    pub local: String,
    pub container: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct EnvironmentVar {
    pub variable: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OauthCredentials {
    pub id: String,
    pub secret: String,
}

#[derive(Deserialize, Debug, Clone)]
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
    pub github_oauth: OauthCredentials,
    pub github_webhook_secret: Option<String>,

    // mongo config
    pub mongo: MongoConfig,
}

fn default_core_port() -> u16 {
    9000
}

fn default_jwt_valid_for() -> Timelength {
    Timelength::OneWeek
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug)]
pub struct PeripheryConfig {
    #[serde(default = "default_periphery_port")]
    pub port: u16,
    #[serde(default)]
    pub is_builder: bool,
    #[serde(default)]
    pub docker_accounts: DockerAccounts,
    #[serde(default)]
    pub github_accounts: GithubAccounts,
    #[serde(default)]
    pub secrets: SecretsMap,
    #[serde(default = "default_repo_dir")]
    pub repo_dir: String,
}

fn default_periphery_port() -> u16 {
    9001
}

fn default_repo_dir() -> String {
    "/repos".to_string()
}

#[derive(Deserialize, Debug)]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemStats {
    pub cpu: f32,       // in %
    pub mem_used: f64,  // in MB
    pub mem_total: f64, // in MB
    pub disk: DiskUsage,
    pub networks: Vec<SystemNetwork>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiskUsage {
    pub used: f64,  // in GB
    pub total: f64, // in GB
    pub read: f64,  // in kB
    pub write: f64, // in kB
    pub disks: Vec<SingleDiskUsage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleDiskUsage {
    pub mount: PathBuf,
    pub used: f64,  // in GB
    pub total: f64, // in GB
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SystemNetwork {
    pub name: String,
    pub recieved: f64,    // in kB
    pub transmitted: f64, // in kB
}

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AccountType {
    Github,
    Docker,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "id")]
pub enum UpdateTarget {
    System,
    Build(BuildId),
    Deployment(DeploymentId),
    Server(ServerId),
}

impl Default for UpdateTarget {
    fn default() -> Self {
        UpdateTarget::System
    }
}

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

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
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
    DeployDeployment,
    StopDeployment,
    StartDeployment,
    PullDeployment,
    RecloneDeployment,
}

impl Default for Operation {
    fn default() -> Self {
        Operation::None
    }
}

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
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

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PermissionsTarget {
    Server,
    Deployment,
    Build,
}

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

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
pub enum RestartMode {
    #[serde(rename = "no")]
    NoRestart,
    #[serde(rename = "on-failure")]
    OnFailure,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "unless-stopped")]
    UnlessStopped,
}

impl Default for RestartMode {
    fn default() -> RestartMode {
        RestartMode::NoRestart
    }
}
