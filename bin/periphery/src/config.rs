use std::sync::OnceLock;

use clap::Parser;
use merge_config_files::parse_config_paths;
use monitor_client::entities::config::periphery::{
  Env, PeripheryConfig,
};
use serde_json::json;

#[derive(Parser)]
#[command(author, about, version)]
struct CliArgs {
  /// Sets the path of a config file or directory to use. can use multiple times
  #[arg(short, long)]
  pub config_path: Option<Vec<String>>,

  /// Sets the keywords to match directory periphery config file names on.
  /// can use multiple times. default "periphery" and "config"
  #[arg(long)]
  pub config_keyword: Option<Vec<String>>,

  /// Merges nested configs, eg. secrets, docker_accounts, github_accounts
  #[arg(long)]
  pub merge_nested_config: bool,

  /// Extends config arrays, eg. allowed_ips, passkeys
  #[arg(long)]
  pub extend_config_arrays: bool,

  /// Configure the logging level: error, warn, info, debug, trace
  #[arg(long, default_value_t = tracing::Level::INFO)]
  pub log_level: tracing::Level,
}

pub fn periphery_config() -> &'static PeripheryConfig {
  static PERIPHERY_CONFIG: OnceLock<PeripheryConfig> =
    OnceLock::new();
  PERIPHERY_CONFIG.get_or_init(|| {
    let env: Env = envy::from_env()
      .expect("failed to parse periphery environment");
    let args = CliArgs::parse();
    let config_paths = args.config_path.unwrap_or(env.config_paths);
    let match_keywords =
      args.config_keyword.unwrap_or(env.config_keywords);
    let mut config = parse_config_paths::<PeripheryConfig>(
      config_paths,
      match_keywords,
      args.merge_nested_config,
      args.extend_config_arrays,
    )
    .expect("failed at parsing config from paths");

    // Overrides
    config.port = env.port.unwrap_or(config.port);
    config.logging.level =
      env.log_level.unwrap_or(config.logging.level);
    config.logging.stdio =
      env.stdio_log_mode.unwrap_or(config.logging.stdio);

    config
  })
}

pub fn accounts_response() -> &'static String {
  static ACCOUNTS_RESPONSE: OnceLock<String> = OnceLock::new();
  ACCOUNTS_RESPONSE.get_or_init(|| json!({
    "docker": periphery_config().docker_accounts.keys().collect::<Vec<_>>(),
    "github": periphery_config().github_accounts.keys().collect::<Vec<_>>(),
  }).to_string())
}

pub fn secrets_response() -> &'static String {
  static SECRETS_RESPONSE: OnceLock<String> = OnceLock::new();
  SECRETS_RESPONSE.get_or_init(|| {
    serde_json::to_string(
      &periphery_config().secrets.keys().collect::<Vec<_>>(),
    )
    .unwrap()
  })
}
