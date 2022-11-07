use std::collections::HashMap;

use async_timing_util::Timelength;
use mungos::ObjectId;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub type PermissionsMap = HashMap<String, PermissionLevel>;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Server {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub host: String,
    pub permissions: PermissionsMap,
    pub to_notify: Vec<String>,
    pub cpu_alert: f64,
    pub mem_alert: f64,
    pub disk_alert: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passkey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_core: Option<bool>,
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
            host: String::new(),
            permissions: HashMap::new(),
            to_notify: Vec::new(),
            cpu_alert: 50.0,
            mem_alert: 75.0,
            disk_alert: 75.0,
            passkey: None,
            is_core: None,
            stats_interval: None,
            region: None,
            instance_id: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Build {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub permissions: PermissionsMap,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Update {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub entity_type: EntityType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    pub operation: Operation,
    pub command: String,
    pub log: Vec<Log>,
    pub ts: i64,
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
    pub image: Option<String>,
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
    pub state: ContainerState,
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Log {
    pub stage: String,
    pub stdout: String,
    pub stderr: String,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Permission {
    pub entity_type: EntityType,
    pub id: String,
    pub level: PermissionLevel,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct OauthCredentials {
    pub id: String,
    pub secret: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct CoreConfig {
    // port the core web server runs on
    pub port: u16,

    // default periphery passkey
    pub passkey: String,

    // docker integration
    pub docker_accounts: DockerAccounts,

    // github integration
    pub github_accounts: GithubAccounts,
    pub github_oauth: OauthCredentials,
    pub github_webhook_secret: String,

    // jwt config
    pub jwt_secret: String,
    pub jwt_valid_for: Timelength,

    // integration with slack app
    pub slack_url: Option<String>,

    //mongo config
    pub mongo_uri: String,
    #[serde(default = "default_core_mongo_app_name")]
    pub mongo_app_name: String,
    #[serde(default = "default_core_mongo_db_name")]
    pub mongo_db_name: String,
}

fn default_core_mongo_app_name() -> String {
    "monitor_core".to_string()
}

fn default_core_mongo_db_name() -> String {
    "monitor".to_string()
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct PeripherySecrets {
    pub passkey: String,
    pub docker_accounts: DockerAccounts,
    pub github_accounts: GithubAccounts,
}

#[derive(Deserialize, Debug)]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EntityType {
    System,
    Build,
    Deployment,
    Server,
}

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Operation {
    // server
    PruneImagesServer,
    PruneContainersServer,
    PruneNetworksServer,

    // build
    BuildBuild,
    RecloneBuild,

    // deployment
    DeployDeployment,
    StopDeployment,
    StartDeployment,
    PullDeployment,
    RecloneDeployment,
}

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PermissionLevel {
    Read,
    Write,
}

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ContainerState {
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
