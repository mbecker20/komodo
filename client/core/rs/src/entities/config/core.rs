use serde::{Deserialize, Serialize};

use crate::entities::{
  logger::{LogConfig, LogLevel, StdioLogMode},
  Timelength,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Env {
  #[serde(default = "default_config_path")]
  pub monitor_config_path: String,

  pub monitor_title: Option<String>,
  pub monitor_host: Option<String>,
  pub monitor_port: Option<u16>,
  pub monitor_passkey: Option<String>,
  pub monitor_jwt_valid_for: Option<Timelength>,
  pub monitor_monitoring_interval: Option<Timelength>,
  pub monitor_keep_stats_for_days: Option<u64>,
  pub monitor_keep_alerts_for_days: Option<u64>,
  pub monitor_github_webhook_secret: Option<String>,
  pub monitor_github_webhook_base_url: Option<String>,
  pub monitor_docker_organizations: Option<Vec<String>>,

  // logging
  pub monitor_logging_level: Option<LogLevel>,
  pub monitor_logging_stdio: Option<StdioLogMode>,
  pub monitor_logging_otlp_endpoint: Option<String>,

  pub monitor_local_auth: Option<bool>,

  // github
  pub monitor_github_oauth_enabled: Option<bool>,
  pub monitor_github_oauth_id: Option<String>,
  pub monitor_github_oauth_secret: Option<String>,

  // google
  pub monitor_google_oauth_enabled: Option<bool>,
  pub monitor_google_oauth_id: Option<String>,
  pub monitor_google_oauth_secret: Option<String>,

  // mongo
  pub monitor_mongo_uri: Option<String>,
  pub monitor_mongo_address: Option<String>,
  pub monitor_mongo_username: Option<String>,
  pub monitor_mongo_password: Option<String>,
  pub monitor_mongo_app_name: Option<String>,
  pub monitor_mongo_db_name: Option<String>,

  // aws
  pub monitor_aws_access_key_id: Option<String>,
  pub monitor_aws_secret_access_key: Option<String>,
}

fn default_config_path() -> String {
  "/config/config.toml".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
  #[serde(default = "default_title")]
  pub title: String,

  /// The host to use with oauth redirect url, whatever host
  /// the user hits to access monitor. eg `https://monitor.mogh.tech`
  #[serde(default)]
  pub host: String,

  /// Port the core web server runs on
  #[serde(default = "default_core_port")]
  pub port: u16,

  /// Sent in auth header with req to periphery
  pub passkey: String,

  /// Control how long distributed JWT remain valid for. Default is 1-day
  #[serde(default = "default_jwt_valid_for")]
  pub jwt_valid_for: Timelength,

  /// interval at which to collect server stats and send any alerts
  #[serde(default = "default_monitoring_interval")]
  pub monitoring_interval: Timelength,

  /// Number of days to keep stats, or 0 to disable pruning. stats older than this number of days are deleted on a daily cycle
  /// Default is 0 (no pruning)
  #[serde(default)]
  pub keep_stats_for_days: u64,

  /// Number of days to keep alerts, or 0 to disable pruning. alerts older than this number of days are deleted on a daily cycle
  /// Default is 0 (no pruning)
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

  /// Configure logging
  #[serde(default)]
  pub logging: LogConfig,

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
  String::from("Monitor")
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

impl CoreConfig {
  pub fn sanitized(&self) -> CoreConfig {
    let mut config = self.clone();

    config.passkey = empty_or_redacted(&config.passkey);
    config.github_webhook_secret =
      empty_or_redacted(&config.github_webhook_secret);

    config.github_oauth.id =
      empty_or_redacted(&config.github_oauth.id);
    config.github_oauth.secret =
      empty_or_redacted(&config.github_oauth.secret);

    config.google_oauth.id =
      empty_or_redacted(&config.google_oauth.id);
    config.google_oauth.secret =
      empty_or_redacted(&config.google_oauth.secret);

    config.mongo.uri =
      config.mongo.uri.map(|cur| empty_or_redacted(&cur));
    config.mongo.username =
      config.mongo.username.map(|cur| empty_or_redacted(&cur));
    config.mongo.password =
      config.mongo.password.map(|cur| empty_or_redacted(&cur));

    config.aws.access_key_id =
      empty_or_redacted(&config.aws.access_key_id);
    config.aws.secret_access_key =
      empty_or_redacted(&config.aws.secret_access_key);

    config
  }
}

fn empty_or_redacted(src: &str) -> String {
  if src.is_empty() {
    String::new()
  } else {
    String::from("##############")
  }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OauthCredentials {
  #[serde(default)]
  pub enabled: bool,
  #[serde(default)]
  pub id: String,
  #[serde(default)]
  pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for MongoConfig {
  fn default() -> Self {
    Self {
      uri: None,
      address: Some("localhost:27017".to_string()),
      username: None,
      password: None,
      app_name: default_core_mongo_app_name(),
      db_name: default_core_mongo_db_name(),
    }
  }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AwsCredentials {
  pub access_key_id: String,
  pub secret_access_key: String,
}
