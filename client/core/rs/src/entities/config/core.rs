use serde::{Deserialize, Serialize};

use crate::entities::{
  logger::{LogConfig, LogLevel, StdioLogMode},
  Timelength,
};

/// # Monitor Core Environment Variables
///
/// You can override any fields of the [CoreConfig] by passing the associated
/// environment variable. The variables should be passed in the traditional `UPPER_SNAKE_CASE` format,
/// although the lower case format can still be parsed.
///
/// *Note.* The monitor core docker image includes the default core configuration found in
/// the `mbecker20/monitor/config_example` folder of the repo. To configigure the core api,
/// you can either mount your own custom configuration file to `/config/config.toml` inside the container,
/// or simply override whichever fields you need using the environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Env {
  /// Specify a custom config path for the core config toml.
  /// Default: `/config/config.toml`
  #[serde(default = "default_config_path")]
  pub monitor_config_path: String,

  /// Override `title`
  pub monitor_title: Option<String>,
  /// Override `host`
  pub monitor_host: Option<String>,
  /// Override `port`
  pub monitor_port: Option<u16>,
  /// Override `passkey`
  pub monitor_passkey: Option<String>,
  /// Override `jwt_valid_for`
  pub monitor_jwt_valid_for: Option<Timelength>,
  /// Override `monitoring_interval`
  pub monitor_monitoring_interval: Option<Timelength>,
  /// Override `keep_stats_for_days`
  pub monitor_keep_stats_for_days: Option<u64>,
  /// Override `keep_alerts_for_days`
  pub monitor_keep_alerts_for_days: Option<u64>,
  /// Override `github_webhook_secret`
  pub monitor_github_webhook_secret: Option<String>,
  /// Override `github_webhook_base_url`
  pub monitor_github_webhook_base_url: Option<String>,
  /// Override `docker_organizations`
  pub monitor_docker_organizations: Option<Vec<String>>,

  /// Override `logging.level`
  pub monitor_logging_level: Option<LogLevel>,
  /// Override `logging.stdio`
  pub monitor_logging_stdio: Option<StdioLogMode>,
  /// Override `logging.otlp_endpoint`
  pub monitor_logging_otlp_endpoint: Option<String>,
  /// Override `logging.opentelemetry_service_name`
  pub monitor_logging_opentelemetry_service_name: Option<String>,

  /// Override `local_auth`
  pub monitor_local_auth: Option<bool>,

  /// Override `github_oauth.enabled`
  pub monitor_github_oauth_enabled: Option<bool>,
  /// Override `github_oauth.id`
  pub monitor_github_oauth_id: Option<String>,
  /// Override `github_oauth.secret`
  pub monitor_github_oauth_secret: Option<String>,

  /// Override `google_oauth.enabled`
  pub monitor_google_oauth_enabled: Option<bool>,
  /// Override `google_oauth.id`
  pub monitor_google_oauth_id: Option<String>,
  /// Override `google_oauth.secret`
  pub monitor_google_oauth_secret: Option<String>,

  /// Override `mongo.uri`
  pub monitor_mongo_uri: Option<String>,
  /// Override `mongo.address`
  pub monitor_mongo_address: Option<String>,
  /// Override `mongo.username`
  pub monitor_mongo_username: Option<String>,
  /// Override `mongo.password`
  pub monitor_mongo_password: Option<String>,
  /// Override `mongo.app_name`
  pub monitor_mongo_app_name: Option<String>,
  /// Override `mongo.db_name`
  pub monitor_mongo_db_name: Option<String>,

  /// Override `aws.access_key_id`
  pub monitor_aws_access_key_id: Option<String>,
  /// Override `aws.secret_access_key`
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
  /// the user hits to access monitor. eg `https://monitor.mogh.tech`.
  /// Only used if oauth used without user specifying redirect url themselves.
  #[serde(default)]
  pub host: String,

  /// Port the core web server runs on.
  /// Default: 9000.
  #[serde(default = "default_core_port")]
  pub port: u16,

  /// Sent in auth header with req to periphery.
  /// Should be some secure hash, maybe 20-40 chars.
  pub passkey: String,

  /// Control how long distributed JWT remain valid for.
  /// Default: `1-day`.
  #[serde(default = "default_jwt_valid_for")]
  pub jwt_valid_for: Timelength,

  /// Interval at which to collect server stats and send any alerts.
  /// Default: `15-sec`
  #[serde(default = "default_monitoring_interval")]
  pub monitoring_interval: Timelength,

  /// Number of days to keep stats, or 0 to disable pruning. stats older than this number of days are deleted on a daily cycle
  /// Default: 0 (no pruning).
  #[serde(default)]
  pub keep_stats_for_days: u64,

  /// Number of days to keep alerts, or 0 to disable pruning. alerts older than this number of days are deleted on a daily cycle
  /// Default: 0 (no pruning).
  #[serde(default)]
  pub keep_alerts_for_days: u64,

  /// Used to verify validity from github webhooks.
  /// Should be some secure hash maybe 20-40 chars.
  /// It needs to be given to github when configuring the webhook.
  #[serde(default)]
  pub github_webhook_secret: String,

  /// Used to form the frontend listener url, if None will use 'host'.
  ///
  /// This can be used if core sits on an internal network which is
  /// unreachable directly from the open internet.
  /// A reverse proxy in a public network (with its own DNS)
  /// can forward webhooks to the internal monitor
  pub github_webhook_base_url: Option<String>,

  /// allowed docker orgs used with monitor. first in this list will be default for build.
  /// Default: none.
  #[serde(default)]
  pub docker_organizations: Vec<String>,

  /// Configure logging
  #[serde(default)]
  pub logging: LogConfig,

  /// enable login with local auth
  #[serde(default)]
  pub local_auth: bool,

  /// Configure github oauth
  #[serde(default)]
  pub github_oauth: OauthCredentials,

  /// Configure google oauth
  #[serde(default)]
  pub google_oauth: OauthCredentials,

  /// Configure core mongo connection.
  ///
  /// An easy deployment method is to use Mongo Atlas to provide
  /// a reliable database.
  pub mongo: MongoConfig,

  /// Configure AWS credentials to use with AWS builds / server launches.
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
