use std::sync::OnceLock;

use anyhow::Context;
use logger::LogConfig;
use merge_config_files::parse_config_file;
use monitor_client::entities::Timelength;
use serde::Deserialize;

pub fn frontend_path() -> &'static String {
  #[derive(Deserialize)]
  struct FrontendEnv {
    #[serde(default = "default_frontend_path")]
    monitor_frontend_path: String,
  }

  fn default_frontend_path() -> String {
    "/frontend".to_string()
  }

  static FRONTEND_PATH: OnceLock<String> = OnceLock::new();
  FRONTEND_PATH.get_or_init(|| {
    let FrontendEnv {
      monitor_frontend_path,
    } = envy::from_env()
      .context("failed to parse FrontendEnv")
      .unwrap();
    monitor_frontend_path
  })
}

pub fn core_config() -> &'static CoreConfig {
  #[derive(Deserialize)]
  struct OverrideEnv {
    #[serde(default = "default_config_path")]
    monitor_config_path: String,

    monitor_title: Option<String>,
    monitor_host: Option<String>,
    monitor_port: Option<u16>,
    monitor_passkey: Option<String>,
    monitor_jwt_valid_for: Option<Timelength>,
    monitor_monitoring_interval: Option<Timelength>,
    monitor_keep_stats_for_days: Option<u64>,
    monitor_keep_alerts_for_days: Option<u64>,
    monitor_github_webhook_secret: Option<String>,
    monitor_github_webhook_base_url: Option<String>,
    monitor_docker_organizations: Option<Vec<String>>,

    // logging
    monitor_logging_level: Option<logger::LogLevel>,
    monitor_logging_stdio: Option<logger::StdioLogMode>,
    monitor_logging_otlp_endpoint: Option<String>,

    monitor_local_auth: Option<bool>,

    // github
    monitor_github_oauth_enabled: Option<bool>,
    monitor_github_oauth_id: Option<String>,
    monitor_github_oauth_secret: Option<String>,

    // google
    monitor_google_oauth_enabled: Option<bool>,
    monitor_google_oauth_id: Option<String>,
    monitor_google_oauth_secret: Option<String>,

    // mongo
    monitor_mongo_uri: Option<String>,
    monitor_mongo_address: Option<String>,
    monitor_mongo_username: Option<String>,
    monitor_mongo_password: Option<String>,
    monitor_mongo_app_name: Option<String>,
    monitor_mongo_db_name: Option<String>,

    // aws
    monitor_aws_access_key_id: Option<String>,
    monitor_aws_secret_access_key: Option<String>,
  }

  fn default_config_path() -> String {
    "/config/config.toml".to_string()
  }

  static CORE_CONFIG: OnceLock<CoreConfig> = OnceLock::new();
  CORE_CONFIG.get_or_init(|| {
    let env: OverrideEnv = envy::from_env()
      .context("failed to parse OverrideEnv")
      .unwrap();
    let config_path = &env.monitor_config_path;
    let mut config =
      parse_config_file::<CoreConfig>(config_path.as_str())
        .unwrap_or_else(|e| {
          panic!("failed at parsing config at {config_path} | {e:#}")
        });

    // Overrides
    config.title = env.monitor_title.unwrap_or(config.title);
    config.host = env.monitor_host.unwrap_or(config.host);
    config.port = env.monitor_port.unwrap_or(config.port);
    config.passkey = env.monitor_passkey.unwrap_or(config.passkey);
    config.jwt_valid_for =
      env.monitor_jwt_valid_for.unwrap_or(config.jwt_valid_for);
    config.monitoring_interval = env
      .monitor_monitoring_interval
      .unwrap_or(config.monitoring_interval);
    config.keep_stats_for_days = env
      .monitor_keep_stats_for_days
      .unwrap_or(config.keep_stats_for_days);
    config.keep_alerts_for_days = env
      .monitor_keep_alerts_for_days
      .unwrap_or(config.keep_alerts_for_days);
    config.github_webhook_secret = env
      .monitor_github_webhook_secret
      .unwrap_or(config.github_webhook_secret);
    config.github_webhook_base_url = env
      .monitor_github_webhook_base_url
      .or(config.github_webhook_base_url);
    config.docker_organizations = env
      .monitor_docker_organizations
      .unwrap_or(config.docker_organizations);

    config.logging.level =
      env.monitor_logging_level.unwrap_or(config.logging.level);
    config.logging.stdio =
      env.monitor_logging_stdio.unwrap_or(config.logging.stdio);
    config.logging.otlp_endpoint = env
      .monitor_logging_otlp_endpoint
      .or(config.logging.otlp_endpoint);

    config.local_auth =
      env.monitor_local_auth.unwrap_or(config.local_auth);

    config.github_oauth.enabled = env
      .monitor_github_oauth_enabled
      .unwrap_or(config.github_oauth.enabled);
    config.github_oauth.id = env
      .monitor_github_oauth_id
      .unwrap_or(config.github_oauth.id);
    config.github_oauth.secret = env
      .monitor_github_oauth_secret
      .unwrap_or(config.github_oauth.secret);

    config.google_oauth.enabled = env
      .monitor_google_oauth_enabled
      .unwrap_or(config.google_oauth.enabled);
    config.google_oauth.id = env
      .monitor_google_oauth_id
      .unwrap_or(config.google_oauth.id);
    config.google_oauth.secret = env
      .monitor_google_oauth_secret
      .unwrap_or(config.google_oauth.secret);

    config.mongo.uri = env.monitor_mongo_uri.or(config.mongo.uri);
    config.mongo.address =
      env.monitor_mongo_address.or(config.mongo.address);
    config.mongo.username =
      env.monitor_mongo_username.or(config.mongo.username);
    config.mongo.password =
      env.monitor_mongo_password.or(config.mongo.password);
    config.mongo.app_name =
      env.monitor_mongo_app_name.unwrap_or(config.mongo.app_name);
    config.mongo.db_name =
      env.monitor_mongo_db_name.unwrap_or(config.mongo.db_name);

    config.aws.access_key_id = env
      .monitor_aws_access_key_id
      .unwrap_or(config.aws.access_key_id);
    config.aws.secret_access_key = env
      .monitor_aws_secret_access_key
      .unwrap_or(config.aws.secret_access_key);

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

#[derive(Deserialize, Debug, Clone, Default)]
pub struct OauthCredentials {
  #[serde(default)]
  pub enabled: bool,
  #[serde(default)]
  pub id: String,
  #[serde(default)]
  pub secret: String,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone, Default)]
pub struct AwsCredentials {
  pub access_key_id: String,
  pub secret_access_key: String,
}
