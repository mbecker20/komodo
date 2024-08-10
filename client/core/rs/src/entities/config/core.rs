//! # Configuring the Core API
//!
//! Monitor core is configured by parsing base configuration file ([CoreConfig]), and overriding
//! any fields given in the file with ones provided on the environment ([Env]).
//!
//! The recommended method for running monitor core is via the docker image. This image has a default
//! configuration file provided in the image, meaning any custom configuration can be provided
//! on the environment alone. However, if a custom configuration file is prefered, it can be mounted
//! into the image at `/config/config.toml`.
//!

use std::{collections::HashMap, path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::entities::{
  logger::{LogConfig, LogLevel, StdioLogMode},
  Timelength,
};

use super::{DockerRegistry, GitProvider};

/// # Monitor Core Environment Variables
///
/// You can override any fields of the [CoreConfig] by passing the associated
/// environment variable. The variables should be passed in the traditional `UPPER_SNAKE_CASE` format,
/// although the lower case format can still be parsed.
///
/// *Note.* The monitor core docker image includes the default core configuration found in
/// the `mbecker20/monitor/config_example` folder of the repo. To configure the core api,
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
  /// Override `jwt_secret`
  pub monitor_jwt_secret: Option<String>,
  /// Override `jwt_ttl`
  pub monitor_jwt_ttl: Option<Timelength>,
  /// Override `repo_directory`
  pub monitor_repo_directory: Option<String>,
  /// Override `sync_poll_interval`
  pub monitor_sync_poll_interval: Option<Timelength>,
  /// Override `stack_poll_interval`
  pub monitor_stack_poll_interval: Option<Timelength>,
  /// Override `build_poll_interval`
  pub monitor_build_poll_interval: Option<Timelength>,
  /// Override `monitoring_interval`
  pub monitor_monitoring_interval: Option<Timelength>,
  /// Override `keep_stats_for_days`
  pub monitor_keep_stats_for_days: Option<u64>,
  /// Override `keep_alerts_for_days`
  pub monitor_keep_alerts_for_days: Option<u64>,
  /// Override `webhook_secret`
  pub monitor_webhook_secret: Option<String>,
  /// Override `webhook_base_url`
  pub monitor_webhook_base_url: Option<String>,

  /// Override `logging.level`
  pub monitor_logging_level: Option<LogLevel>,
  /// Override `logging.stdio`
  pub monitor_logging_stdio: Option<StdioLogMode>,
  /// Override `logging.otlp_endpoint`
  pub monitor_logging_otlp_endpoint: Option<String>,
  /// Override `logging.opentelemetry_service_name`
  pub monitor_logging_opentelemetry_service_name: Option<String>,

  /// Override `transparent_mode`
  pub monitor_transparent_mode: Option<bool>,
  /// Override `ui_write_disabled`
  pub monitor_ui_write_disabled: Option<bool>,
  /// Override `enable_new_users`
  pub monitor_enable_new_users: Option<bool>,

  /// Override `local_auth`
  pub monitor_local_auth: Option<bool>,

  /// Override `google_oauth.enabled`
  pub monitor_google_oauth_enabled: Option<bool>,
  /// Override `google_oauth.id`
  pub monitor_google_oauth_id: Option<String>,
  /// Override `google_oauth.secret`
  pub monitor_google_oauth_secret: Option<String>,

  /// Override `github_oauth.enabled`
  pub monitor_github_oauth_enabled: Option<bool>,
  /// Override `github_oauth.id`
  pub monitor_github_oauth_id: Option<String>,
  /// Override `github_oauth.secret`
  pub monitor_github_oauth_secret: Option<String>,

  /// Override `github_webhook_app.app_id`
  pub monitor_github_webhook_app_app_id: Option<i64>,
  /// Override `github_webhook_app.installations[i].id`. Accepts comma seperated list.
  ///
  /// Note. Paired by index with values in `monitor_github_webhook_app_installations_namespaces`
  pub monitor_github_webhook_app_installations_ids: Option<Vec<i64>>,
  /// Override `github_webhook_app.installations[i].namespace`. Accepts comma seperated list.
  ///
  /// Note. Paired by index with values in `monitor_github_webhook_app_installations_ids`
  pub monitor_github_webhook_app_installations_namespaces:
    Option<Vec<String>>,
  /// Override `github_webhook_app.pk_path`
  pub monitor_github_webhook_app_pk_path: Option<String>,

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

  /// Override `hetzner.token`
  pub monitor_hetzner_token: Option<String>,
}

fn default_config_path() -> String {
  "/config/config.toml".to_string()
}

/// # Core Configuration File
///
/// The Core API initializes it's configuration by reading the environment,
/// parsing the [CoreConfig] schema from the file path specified by `env.monitor_config_path`,
/// and then applying any config field overrides specified in the environment.
///
/// *Note.* The monitor core docker image includes the default core configuration found below.
/// To configure the core api, you can either mount your own custom configuration file
/// to `/config/config.toml` inside the container, or simply override whichever fields
/// you need using the environment.
///
/// Refer to the [example file](https://github.com/mbecker20/monitor/blob/main/config_example/core.config.example.toml) for a full example.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
  /// The title of this monitor deployment. Will be used in the browser page title.
  /// Default: 'Monitor'
  #[serde(default = "default_title")]
  pub title: String,

  /// The host to use with oauth redirect url, whatever host
  /// the user hits to access monitor. eg `https://monitor.mogh.tech`.
  /// Only used if oauth used without user specifying redirect url themselves.
  #[serde(default)]
  pub host: String,

  /// Port the core web server runs on.
  /// Default: 9120.
  #[serde(default = "default_core_port")]
  pub port: u16,

  /// Sent in auth header with req to periphery.
  /// Should be some secure hash, maybe 20-40 chars.
  pub passkey: String,

  /// Optionally provide a specific jwt secret.
  /// Passing nothing or an empty string will cause one to be generated.
  /// Default: "" (empty string)
  #[serde(default)]
  pub jwt_secret: String,

  /// Control how long distributed JWT remain valid for.
  /// Default: `1-day`.
  #[serde(default = "default_jwt_ttl")]
  pub jwt_ttl: Timelength,

  /// Specify the directory used to clone stack / repo / build repos, for latest hash / contents.
  /// The default is fine when using a container.
  /// This directory has no need for persistence, so no need to mount it.
  /// Default: `/repos`
  #[serde(default = "default_repo_directory")]
  pub repo_directory: PathBuf,

  /// Interval at which to poll stacks for any updates / automated actions.
  /// Options: `15-sec`, `1-min`, `5-min`, `15-min`, `1-hr`
  /// Default: `5-min`.  
  #[serde(default = "default_poll_interval")]
  pub stack_poll_interval: Timelength,

  /// Interval at which to poll syncs for any updates / automated actions.
  /// Options: `15-sec`, `1-min`, `5-min`, `15-min`, `1-hr`
  /// Default: `5-min`.  
  #[serde(default = "default_poll_interval")]
  pub sync_poll_interval: Timelength,

  /// Interval at which to poll build commit hash for any updates / automated actions.
  /// Options: `15-sec`, `1-min`, `5-min`, `15-min`, `1-hr`
  /// Default: `5-min`.  
  #[serde(default = "default_poll_interval")]
  pub build_poll_interval: Timelength,

  /// Interval at which to collect server stats and send any alerts.
  /// Default: `15-sec`
  #[serde(default = "default_monitoring_interval")]
  pub monitoring_interval: Timelength,

  /// Number of days to keep stats, or 0 to disable pruning.
  /// Stats older than this number of days are deleted on a daily cycle
  /// Default: 14
  #[serde(default = "default_prune_days")]
  pub keep_stats_for_days: u64,

  /// Number of days to keep alerts, or 0 to disable pruning.
  /// Alerts older than this number of days are deleted on a daily cycle
  /// Default: 14
  #[serde(default = "default_prune_days")]
  pub keep_alerts_for_days: u64,

  /// Configure logging
  #[serde(default)]
  pub logging: LogConfig,

  /// Enable transparent mode, which gives all (enabled) users read access to all resources.
  #[serde(default)]
  pub transparent_mode: bool,

  /// Disable user ability to use the UI to update resource configuration.
  #[serde(default)]
  pub ui_write_disabled: bool,

  /// enable login with local auth
  #[serde(default)]
  pub local_auth: bool,

  /// Configure google oauth
  #[serde(default)]
  pub google_oauth: OauthCredentials,

  /// Configure github oauth
  #[serde(default)]
  pub github_oauth: OauthCredentials,

  /// New users will be automatically enabled.
  /// Combined with transparent mode, this is suitable for a demo instance.
  #[serde(default)]
  pub enable_new_users: bool,

  /// Used to verify validity from webhooks.
  /// Should be some secure hash maybe 20-40 chars.
  /// It is given to git provider when configuring the webhook.
  #[serde(default)]
  pub webhook_secret: String,

  /// Override the webhook listener base url, if None will use the address defined as 'host'.
  /// Example: `https://webhooks.mogh.tech`
  ///
  /// This can be used if core sits on an internal network which is
  /// unreachable directly from the open internet.
  /// A reverse proxy in a public network can forward webhooks to the internal monitor.
  pub webhook_base_url: Option<String>,

  /// Configure a Github Webhook app.
  /// Allows users to manage repo webhooks from within the Monitor UI.
  #[serde(default)]
  pub github_webhook_app: GithubWebhookAppConfig,

  /// Configure core mongo connection.
  ///
  /// An easy deployment method is to use Mongo Atlas to provide
  /// a reliable database.
  pub mongo: MongoConfig,

  /// Configure AWS credentials to use with AWS builds / server launches.
  #[serde(default)]
  pub aws: AwsCredentials,

  /// Configure Hetzner credentials to use with Hetzner builds / server launches.
  #[serde(default)]
  pub hetzner: HetznerCredentials,

  /// Configure core-based secrets. These will be preferentially interpolated into
  /// values if they contain a matching secret. Otherwise, the periphery will have to have the
  /// secret configured.
  #[serde(default)]
  pub secrets: HashMap<String, String>,

  /// Configure git credentials used to clone private repos.
  /// Supports any git provider.
  #[serde(default, alias = "git_provider")]
  pub git_providers: Vec<GitProvider>,

  /// Configure docker credentials used to push / pull images.
  /// Supports any docker image repository.
  #[serde(default, alias = "docker_registry")]
  pub docker_registries: Vec<DockerRegistry>,

  /// Configure aws ecr registries, which are handled differently than other registries
  #[serde(default, alias = "aws_ecr_registry")]
  pub aws_ecr_registries: Vec<AwsEcrConfigWithCredentials>,
}

fn default_title() -> String {
  String::from("Monitor")
}

fn default_core_port() -> u16 {
  9120
}

fn default_jwt_ttl() -> Timelength {
  Timelength::OneDay
}

fn default_repo_directory() -> PathBuf {
  // unwrap ok: `/repos` will always be valid path
  PathBuf::from_str("/repos").unwrap()
}

fn default_prune_days() -> u64 {
  14
}

fn default_poll_interval() -> Timelength {
  Timelength::FiveMinutes
}

fn default_monitoring_interval() -> Timelength {
  Timelength::FifteenSeconds
}

impl CoreConfig {
  pub fn sanitized(&self) -> CoreConfig {
    let config = self.clone();
    CoreConfig {
      title: config.title,
      host: config.host,
      port: config.port,
      passkey: empty_or_redacted(&config.passkey),
      jwt_secret: empty_or_redacted(&config.jwt_secret),
      jwt_ttl: config.jwt_ttl,
      repo_directory: config.repo_directory,
      sync_poll_interval: config.sync_poll_interval,
      stack_poll_interval: config.stack_poll_interval,
      build_poll_interval: config.build_poll_interval,
      monitoring_interval: config.monitoring_interval,
      keep_stats_for_days: config.keep_stats_for_days,
      keep_alerts_for_days: config.keep_alerts_for_days,
      logging: config.logging,
      transparent_mode: config.transparent_mode,
      ui_write_disabled: config.ui_write_disabled,
      enable_new_users: config.enable_new_users,
      local_auth: config.local_auth,
      google_oauth: OauthCredentials {
        enabled: config.google_oauth.enabled,
        id: empty_or_redacted(&config.google_oauth.id),
        secret: empty_or_redacted(&config.google_oauth.id),
      },
      github_oauth: OauthCredentials {
        enabled: config.github_oauth.enabled,
        id: empty_or_redacted(&config.github_oauth.id),
        secret: empty_or_redacted(&config.github_oauth.id),
      },
      webhook_secret: empty_or_redacted(&config.webhook_secret),
      webhook_base_url: config.webhook_base_url,
      github_webhook_app: config.github_webhook_app,
      mongo: MongoConfig {
        uri: config.mongo.uri.map(|cur| empty_or_redacted(&cur)),
        address: config.mongo.address,
        username: config
          .mongo
          .username
          .map(|cur| empty_or_redacted(&cur)),
        password: config
          .mongo
          .password
          .map(|cur| empty_or_redacted(&cur)),
        app_name: config.mongo.app_name,
        db_name: config.mongo.db_name,
      },
      aws: AwsCredentials {
        access_key_id: empty_or_redacted(&config.aws.access_key_id),
        secret_access_key: empty_or_redacted(
          &config.aws.secret_access_key,
        ),
      },
      hetzner: HetznerCredentials {
        token: empty_or_redacted(&config.hetzner.token),
      },
      secrets: config
        .secrets
        .into_iter()
        .map(|(id, secret)| (id, empty_or_redacted(&secret)))
        .collect(),
      git_providers: config
        .git_providers
        .into_iter()
        .map(|mut provider| {
          provider.accounts.iter_mut().for_each(|account| {
            account.token = empty_or_redacted(&account.token);
          });
          provider
        })
        .collect(),
      docker_registries: config
        .docker_registries
        .into_iter()
        .map(|mut provider| {
          provider.accounts.iter_mut().for_each(|account| {
            account.token = empty_or_redacted(&account.token);
          });
          provider
        })
        .collect(),
      aws_ecr_registries: config
        .aws_ecr_registries
        .into_iter()
        .map(|mut ecr| {
          ecr.access_key_id = empty_or_redacted(&ecr.access_key_id);
          ecr.secret_access_key =
            empty_or_redacted(&ecr.secret_access_key);
          ecr
        })
        .collect(),
    }
  }
}

fn empty_or_redacted(src: &str) -> String {
  if src.is_empty() {
    String::new()
  } else {
    String::from("##############")
  }
}

/// Generic Oauth credentials
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OauthCredentials {
  /// Whether this oauth method is available for usage.
  #[serde(default)]
  pub enabled: bool,
  /// The Oauth client id.
  #[serde(default)]
  pub id: String,
  /// The Oauth client secret.
  #[serde(default)]
  pub secret: String,
}

/// Provide mongo connection information.
/// Must provide ONE of:
/// 1. `uri`
/// 2. `address` + `username` + `password`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoConfig {
  /// Full mongo uri string, eg. `mongodb://username:password@your.mongo.int:27017`
  pub uri: Option<String>,
  /// Just the address part of the uri, eg `your.mongo.int:27017`
  pub address: Option<String>,
  /// Mongo user username
  pub username: Option<String>,
  /// Mongo user password
  pub password: Option<String>,
  /// Mongo app name. default: `monitor_core`
  #[serde(default = "default_core_mongo_app_name")]
  pub app_name: String,
  /// Mongo db name. Which mongo database to create the collections in.
  /// Default: `monitor`.
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

/// Provide AWS credentials for monitor to use.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AwsCredentials {
  /// The aws ACCESS_KEY_ID
  pub access_key_id: String,
  /// The aws SECRET_ACCESS_KEY
  pub secret_access_key: String,
}

/// Provide Hetzner credentials for monitor to use.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HetznerCredentials {
  pub token: String,
}

/// Provide configuration for an Aws Ecr registry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AwsEcrConfigWithCredentials {
  /// A label for the registry
  pub label: String,
  /// The Aws region
  pub region: String,
  /// The Aws account id
  pub account_id: String,
  /// The Aws ACCESS_KEY_ID
  pub access_key_id: String,
  /// The Aws SECRET_ACCESS_KEY
  pub secret_access_key: String,
}

/// Provide configuration for an Aws Ecr registry.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AwsEcrConfig {
  /// The Aws region
  pub region: String,
  /// The Aws account id
  pub account_id: String,
}

impl AwsEcrConfig {
  pub fn from(config: &AwsEcrConfigWithCredentials) -> AwsEcrConfig {
    AwsEcrConfig {
      region: config.region.to_string(),
      account_id: config.account_id.to_string(),
    }
  }
}

/// Provide configuration for a Github Webhook app.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubWebhookAppConfig {
  /// Github app id
  pub app_id: i64,
  /// Configure the app installations on multiple accounts / organizations.
  pub installations: Vec<GithubWebhookAppInstallationConfig>,
  /// Private key path. Default: /github-private-key.pem.
  #[serde(default = "default_private_key_path")]
  pub pk_path: String,
}

fn default_private_key_path() -> String {
  String::from("/github/private-key.pem")
}

impl Default for GithubWebhookAppConfig {
  fn default() -> Self {
    GithubWebhookAppConfig {
      app_id: 0,
      installations: Default::default(),
      pk_path: default_private_key_path(),
    }
  }
}

/// Provide configuration for a Github Webhook app installation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GithubWebhookAppInstallationConfig {
  /// The installation ID
  pub id: i64,
  /// The user or organization name
  pub namespace: String,
}
