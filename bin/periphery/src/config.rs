use std::{collections::HashMap, net::IpAddr, path::PathBuf};

use anyhow::Context;
use clap::Parser;
use merge_config_files::parse_config_paths;
use monitor_client::entities::Timelength;
use parse_csl::parse_comma_seperated;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(author, about, version)]
pub struct CliArgs {
  /// Sets the path of a config file or directory to use. can use multiple times
  #[arg(short, long)]
  pub config_path: Option<Vec<String>>,

  /// Sets the keywords to match directory periphery config file names on. can use multiple times. default "periphery" and "config"
  #[arg(long)]
  pub config_keyword: Option<Vec<String>>,

  /// Merges nested configs, eg. secrets, docker_accounts, github_accounts
  #[arg(long)]
  pub merge_nested_config: bool,

  /// Extends config arrays, eg. allowed_ips, passkeys
  #[arg(long)]
  pub extend_config_arrays: bool,

  /// Configure the logging level: error, warn, info, debug, trace
  #[arg(long, default_value_t = log::LevelFilter::Info)]
  pub log_level: log::LevelFilter,
}

#[derive(Deserialize)]
pub struct Env {
  #[serde(default = "default_config_path")]
  config_paths: String,
  #[serde(default)]
  config_keywords: String,
  port: Option<u16>,
}

impl Env {
  pub fn load() -> anyhow::Result<Env> {
    dotenv::dotenv().ok();
    envy::from_env().context("failed to parse environment")
  }
}

fn default_config_path() -> String {
  "~/.config/monitor.periphery.config.toml".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PeripheryConfig {
  #[serde(default = "default_periphery_port")]
  pub port: u16,
  #[serde(default = "default_repo_dir")]
  pub repo_dir: PathBuf,
  #[serde(default = "default_stats_refresh_interval")]
  pub stats_polling_rate: Timelength,
  #[serde(default)]
  pub allowed_ips: Vec<IpAddr>,
  #[serde(default)]
  pub passkeys: Vec<String>,
  #[serde(default)]
  pub secrets: HashMap<String, String>,
  #[serde(default)]
  pub github_accounts: HashMap<String, String>,
  #[serde(default)]
  pub docker_accounts: HashMap<String, String>,
}

impl PeripheryConfig {
  pub fn load(
    env: &Env,
    args: &CliArgs,
  ) -> anyhow::Result<PeripheryConfig> {
    let env_config_paths = parse_comma_seperated(&env.config_paths)
            .context("failed to parse config paths on environment into comma seperated list")?;
    let config_paths = args
      .config_path
      .as_ref()
      .unwrap_or(&env_config_paths)
      .to_vec();
    let env_match_keywords = parse_comma_seperated::<String>(&env.config_keywords)
            .context("failed to parse environemt CONFIG_KEYWORDS into comma seperated list")?;
    let match_keywords = args
      .config_keyword
      .as_ref()
      .unwrap_or(&env_match_keywords)
      .iter()
      .map(|kw| kw.as_str());
    let mut config = parse_config_paths::<PeripheryConfig>(
      config_paths,
      match_keywords,
      args.merge_nested_config,
      args.extend_config_arrays,
    )
    .expect("failed at parsing config from paths");
    if let Some(port) = env.port {
      config.port = port;
    }
    Ok(config)
  }
}

fn default_periphery_port() -> u16 {
  8000
}

fn default_repo_dir() -> PathBuf {
  "/repos".parse().unwrap()
}

fn default_stats_refresh_interval() -> Timelength {
  Timelength::FiveSeconds
}
