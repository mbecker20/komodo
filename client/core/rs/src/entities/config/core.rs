//! # Configuring the Komodo Core API
//!
//! Komodo Core is configured by parsing base configuration file ([CoreConfig]), and overriding
//! any fields given in the file with ones provided on the environment ([Env]).
//!
//! The recommended method for running Komodo Core is via the docker image. This image has a default
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

use super::{empty_or_redacted, DockerRegistry, GitProvider};

/// # Komodo Core Environment Variables
///
/// You can override any fields of the [CoreConfig] by passing the associated
/// environment variable. The variables should be passed in the traditional `UPPER_SNAKE_CASE` format,
/// although the lower case format can still be parsed.
///
/// *Note.* The Komodo Core docker image includes the default core configuration found at
/// [https://github.com/mbecker20/komodo/blob/main/config/core.config.toml](https://github.com/mbecker20/komodo/blob/main/config/core.config.toml).
/// To configure the core api, you can either mount your own custom configuration file to
/// `/config/config.toml` inside the container,
/// or simply override whichever fields you need using the environment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Env {
  /// Specify a custom config path for the core config toml.
  /// Default: `/config/config.toml`
  #[serde(default = "default_config_path")]
  pub komodo_config_path: String,

  /// Override `title`
  pub komodo_title: Option<String>,
  /// Override `host`
  pub komodo_host: Option<String>,
  /// Override `port`
  pub komodo_port: Option<u16>,
  /// Override `passkey`
  pub komodo_passkey: Option<String>,
  /// Override `passkey` with file
  pub komodo_passkey_file: Option<PathBuf>,
  /// Override `first_server`
  pub komodo_first_server: Option<String>,
  /// Override `frontend_path`
  pub komodo_frontend_path: Option<String>,
  /// Override `jwt_secret`
  pub komodo_jwt_secret: Option<String>,
  /// Override `jwt_secret` from file
  pub komodo_jwt_secret_file: Option<PathBuf>,
  /// Override `jwt_ttl`
  pub komodo_jwt_ttl: Option<Timelength>,
  /// Override `repo_directory`
  pub komodo_repo_directory: Option<PathBuf>,
  /// Override `sync_poll_interval`
  pub komodo_sync_poll_interval: Option<Timelength>,
  /// Override `stack_poll_interval`
  pub komodo_stack_poll_interval: Option<Timelength>,
  /// Override `build_poll_interval`
  pub komodo_build_poll_interval: Option<Timelength>,
  /// Override `repo_poll_interval`
  pub komodo_repo_poll_interval: Option<Timelength>,
  /// Override `monitoring_interval`
  pub komodo_monitoring_interval: Option<Timelength>,
  /// Override `keep_stats_for_days`
  pub komodo_keep_stats_for_days: Option<u64>,
  /// Override `keep_alerts_for_days`
  pub komodo_keep_alerts_for_days: Option<u64>,
  /// Override `webhook_secret`
  pub komodo_webhook_secret: Option<String>,
  /// Override `webhook_secret` with file
  pub komodo_webhook_secret_file: Option<PathBuf>,
  /// Override `webhook_base_url`
  pub komodo_webhook_base_url: Option<String>,

  /// Override `logging.level`
  pub komodo_logging_level: Option<LogLevel>,
  /// Override `logging.stdio`
  pub komodo_logging_stdio: Option<StdioLogMode>,
  /// Override `logging.otlp_endpoint`
  pub komodo_logging_otlp_endpoint: Option<String>,
  /// Override `logging.opentelemetry_service_name`
  pub komodo_logging_opentelemetry_service_name: Option<String>,

  /// Override `transparent_mode`
  pub komodo_transparent_mode: Option<bool>,
  /// Override `ui_write_disabled`
  pub komodo_ui_write_disabled: Option<bool>,
  /// Override `enable_new_users`
  pub komodo_enable_new_users: Option<bool>,
  /// Override `disable_user_registration`
  pub komodo_disable_user_registration: Option<bool>,
  /// Override `disable_confirm_dialog`
  pub komodo_disable_confirm_dialog: Option<bool>,
  /// Override `disable_non_admin_create`
  pub komodo_disable_non_admin_create: Option<bool>,

  /// Override `local_auth`
  pub komodo_local_auth: Option<bool>,

  /// Override `google_oauth.enabled`
  pub komodo_google_oauth_enabled: Option<bool>,
  /// Override `google_oauth.id`
  pub komodo_google_oauth_id: Option<String>,
  /// Override `google_oauth.id` from file
  pub komodo_google_oauth_id_file: Option<PathBuf>,
  /// Override `google_oauth.secret`
  pub komodo_google_oauth_secret: Option<String>,
  /// Override `google_oauth.secret` from file
  pub komodo_google_oauth_secret_file: Option<PathBuf>,

  /// Override `github_oauth.enabled`
  pub komodo_github_oauth_enabled: Option<bool>,
  /// Override `github_oauth.id`
  pub komodo_github_oauth_id: Option<String>,
  /// Override `github_oauth.id` from file
  pub komodo_github_oauth_id_file: Option<PathBuf>,
  /// Override `github_oauth.secret`
  pub komodo_github_oauth_secret: Option<String>,
  /// Override `github_oauth.secret` from file
  pub komodo_github_oauth_secret_file: Option<PathBuf>,

  /// Override `github_webhook_app.app_id`
  pub komodo_github_webhook_app_app_id: Option<i64>,
  /// Override `github_webhook_app.app_id` from file
  pub komodo_github_webhook_app_app_id_file: Option<PathBuf>,
  /// Override `github_webhook_app.installations[i].id`. Accepts comma seperated list.
  ///
  /// Note. Paired by index with values in `komodo_github_webhook_app_installations_namespaces`
  pub komodo_github_webhook_app_installations_ids: Option<Vec<i64>>,
  /// Override `github_webhook_app.installations[i].id` from file
  pub komodo_github_webhook_app_installations_ids_file:
    Option<PathBuf>,
  /// Override `github_webhook_app.installations[i].namespace`. Accepts comma seperated list.
  ///
  /// Note. Paired by index with values in `komodo_github_webhook_app_installations_ids`
  pub komodo_github_webhook_app_installations_namespaces:
    Option<Vec<String>>,
  /// Override `github_webhook_app.pk_path`
  pub komodo_github_webhook_app_pk_path: Option<String>,

  /// Override `database.uri`
  #[serde(alias = "KOMODO_MONGO_URI")]
  pub komodo_database_uri: Option<String>,
  /// Override `database.uri` from file
  #[serde(alias = "KOMODO_MONGO_URI_FILE")]
  pub komodo_database_uri_file: Option<PathBuf>,
  /// Override `database.address`
  #[serde(alias = "KOMODO_MONGO_ADDRESS")]
  pub komodo_database_address: Option<String>,
  /// Override `database.username`
  #[serde(alias = "KOMODO_MONGO_USERNAME")]
  pub komodo_database_username: Option<String>,
  /// Override `database.username` with file
  #[serde(alias = "KOMODO_MONGO_USERNAME_FILE")]
  pub komodo_database_username_file: Option<PathBuf>,
  /// Override `database.password`
  #[serde(alias = "KOMODO_MONGO_PASSWORD")]
  pub komodo_database_password: Option<String>,
  /// Override `database.password` with file
  #[serde(alias = "KOMODO_MONGO_PASSWORD_FILE")]
  pub komodo_database_password_file: Option<PathBuf>,
  /// Override `database.app_name`
  #[serde(alias = "KOMODO_MONGO_APP_NAME")]
  pub komodo_database_app_name: Option<String>,
  /// Override `database.db_name`
  #[serde(alias = "KOMODO_MONGO_DB_NAME")]
  pub komodo_database_db_name: Option<String>,

  /// Override `aws.access_key_id`
  pub komodo_aws_access_key_id: Option<String>,
  /// Override `aws.access_key_id` with file
  pub komodo_aws_access_key_id_file: Option<PathBuf>,
  /// Override `aws.secret_access_key`
  pub komodo_aws_secret_access_key: Option<String>,
  /// Override `aws.secret_access_key` with file
  pub komodo_aws_secret_access_key_file: Option<PathBuf>,

  /// Override `hetzner.token`
  pub komodo_hetzner_token: Option<String>,
  /// Override `hetzner.token` with file
  pub komodo_hetzner_token_file: Option<PathBuf>,

  /// Override `ssl_enabled`.
  pub komodo_ssl_enabled: Option<bool>,
  /// Override `ssl_key`
  pub komodo_ssl_key: Option<PathBuf>,
  /// Override `ssl_cert`
  pub komodo_ssl_cert: Option<PathBuf>,

  /// Override `periphery_accept_self_signed_certs`
  pub komodo_periphery_accept_self_signed_certs: Option<bool>,
  /// Override `periphery_ca_cert_path`
  pub komodo_periphery_ca_cert_path: Option<PathBuf>,
  /// Override `periphery_ca_key_path`
  pub komodo_periphery_ca_key_path: Option<PathBuf>,
}

fn default_config_path() -> String {
  "/config/config.toml".to_string()
}

/// # Core Configuration File
///
/// The Core API initializes it's configuration by reading the environment,
/// parsing the [CoreConfig] schema from the file path specified by `env.komodo_config_path`,
/// and then applying any config field overrides specified in the environment.
///
/// *Note.* The Komodo Core docker image includes the default core configuration found at
/// [https://github.com/mbecker20/komodo/blob/main/config/core.config.toml](https://github.com/mbecker20/komodo/blob/main/config/core.config.toml).
/// To configure the core api, you can either mount your own custom configuration file to
/// `/config/config.toml` inside the container,
/// or simply override whichever fields you need using the environment.
///
/// Refer to the [example file](https://github.com/mbecker20/komodo/blob/main/config/core.config.toml) for a full example.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConfig {
  // ===========
  // = General =
  // ===========
  /// The title of this Komodo Core deployment. Will be used in the browser page title.
  /// Default: 'Komodo'
  #[serde(default = "default_title")]
  pub title: String,

  /// The host to use with oauth redirect url, whatever host
  /// the user hits to access Komodo. eg `https://komodo.domain.com`.
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

  /// Disable user ability to use the UI to update resource configuration.
  #[serde(default)]
  pub ui_write_disabled: bool,

  /// Disable the popup confirm dialogs. All buttons will just be double click.
  #[serde(default)]
  pub disable_confirm_dialog: bool,

  /// If defined, ensure an enabled first server exists at this address.
  /// Example: `http://periphery:8120`
  #[serde(default)]
  pub first_server: String,

  /// The path to the built frontend folder.
  #[serde(default = "default_frontend_path")]
  pub frontend_path: String,

  /// Configure database connection
  #[serde(alias = "mongo")]
  pub database: DatabaseConfig,

  // ================
  // = Auth / Login =
  // ================
  /// enable login with local auth
  #[serde(default)]
  pub local_auth: bool,

  /// Enable transparent mode, which gives all (enabled) users read access to all resources.
  #[serde(default)]
  pub transparent_mode: bool,

  /// New users will be automatically enabled.
  /// Combined with transparent mode, this is suitable for a demo instance.
  #[serde(default)]
  pub enable_new_users: bool,

  /// Normally new users will be registered, but not enabled until an Admin enables them.
  /// With `disable_user_registration = true`, only the first user to log in will registered as a user.
  #[serde(default)]
  pub disable_user_registration: bool,

  /// Normally all users can create resources.
  /// If `disable_non_admin_create = true`, only admins will be able to create resources.
  #[serde(default)]
  pub disable_non_admin_create: bool,

  /// Optionally provide a specific jwt secret.
  /// Passing nothing or an empty string will cause one to be generated.
  /// Default: "" (empty string)
  #[serde(default)]
  pub jwt_secret: String,

  /// Control how long distributed JWT remain valid for.
  /// Default: `1-day`.
  #[serde(default = "default_jwt_ttl")]
  pub jwt_ttl: Timelength,

  // =========
  // = Oauth =
  // =========
  /// Configure google oauth
  #[serde(default)]
  pub google_oauth: OauthCredentials,

  /// Configure github oauth
  #[serde(default)]
  pub github_oauth: OauthCredentials,

  // ============
  // = Webhooks =
  // ============
  /// Used to verify validity from webhooks.
  /// Should be some secure hash maybe 20-40 chars.
  /// It is given to git provider when configuring the webhook.
  #[serde(default)]
  pub webhook_secret: String,

  /// Override the webhook listener base url, if None will use the address defined as 'host'.
  /// Example: `https://webhooks.komo.do`
  ///
  /// This can be used if Komodo Core sits on an internal network which is
  /// unreachable directly from the open internet.
  /// A reverse proxy in a public network can forward webhooks to Komodo.
  pub webhook_base_url: Option<String>,

  /// Configure a Github Webhook app.
  /// Allows users to manage repo webhooks from within the Komodo UI.
  #[serde(default)]
  pub github_webhook_app: GithubWebhookAppConfig,

  // ===========
  // = Logging =
  // ===========
  /// Configure logging
  #[serde(default)]
  pub logging: LogConfig,

  // ===========
  // = Pruning =
  // ===========
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

  // ==================
  // = Poll Intervals =
  // ==================
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

  /// Interval at which to poll repo commit hash for any updates / automated actions.
  /// Options: `15-sec`, `1-min`, `5-min`, `15-min`, `1-hr`
  /// Default: `5-min`.  
  #[serde(default = "default_poll_interval")]
  pub repo_poll_interval: Timelength,

  /// Interval at which to collect server stats and send any alerts.
  /// Default: `15-sec`
  #[serde(default = "default_monitoring_interval")]
  pub monitoring_interval: Timelength,

  // ===================
  // = Cloud Providers =
  // ===================
  /// Configure AWS credentials to use with AWS builds / server launches.
  #[serde(default)]
  pub aws: AwsCredentials,

  /// Configure Hetzner credentials to use with Hetzner builds / server launches.
  #[serde(default)]
  pub hetzner: HetznerCredentials,

  // =================
  // = Git Providers =
  // =================
  /// Configure git credentials used to clone private repos.
  /// Supports any git provider.
  #[serde(default, alias = "git_provider")]
  pub git_providers: Vec<GitProvider>,

  // ======================
  // = Registry Providers =
  // ======================
  /// Configure docker credentials used to push / pull images.
  /// Supports any docker image repository.
  #[serde(default, alias = "docker_registry")]
  pub docker_registries: Vec<DockerRegistry>,

  // ===========
  // = Secrets =
  // ===========
  /// Configure core-based secrets. These will be preferentially interpolated into
  /// values if they contain a matching secret. Otherwise, the periphery will have to have the
  /// secret configured.
  #[serde(default)]
  pub secrets: HashMap<String, String>,

  // =========
  // = Other =
  // =========
  /// Specify the directory used to clone stack / repo / build repos, for latest hash / contents.
  /// The default is fine when using a container.
  /// This directory has no need for persistence, so no need to mount it.
  /// Default: `/repos`
  #[serde(default = "default_repo_directory")]
  pub repo_directory: PathBuf,

  /// Whether to enable ssl.
  #[serde(default)]
  pub ssl_enabled: bool,

  /// Path to the ssl key.
  /// Default: `/etc/komodo/ssl/key.pem`.
  #[serde(default = "default_ssl_key")]
  pub ssl_key: PathBuf,

  /// Path to the ssl cert.
  /// Default: `/etc/komodo/ssl/cert.pem`.
  #[serde(default = "default_ssl_cert")]
  pub ssl_cert: PathBuf,

  /// Whether to accept https communication with self signed certs.
  #[serde(default = "default_periphery_accept_self_signed_certs")]
  pub periphery_accept_self_signed_certs: bool,

  /// Path to periphery ca cert.
  #[serde(default = "default_periphery_ca_cert_path")]
  pub periphery_ca_cert_path: PathBuf,

  /// Path to periphery ca key.
  #[serde(default = "default_periphery_ca_key_path")]
  pub periphery_ca_key_path: PathBuf,
}

fn default_title() -> String {
  String::from("Komodo")
}

fn default_core_port() -> u16 {
  9120
}

fn default_frontend_path() -> String {
  "/app/frontend".to_string()
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

fn default_ssl_key() -> PathBuf {
  "/etc/komodo/ssl/core/key.pem".parse().unwrap()
}

fn default_ssl_cert() -> PathBuf {
  "/etc/komodo/ssl/core/cert.pem".parse().unwrap()
}

fn default_periphery_accept_self_signed_certs() -> bool {
  true
}

fn default_periphery_ca_cert_path() -> PathBuf {
  "/etc/komodo/ssl/periphery/ca.crt".parse().unwrap()
}

fn default_periphery_ca_key_path() -> PathBuf {
  "/etc/komodo/ssl/periphery/ca.key".parse().unwrap()
}

impl CoreConfig {
  pub fn sanitized(&self) -> CoreConfig {
    let config = self.clone();
    CoreConfig {
      title: config.title,
      host: config.host,
      port: config.port,
      passkey: empty_or_redacted(&config.passkey),
      first_server: config.first_server,
      frontend_path: config.frontend_path,
      jwt_secret: empty_or_redacted(&config.jwt_secret),
      jwt_ttl: config.jwt_ttl,
      repo_directory: config.repo_directory,
      sync_poll_interval: config.sync_poll_interval,
      stack_poll_interval: config.stack_poll_interval,
      build_poll_interval: config.build_poll_interval,
      repo_poll_interval: config.repo_poll_interval,
      monitoring_interval: config.monitoring_interval,
      keep_stats_for_days: config.keep_stats_for_days,
      keep_alerts_for_days: config.keep_alerts_for_days,
      logging: config.logging,
      transparent_mode: config.transparent_mode,
      ui_write_disabled: config.ui_write_disabled,
      disable_confirm_dialog: config.disable_confirm_dialog,
      enable_new_users: config.enable_new_users,
      disable_user_registration: config.disable_user_registration,
      disable_non_admin_create: config.disable_non_admin_create,
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
      database: DatabaseConfig {
        uri: config.database.uri.map(|cur| empty_or_redacted(&cur)),
        address: config.database.address,
        username: config
          .database
          .username
          .map(|cur| empty_or_redacted(&cur)),
        password: config
          .database
          .password
          .map(|cur| empty_or_redacted(&cur)),
        app_name: config.database.app_name,
        db_name: config.database.db_name,
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

      ssl_enabled: config.ssl_enabled,
      ssl_key: config.ssl_key,
      ssl_cert: config.ssl_cert,
      periphery_accept_self_signed_certs: config
        .periphery_accept_self_signed_certs,
      periphery_ca_cert_path: config.periphery_ca_cert_path,
      periphery_ca_key_path: config.periphery_ca_key_path,
    }
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

/// Provide database connection information.
/// Komodo uses the MongoDB api driver for database communication,
/// and FerretDB to support Postgres and Sqlite storage options.
///
/// Must provide ONE of:
/// 1. `uri`
/// 2. `address` + `username` + `password`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
  /// Full mongo uri string, eg. `mongodb://username:password@your.mongo.int:27017`
  pub uri: Option<String>,
  /// Just the address part of the mongo uri, eg `your.mongo.int:27017`
  pub address: Option<String>,
  /// Mongo user username
  pub username: Option<String>,
  /// Mongo user password
  pub password: Option<String>,
  /// Mongo app name. default: `komodo_core`
  #[serde(default = "default_core_database_app_name")]
  pub app_name: String,
  /// Mongo db name. Which mongo database to create the collections in.
  /// Default: `komodo`.
  #[serde(default = "default_core_database_db_name")]
  pub db_name: String,
}

fn default_core_database_app_name() -> String {
  "komodo_core".to_string()
}

fn default_core_database_db_name() -> String {
  "komodo".to_string()
}

impl Default for DatabaseConfig {
  fn default() -> Self {
    Self {
      uri: None,
      address: Some("localhost:27017".to_string()),
      username: None,
      password: None,
      app_name: default_core_database_app_name(),
      db_name: default_core_database_db_name(),
    }
  }
}

/// Provide AWS credentials for Komodo to use.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AwsCredentials {
  /// The aws ACCESS_KEY_ID
  pub access_key_id: String,
  /// The aws SECRET_ACCESS_KEY
  pub secret_access_key: String,
}

/// Provide Hetzner credentials for Komodo to use.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HetznerCredentials {
  pub token: String,
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
