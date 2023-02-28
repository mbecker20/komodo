use std::{collections::HashMap, net::IpAddr};

use async_timing_util::Timelength;
use serde_derive::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CoreConfig {
    // the host to use with oauth redirect url, whatever host the user hits to access monitor. eg 'https://monitor.mogh.tech'
    pub host: String,

    // port the core web server runs on
    #[serde(default = "default_core_port")]
    pub port: u16,

    // daily utc offset in hours to run daily update. eg 8:00 eastern time is 13:00 UTC, so offset should be 13. default of 0 runs at UTC midnight.
    #[serde(default)]
    pub daily_offset_hours: u8,

    // number of days to keep stats around, or 0 to disable pruning. stats older than this number of days are deleted daily
    #[serde(default)]
    pub keep_stats_for_days: u64, // 0 means never prune

    pub jwt_secret: String,
    #[serde(default = "default_jwt_valid_for")]
    pub jwt_valid_for: Timelength,

    // interval at which to collect server stats and alert for out of bounds
    pub monitoring_interval: Timelength,

    // used to verify validity from github webhooks
    pub github_webhook_secret: String,

    // used to form the frontend listener url, if None will use 'host'.
    pub github_webhook_base_url: Option<String>,

    // sent in auth header with req to periphery
    pub passkey: String,

    // integration with slack app
    pub slack_url: Option<String>,

    // enable login with local auth
    pub local_auth: bool,

    pub mongo: MongoConfig,

    #[serde(default)]
    pub github_oauth: OauthCredentials,

    #[serde(default)]
    pub google_oauth: OauthCredentials,

    #[serde(default)]
    pub aws: AwsBuilderConfig,
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AwsBuilderConfig {
    #[serde(skip_serializing)]
    pub access_key_id: String,

    #[serde(skip_serializing)]
    pub secret_access_key: String,

    pub default_ami_id: String,
    pub default_subnet_id: String,
    pub default_key_pair_name: String,

    #[serde(default)]
    pub available_ami_accounts: AvailableAmiAccounts,

    #[serde(default = "default_aws_region")]
    pub default_region: String,

    #[serde(default = "default_volume_gb")]
    pub default_volume_gb: i32,

    #[serde(default = "default_instance_type")]
    pub default_instance_type: String,

    #[serde(default)]
    pub default_security_group_ids: Vec<String>,

    #[serde(default)]
    pub default_assign_public_ip: bool,
}

fn default_aws_region() -> String {
    String::from("us-east-1")
}

fn default_volume_gb() -> i32 {
    8
}

fn default_instance_type() -> String {
    String::from("m5.2xlarge")
}

pub type AvailableAmiAccounts = HashMap<String, AmiAccounts>; // (ami_id, AmiAccounts)

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct AmiAccounts {
    pub name: String,
    #[serde(default)]
    pub github: Vec<String>,
    #[serde(default)]
    pub docker: Vec<String>,
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
    #[serde(default = "default_stats_refresh_interval")]
    pub stats_polling_rate: Timelength,
    #[serde(default)]
    pub allowed_ips: Vec<IpAddr>,
    #[serde(default)]
    pub passkeys: Vec<String>,
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

fn default_stats_refresh_interval() -> Timelength {
    Timelength::FiveSeconds
}

#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
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
