use std::{collections::HashMap, net::IpAddr, path::PathBuf};

use serde::Deserialize;

use crate::entities::{
  logger::{LogConfig, LogLevel, StdioLogMode},
  Timelength,
};

#[derive(Deserialize)]
pub struct Env {
  #[serde(default = "default_config_paths")]
  pub config_paths: Vec<String>,
  #[serde(default)]
  pub config_keywords: Vec<String>,

  // Overrides
  pub port: Option<u16>,
  pub log_level: Option<LogLevel>,
  pub stdio_log_mode: Option<StdioLogMode>,
}

fn default_config_paths() -> Vec<String> {
  vec!["~/.config/monitor/periphery.config.toml".to_string()]
}

#[derive(Deserialize, Debug, Clone)]
pub struct PeripheryConfig {
  /// The port periphery will run on
  #[serde(default = "default_periphery_port")]
  pub port: u16,

  /// Configure the logging level: error, warn, info, debug, trace
  #[serde(default)]
  pub log_level: LogLevel,

  /// The system directory where monitor managed repos will be cloned
  #[serde(default = "default_repo_dir")]
  pub repo_dir: PathBuf,

  /// The rate at which the system stats will be polled to update the cache
  #[serde(default = "default_stats_refresh_interval")]
  pub stats_polling_rate: Timelength,

  /// Logging configuration
  #[serde(default)]
  pub logging: LogConfig,

  /// Limits which IPv4 addresses are allowed to call the api
  #[serde(default)]
  pub allowed_ips: Vec<IpAddr>,

  /// Limits the accepted passkeys
  #[serde(default)]
  pub passkeys: Vec<String>,

  /// Mapping on local periphery secrets. These can be interpolated into eg. Deployment environment variables.
  #[serde(default)]
  pub secrets: HashMap<String, String>,

  /// Mapping of github usernames to access tokens
  #[serde(default)]
  pub github_accounts: HashMap<String, String>,

  /// Mapping of docker usernames to access tokens
  #[serde(default)]
  pub docker_accounts: HashMap<String, String>,
}

fn default_periphery_port() -> u16 {
  8120
}

fn default_repo_dir() -> PathBuf {
  "/repos".parse().unwrap()
}

fn default_stats_refresh_interval() -> Timelength {
  Timelength::FiveSeconds
}
