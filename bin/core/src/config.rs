use std::sync::OnceLock;

use logger::LogConfig;
use merge_config_files::parse_config_file;
use monitor_client::entities::Timelength;
use serde::Deserialize;

pub fn env() -> &'static Env {
  static ENV: OnceLock<Env> = OnceLock::new();
  ENV.get_or_init(|| {
    envy::from_env().expect("failed to parse environment")
  })
}

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

pub fn core_config() -> &'static CoreConfig {
  static CORE_CONFIG: OnceLock<CoreConfig> = OnceLock::new();
  CORE_CONFIG.get_or_init(|| {
    let env = env();
    let config_path = &env.config_path;
    let mut config =
      parse_config_file::<CoreConfig>(config_path.as_str())
        .unwrap_or_else(|e| {
          panic!("failed at parsing config at {config_path} | {e:#}")
        });
    if let Some(port) = env.port {
      config.port = port;
    }
    config
  })
}

#[derive(Deserialize, Debug, Clone)]
pub struct CoreConfig {
  #[serde(default = "default_title")]
  pub title: String,

  /// The host to use with oauth redirect url, whatever host the user hits to access monitor. eg 'https://monitor.mogh.tech'
  #[serde(default)]
  pub host: String,

  /// Port the core web server runs on
  #[serde(default = "default_core_port")]
  pub port: u16,

  /// Sent in auth header with req to periphery
  pub passkey: String,

  /// Configure logging
  #[serde(default)]
  pub logging: LogConfig,

  /// Control how long distributed JWT remain valid for. Default is 1-day
  #[serde(default = "default_jwt_valid_for")]
  pub jwt_valid_for: Timelength,

  /// interval at which to collect server stats and send any alerts
  #[serde(default = "default_monitoring_interval")]
  pub monitoring_interval: Timelength,

  /// number of days to keep stats, or 0 to disable pruning. stats older than this number of days are deleted on a daily cycle
  #[serde(default)]
  pub keep_stats_for_days: u64,

  /// number of days to keep alerts, or 0 to disable pruning. alerts older than this number of days are deleted on a daily cycle
  #[serde(default)]
  pub keep_alerts_for_days: u64,

  /// used to verify validity from github webhooks
  #[serde(default)]
  pub github_webhook_secret: String,

  /// used to form the frontend listener url, if None will use 'host'.
  pub github_webhook_base_url: Option<String>,

  /// allowed docker orgs used with monitor. first in this list will be default for build
  #[serde(default)]
  pub docker_organizations: Vec<String>,

  /// enable login with local auth
  #[serde(default)]
  pub local_auth: bool,

  #[serde(default)]
  pub github_oauth: OauthCredentials,

  #[serde(default)]
  pub google_oauth: OauthCredentials,

  pub mongo: MongoConfig,

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
  Timelength::OneDay
}

fn default_monitoring_interval() -> Timelength {
  Timelength::FifteenSeconds
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct OauthCredentials {
  #[serde(default)]
  pub enabled: bool,
  #[serde(default)]
  pub id: String,
  #[serde(default)]
  pub secret: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct MongoConfig {
  pub uri: Option<String>,
  pub address: Option<String>,
  pub username: Option<String>,
  pub password: Option<String>,
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

#[derive(Deserialize, Debug, Clone, Default)]
pub struct AwsCredentials {
  pub access_key_id: String,
  pub secret_access_key: String,
}
