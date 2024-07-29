use std::sync::OnceLock;

use clap::Parser;
use merge_config_files::parse_config_paths;
use monitor_client::entities::{
  config::periphery::{CliArgs, Env, PeripheryConfig},
  logger::LogLevel,
};

pub fn periphery_config() -> &'static PeripheryConfig {
  static PERIPHERY_CONFIG: OnceLock<PeripheryConfig> =
    OnceLock::new();
  PERIPHERY_CONFIG.get_or_init(|| {
    let env: Env = envy::from_env()
      .expect("failed to parse periphery environment");
    let args = CliArgs::parse();
    let mut config = parse_config_paths::<PeripheryConfig>(
      args.config_path.unwrap_or(env.monitor_config_paths),
      args.config_keyword.unwrap_or(env.monitor_config_keywords),
      args
        .merge_nested_config
        .unwrap_or(env.monitor_merge_nested_config),
      args
        .extend_config_arrays
        .unwrap_or(env.monitor_extend_config_arrays),
    )
    .expect("failed at parsing config from paths");

    // Overrides
    config.port = env.monitor_port.unwrap_or(config.port);
    config.repo_dir = env.monitor_repo_dir.unwrap_or(config.repo_dir);
    config.stats_polling_rate = env
      .monitor_stats_polling_rate
      .unwrap_or(config.stats_polling_rate);

    // logging
    config.logging.level = args
      .log_level
      .map(LogLevel::from)
      .or(env.monitor_logging_level)
      .unwrap_or(config.logging.level);
    config.logging.stdio =
      env.monitor_logging_stdio.unwrap_or(config.logging.stdio);
    config.logging.otlp_endpoint = env
      .monitor_logging_otlp_endpoint
      .or(config.logging.otlp_endpoint);
    config.logging.opentelemetry_service_name = env
      .monitor_logging_opentelemetry_service_name
      .unwrap_or(config.logging.opentelemetry_service_name);

    config.allowed_ips =
      env.monitor_allowed_ips.unwrap_or(config.allowed_ips);
    config.passkeys = env.monitor_passkeys.unwrap_or(config.passkeys);

    config
  })
}

pub fn git_providers_response() -> &'static String {
  static GIT_PROVIDERS_RESPONSE: OnceLock<String> = OnceLock::new();
  GIT_PROVIDERS_RESPONSE.get_or_init(|| {
    let config = periphery_config();
    serde_json::to_string(&config.git_providers).unwrap()
  })
}

pub fn docker_registries_response() -> &'static String {
  static DOCKER_REGISTRIES_RESPONSE: OnceLock<String> = OnceLock::new();
  DOCKER_REGISTRIES_RESPONSE.get_or_init(|| {
    let config = periphery_config();
    serde_json::to_string(&config.docker_registries).unwrap()
  })
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
