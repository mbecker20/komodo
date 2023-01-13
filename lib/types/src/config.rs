use std::{collections::HashMap, net::IpAddr, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::Timelength;

pub type GithubUsername = String;
pub type GithubToken = String;
pub type GithubAccounts = HashMap<GithubUsername, GithubToken>;

pub type DockerUsername = String;
pub type DockerToken = String;
pub type DockerAccounts = HashMap<DockerUsername, DockerToken>;

pub type SecretsMap = HashMap<String, String>; // these are used for injection into deployments run commands

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoreConfig {
    // the host to use with oauth redirect url, whatever host the user hits to access monitor. eg 'https://monitor.mogh.tech'
    pub host: String,

    // port the core web server runs on
    #[serde(default = "default_core_port")]
    pub port: u16,

    // jwt config
    pub jwt_secret: String,
    #[serde(default = "default_jwt_valid_for")]
    pub jwt_valid_for: Timelength,

    // interval at which to collect server stats and alert for out of bounds
    pub monitoring_interval: Timelength,

    // daily utc offset in hours to run daily update. eg 8:00 eastern time is 13:00 UTC, so offset should be 13. default of 0 runs at UTC midnight.
    #[serde(default)]
    pub daily_offset_hours: u8,

    // number of days to keep stats, or 0 to disable pruning. stats older than this number of days are deleted on a daily cycle
    #[serde(default)]
    pub keep_stats_for_days: u64,

    // used to verify validity from github webhooks
    pub github_webhook_secret: String,

    // integration with slack app
    pub slack_url: Option<String>,

    // enable login with local auth
    pub local_auth: bool,

    // github integration
    pub github_oauth: OauthCredentials,

    // google integration
    pub google_oauth: OauthCredentials,

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
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PeripheryConfig {
    #[serde(default = "default_periphery_port")]
    pub port: u16,
    #[serde(default = "default_repo_dir")]
    pub repo_dir: PathBuf,
    #[serde(default = "default_stats_refresh_interval")]
    pub stats_polling_rate: Timelength,
    #[serde(default)]
    pub allowed_ips: Vec<IpAddr>,
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

fn default_repo_dir() -> PathBuf {
    "/repos".parse().unwrap()
}

fn default_stats_refresh_interval() -> Timelength {
    Timelength::FiveSeconds
}
