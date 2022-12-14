use std::collections::HashMap;

use async_timing_util::Timelength;
use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct OauthCredentials {
    pub id: String,
    pub secret: String,
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

pub type GithubUsername = String;
pub type GithubToken = String;
pub type GithubAccounts = HashMap<GithubUsername, GithubToken>;

pub type DockerUsername = String;
pub type DockerToken = String;
pub type DockerAccounts = HashMap<DockerUsername, DockerToken>;

pub type SecretsMap = HashMap<String, String>;

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

#[derive(
    Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy,
)]
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
