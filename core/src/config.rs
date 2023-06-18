use anyhow::Context;
use merge_config_files::parse_config_file;
use monitor_types::entities::Timelength;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Env {
    #[serde(default = "default_config_path")]
    pub config_path: String,
    #[serde(default = "default_frontend_path")]
    pub frontend_path: String,
    pub port: Option<u16>,
}

fn default_config_path() -> String {
    "/config/config.toml".to_string()
}

fn default_frontend_path() -> String {
    "/frontend".to_string()
}

impl Env {
    pub fn load() -> anyhow::Result<Env> {
        dotenv::dotenv().ok();
        envy::from_env().context("failed to parse environment")
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CoreConfig {
    #[serde(default = "default_title")]
    pub title: String,

    // the host to use with oauth redirect url, whatever host the user hits to access monitor. eg 'https://monitor.mogh.tech'
    pub host: String,

    // port the core web server runs on
    #[serde(default = "default_core_port")]
    pub port: u16,

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

    // used to form the frontend listener url, if None will use 'host'.
    pub github_webhook_base_url: Option<String>,

    // sent in auth header with req to periphery
    pub passkey: String,

    // integration with slack app
    pub slack_url: Option<String>,

    // enable login with local auth
    pub local_auth: bool,

    #[serde(default = "default_log_level")]
    pub log_level: LogLevel,

    // allowed docker orgs used with monitor. first in this list will be default for build
    #[serde(default)]
    pub docker_organizations: Vec<String>,

    pub mongo: MongoConfig,

    #[serde(default)]
    pub github_oauth: OauthCredentials,

    #[serde(default)]
    pub google_oauth: OauthCredentials,

    #[serde(default)]
    pub aws: AwsCredentials,
}

fn default_title() -> String {
    String::from("monitor")
}

fn default_core_port() -> u16 {
    9000
}

fn default_jwt_valid_for() -> Timelength {
    Timelength::OneWeek
}

fn default_log_level() -> LogLevel {
    LogLevel::Info
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
pub struct AwsCredentials {
    #[serde(skip_serializing)]
    pub access_key_id: String,
    #[serde(skip_serializing)]
    pub secret_access_key: String,
}

impl CoreConfig {
    pub fn load(config_path: &str) -> CoreConfig {
        parse_config_file::<CoreConfig>(config_path)
            .unwrap_or_else(|e| panic!("failed at parsing config at {config_path} | {e:#?}"))
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<LogLevel> for log::LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Off => log::LevelFilter::Off,
            LogLevel::Error => log::LevelFilter::Error,
            LogLevel::Warn => log::LevelFilter::Warn,
            LogLevel::Info => log::LevelFilter::Info,
            LogLevel::Debug => log::LevelFilter::Debug,
            LogLevel::Trace => log::LevelFilter::Trace,
        }
    }
}
