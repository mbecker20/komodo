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
  /// Override `jwt_valid_for`
  pub monitor_jwt_valid_for: Option<Timelength>,
  /// Override `sync_directory`
  pub monitor_sync_directory: Option<String>,
  /// Override `stack_directory`
  pub monitor_stack_directory: Option<String>,
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
/// ## Example TOML
/// ```toml
/// ## this will be the document title on the web page (shows up as text in the browser tab).
/// ## default: 'Monitor'
/// title = "Monitor"
///
/// ## Required for oauth functionality. This should be the url used to access monitor in browser,
/// ## potentially behind DNS.
/// ## Eg https://monitor.dev or http://12.34.56.78:9000.
/// ## This should match the address configured in your oauth app.
/// ## Required (no default).
/// host = "https://monitor.dev"
///
/// ## The port the core system will run on. If running core in docker container,
/// ## Leave as this port as 9000 and use port bind eg. -p 9001:9000
/// ## Default: 9000
/// port = 9000
///
/// ## Must match a passkey in periphery config to communicate with periphery.
/// ## Required (No default)
/// passkey = "a_random_passkey"
/// 
/// ## token that has to be given to git provider during repo webhook config as the secret
/// ## default: empty (none)
/// webhook_secret = "a_random_webhook_secret"
/// 
/// ## an alternate base url that is used to recieve git webhook requests
/// ## if empty or not specified, will use 'host' address as base
/// ## default: empty (none)
/// # webhook_base_url = "https://git-webhook.monitor.dev"
///
/// ## Specify the log level of the monitor core application.
/// ## Default: `info`.
/// ## Options: `off`, `error`, `warn`, `info`, `debug`, `trace`.
/// logging.level = "info"
///
/// ## Specify the logging format for stdout / stderr.
/// ## Default: standard
/// ## Options: `standard`, `json`, `none`
/// logging.stdio = "standard"
///
/// ## Specify a opentelemetry otlp endpoint to send traces to.
/// ## Optional, default unassigned (don't export telemetry).
/// # logging.otlp_endpoint = "http://localhost:4317"
///
/// ## Specify how long an issued jwt stays valid.
/// ## All jwts are invalidated on application restart.
/// ## Default: `1-day`.
/// ## Options: `1-hr`, `12-hr`, `1-day`, `3-day`, `1-wk`, `2-wk`, `30-day`.
/// jwt_valid_for = "1-day"
///
/// ## Controls the granularity of the system stats collection by monitor core.
/// ## Options: `5-sec`, `15-sec`, `30-sec`, `1-min`, `2-min`, `5-min`.
/// ## Default: `15-sec`.
/// monitoring_interval = "15-sec"
///
/// ## Number of days to store stats, or 0 to disable stats pruning.
/// ## Stats older than this number of days are deleted daily
/// ## Default: 0 (pruning disabled)
/// keep_stats_for_days = 14
///
/// ## Number of days to store alerts, or 0 to disable alert pruning.
/// ## Alerts older than this number of days are deleted daily
/// ## Default: 0 (pruning disabled)
/// keep_alerts_for_days = 14
///
/// ## allows all users to have read access on all resources
/// ## default: false
/// # transparent_mode = true
///
/// ## disables write support on resources in the UI
/// ## default: false
/// # ui_write_disabled = true
///
/// ## allow or deny user login with username / password
/// ## default: false
/// # local_auth = true
///
/// ## Use to configure google oauth
/// # google_oauth.enabled = true
/// # google_oauth.id = "your_google_client_id"
/// # google_oauth.secret = "your_google_client_secret"
///
/// ## Use to configure github oauth
/// # github_oauth.enabled = true
/// # github_oauth.id = "your_github_client_id"
/// # github_oauth.secret = "your_github_client_secret"
///
/// ## Configure github webhook app. Enables webhook management apis.
/// # github_webhook_app.app_id = 1234455 # Find on the app page.
/// # github_webhook_app.installations = [
/// #   ## Find the id after installing the app to user / organization. "namespace" is the username / organization name.
/// #   { id = 1234, namespace = "mbecker20" }
/// # ]
///
/// ## Path to github webhook app private key.
/// ## This is defaulted to `/github/private-key.pem`, and doesn't need to be changed if running in Docker.
/// ## Just mount the private key pem file on the host to `/github/private-key.pem` in the container.
/// # github_webhook_app.pk_path = "/path/to/pk.pem"
///
/// ## MUST comment back in some way to configure mongo.
/// # mongo.uri = "mongodb://username:password@localhost:27017"
/// ## ==== or ====
/// mongo.address = "localhost:27017"
/// # mongo.username = "username"
/// # mongo.password = "password"
/// ## ==== other ====
/// ## default: monitor. this is the name of the mongo database that monitor will create its collections in.
/// mongo.db_name = "monitor"
/// ## default: monitor_core. this is the assigned app_name of the mongo client
/// mongo.app_name = "monitor_core"
///
/// ## provide aws api keys for ephemeral builders
/// # aws.access_key_id = "your_aws_key_id"
/// # aws.secret_access_key = "your_aws_secret_key"
///
/// ## provide hetzner api token for ephemeral builders
/// # hetzner.token = "your_hetzner_token"
///
/// ## provide core-base secrets
/// [secrets]
/// # SECRET_1 = "value_1"
/// # SECRET_2 = "value_2"
///
/// ## configure git providers
/// # [[git_provider]]
/// # domain = "git.mogh.tech" # use a custom provider, like self-hosted gitea
/// # accounts = [
/// #     { username = "mbecker20", token = "access_token_for_account" },
/// # ]
///
/// ## configure docker registries
/// # [[docker_registry]]
/// # domain = "docker.io"
/// # accounts = [
/// #     { username = "mbecker2020", token = "access_token_for_account" }
/// # ]
/// # organizations = ["DockerhubOrganization"]
///
/// ## configure aws ecr registries
/// # [aws_ecr_registry.label_1]
/// # region = "us-east-1"
/// # account_id = "123456677"
/// # access_key_id = "your_aws_key_id_1"
/// # secret_access_key = "your_aws_secret_key_1"
///
/// # [aws_ecr_registry.label_2]
/// # region = "us-west-1"
/// # account_id = "123456677"
/// # access_key_id = "your_aws_key_id_2"
/// # secret_access_key = "your_aws_secret_key_2"
/// ```
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

  /// Specify the directory used to clone sync repos. The default is fine when using a container.
  /// This directory has no need for persistence, so no need to mount it.
  /// Default: `/syncs`
  #[serde(default = "default_sync_directory")]
  pub sync_directory: PathBuf,

  /// Specify the directory used to clone stack repos. The default is fine when using a container.
  /// This directory has no need for persistence, so no need to mount it.
  /// Default: `/stacks`
  #[serde(default = "default_stack_directory")]
  pub stack_directory: PathBuf,

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
  9000
}

fn default_jwt_valid_for() -> Timelength {
  Timelength::OneDay
}

fn default_sync_directory() -> PathBuf {
  // `/syncs` will always be valid path
  PathBuf::from_str("/syncs").unwrap()
}

fn default_stack_directory() -> PathBuf {
  // `/stacks` will always be valid path
  PathBuf::from_str("/stacks").unwrap()
}

fn default_monitoring_interval() -> Timelength {
  Timelength::FifteenSeconds
}

impl CoreConfig {
  pub fn sanitized(&self) -> CoreConfig {
    let mut config = self.clone();

    config.passkey = empty_or_redacted(&config.passkey);
    config.webhook_secret = empty_or_redacted(&config.webhook_secret);

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

    config.hetzner.token = empty_or_redacted(&config.hetzner.token);

    config.secrets.iter_mut().for_each(|(_, secret)| {
      *secret = empty_or_redacted(secret);
    });

    config.git_providers.iter_mut().for_each(|provider| {
      provider.accounts.iter_mut().for_each(|account| {
        account.token = empty_or_redacted(&account.token);
      })
    });

    config.docker_registries.iter_mut().for_each(|provider| {
      provider.accounts.iter_mut().for_each(|account| {
        account.token = empty_or_redacted(&account.token);
      })
    });

    config.aws_ecr_registries.iter_mut().for_each(|ecr| {
      ecr.access_key_id = empty_or_redacted(&ecr.access_key_id);
      ecr.secret_access_key =
        empty_or_redacted(&ecr.secret_access_key);
    });

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
