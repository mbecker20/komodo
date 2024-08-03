//! # Configuring the periphery agent
//!
//! The periphery configuration is passed in three ways:
//! 1. Command line args ([CliArgs])
//! 2. Environment Variables ([Env])
//! 3. Configuration File ([PeripheryConfig])
//!
//! The final configuration is built by combining parameters
//! passed through the different methods. The priority of the args is
//! strictly hierarchical, meaning params passed with [CliArgs] have top priority,
//! followed by those passed in the environment, followed by those passed in
//! the configuration file.
//!

use std::{collections::HashMap, net::IpAddr, path::PathBuf};

use clap::Parser;
use serde::Deserialize;

use crate::entities::{
  logger::{LogConfig, LogLevel, StdioLogMode},
  Timelength,
};

use super::{DockerRegistry, GitProvider};

/// # Periphery Command Line Arguments.
///
/// This structure represents the periphery command line arguments used to
/// configure the periphery agent. A help manual for the periphery binary
/// can be printed using `/path/to/periphery --help`.
///
/// Example command:
/// ```sh
/// periphery \
///   --config-path /path/to/periphery.config.base.toml \
///   --config-path /other_path/to/overide-periphery-config-directory \
///   --config-keyword periphery \
///   --config-keyword config \
///   --merge-nested-config true \
///   --extend-config-arrays false \
///   --log-level info
/// ```
#[derive(Parser)]
#[command(name = "periphery", author, about, version)]
pub struct CliArgs {
  /// Sets the path of a config file or directory to use.
  /// Can use multiple times
  #[arg(short, long)]
  pub config_path: Option<Vec<String>>,

  /// Sets the keywords to match directory periphery config file names on.
  /// Can use multiple times.
  #[arg(long)]
  pub config_keyword: Option<Vec<String>>,

  /// Merges nested configs, eg. secrets, providers.
  /// Will override the equivalent env configuration.
  /// Default: false
  #[arg(long)]
  pub merge_nested_config: Option<bool>,

  /// Extends config arrays, eg. allowed_ips, passkeys.
  /// Will override the equivalent env configuration.
  /// Default: false
  #[arg(long)]
  pub extend_config_arrays: Option<bool>,

  /// Configure the logging level: error, warn, info, debug, trace.
  /// Default: info
  /// If passed, will override any other log_level set.
  #[arg(long)]
  pub log_level: Option<tracing::Level>,
}

/// # Periphery Environment Variables
///
/// The variables should be passed in the traditional `UPPER_SNAKE_CASE` format,
/// although the lower case format can still be parsed. If equivalent paramater is passed
/// in [CliArgs], the value passed to the environment will be ignored in favor of the cli arg.
#[derive(Deserialize)]
pub struct Env {
  /// Specify the config paths (files or folders) used to build up the
  /// final [PeripheryConfig].
  /// Default: `~/.config/monitor/periphery.config.toml`.
  ///
  /// Note. This is overridden if the equivalent arg is passed in [CliArgs].
  #[serde(default = "default_config_paths")]
  pub monitor_config_paths: Vec<String>,
  /// If specifying folders, use this to narrow down which
  /// files will be matched to parse into the final [PeripheryConfig].
  /// Only files inside the folders which have names containing all keywords
  /// provided to `config_keywords` will be included.
  ///
  /// Note. This is overridden if the equivalent arg is passed in [CliArgs].
  #[serde(default)]
  pub monitor_config_keywords: Vec<String>,

  /// Will merge nested config object (eg. secrets, providers) across multiple
  /// config files. Default: `false`
  ///
  /// Note. This is overridden if the equivalent arg is passed in [CliArgs].
  #[serde(default)]
  pub monitor_merge_nested_config: bool,

  /// Will extend config arrays (eg. `allowed_ips`, `passkeys`) across multiple config files.
  /// Default: `false`
  ///
  /// Note. This is overridden if the equivalent arg is passed in [CliArgs].
  #[serde(default)]
  pub monitor_extend_config_arrays: bool,

  /// Override `port`
  pub monitor_port: Option<u16>,
  /// Override `repo_dir`
  pub monitor_repo_dir: Option<PathBuf>,
  /// Override `stack_dir`
  pub monitor_stack_dir: Option<PathBuf>,
  /// Override `stats_polling_rate`
  pub monitor_stats_polling_rate: Option<Timelength>,
  /// Override `legacy_compose_cli`
  pub monitor_legacy_compose_cli: Option<bool>,

  // LOGGING
  /// Override `logging.level`
  pub monitor_logging_level: Option<LogLevel>,
  /// Override `logging.stdio`
  pub monitor_logging_stdio: Option<StdioLogMode>,
  /// Override `logging.otlp_endpoint`
  pub monitor_logging_otlp_endpoint: Option<String>,
  /// Override `logging.opentelemetry_service_name`
  pub monitor_logging_opentelemetry_service_name: Option<String>,

  /// Override `allowed_ips`
  pub monitor_allowed_ips: Option<Vec<IpAddr>>,
  /// Override `passkeys`
  pub monitor_passkeys: Option<Vec<String>>,
}

fn default_config_paths() -> Vec<String> {
  vec!["~/.config/monitor/periphery.config.toml".to_string()]
}

/// # Periphery Configuration File
///
/// The periphery agent initializes it's configuration by reading the environment,
/// parsing the [PeripheryConfig] schema from the files specified by cli args (and falling back to `env.config_paths`),
/// and then applying any config field overrides specified in the environment.
///
/// ## Example TOML
/// ```toml
/// ## optional. 8120 is default
/// port = 8120
///
/// ## optional. `/etc/monitor/repos` is default.
/// repo_dir = "/etc/monitor/repos"
/// 
/// ## optional. `/etc/monitor/stacks` is default.
/// stack_dir = "/etc/monitor/stacks"
///
/// ## optional. 5-sec is default.
/// ## can use 1-sec, 5-sec, 10-sec, 30-sec, 1-min.
/// ## controls granularity of system stats recorded
/// stats_polling_rate = "5-sec"
///
/// ## Whether stack actions should use `docker-compose ...`
/// ## instead of `docker compose ...`.
/// ## default: false
/// legacy_compose_cli = false
///
/// ## optional. default is empty, which will not block any request by ip.
/// allowed_ips = ["127.0.0.1"]
///
/// ## optional. default is empty, which will not require any passkey to be passed by core.
/// passkeys = ["abcdefghijk"]
///
/// ## specify the log level of the monitor core application
/// ## default: info
/// ## options: off, error, warn, info, debug, trace
/// logging.level = "info"
///
/// ## specify the logging format for stdout / stderr.
/// ## default: standard
/// ## options: standard, json, none
/// logging.stdio = "standard"
///
/// ## specify an otlp endpoint to send traces to
/// ## optional, default unassigned
/// # logging.otlp_endpoint = "http://localhost:4317"
///
/// ## specify the service name to send with otlp traces.
/// ## optional, default 'Monitor'.
/// # logging.opentelemetry_service_name = "Monitor"
///
/// ## configure perihery-based secrets
/// [secrets]
/// # SECRET_1 = "value_1"
/// # SECRET_2 = "value_2"
///
/// ## configure periphery-based git providers
/// # [[git_provider]]
/// # domain = "git.mogh.tech" # use a custom provider, like self-hosted gitea
/// # accounts = [
/// #     { username = "mbecker20", token = "access_token_for_account" },
/// # ]
///
/// ## configure periphery-based docker registries
/// # [[docker_registry]]
/// # domain = "docker.io"
/// # accounts = [
/// #     { username = "mbecker2020", token = "access_token_for_account" }
/// # ]
/// # organizations = ["DockerhubOrganization"]
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct PeripheryConfig {
  /// The port periphery will run on.
  /// Default: `8120`
  #[serde(default = "default_periphery_port")]
  pub port: u16,

  /// The system directory where monitor managed repos will be cloned.
  /// Default: `/etc/monitor/repos`
  #[serde(default = "default_repo_dir")]
  pub repo_dir: PathBuf,

  /// The system directory where stacks will managed.
  /// Default: `/etc/monitor/stacks`
  #[serde(default = "default_stack_dir")]
  pub stack_dir: PathBuf,

  /// The rate at which the system stats will be polled to update the cache.
  /// Default: `5-sec`
  #[serde(default = "default_stats_refresh_interval")]
  pub stats_polling_rate: Timelength,

  /// Whether stack actions should use `docker-compose ...`
  /// instead of `docker compose ...`.
  /// Default: false
  #[serde(default)]
  pub legacy_compose_cli: bool,

  /// Logging configuration
  #[serde(default)]
  pub logging: LogConfig,

  /// Limits which IPv4 addresses are allowed to call the api.
  /// Default: none
  ///
  /// Note: this should be configured to increase security.
  #[serde(default)]
  pub allowed_ips: Vec<IpAddr>,

  /// Limits the accepted passkeys.
  /// Default: none
  ///
  /// Note: this should be configured to increase security.
  #[serde(default)]
  pub passkeys: Vec<String>,

  /// Mapping on local periphery secrets. These can be interpolated into eg. Deployment environment variables.
  /// Default: none
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
}

fn default_periphery_port() -> u16 {
  8120
}

fn default_repo_dir() -> PathBuf {
  "/etc/monitor/repos".parse().unwrap()
}

fn default_stack_dir() -> PathBuf {
  "/etc/monitor/stacks".parse().unwrap()
}

fn default_stats_refresh_interval() -> Timelength {
  Timelength::FiveSeconds
}
