use std::sync::OnceLock;

use clap::Parser;
use merge_config_files::parse_config_paths;
use monitor_client::entities::{
  config::periphery::{CliArgs, Env, PeripheryConfig},
  logger::{LogConfig, LogLevel},
};

pub fn periphery_config() -> &'static PeripheryConfig {
  static PERIPHERY_CONFIG: OnceLock<PeripheryConfig> =
    OnceLock::new();
  PERIPHERY_CONFIG.get_or_init(|| {
    let env: Env = envy::from_env()
      .expect("failed to parse periphery environment");
    let args = CliArgs::parse();
    let config_paths =
      args.config_path.unwrap_or(env.periphery_config_paths);
    let config = if config_paths.is_empty() {
      PeripheryConfig::default()
    } else {
      parse_config_paths::<PeripheryConfig>(
        config_paths,
        args.config_keyword.unwrap_or(env.periphery_config_keywords),
        args
          .merge_nested_config
          .unwrap_or(env.periphery_merge_nested_config),
        args
          .extend_config_arrays
          .unwrap_or(env.periphery_extend_config_arrays),
      )
      .expect("failed at parsing config from paths")
    };

    PeripheryConfig {
      port: env.periphery_port.unwrap_or(config.port),
      repo_dir: env.periphery_repo_dir.unwrap_or(config.repo_dir),
      stack_dir: env.periphery_stack_dir.unwrap_or(config.stack_dir),
      stats_polling_rate: env
        .periphery_stats_polling_rate
        .unwrap_or(config.stats_polling_rate),
      legacy_compose_cli: env
        .periphery_legacy_compose_cli
        .unwrap_or(config.legacy_compose_cli),
      logging: LogConfig {
        level: args
          .log_level
          .map(LogLevel::from)
          .or(env.periphery_logging_level)
          .unwrap_or(config.logging.level),
        stdio: env
          .periphery_logging_stdio
          .unwrap_or(config.logging.stdio),
        otlp_endpoint: env
          .periphery_logging_otlp_endpoint
          .or(config.logging.otlp_endpoint),
        opentelemetry_service_name: env
          .periphery_logging_opentelemetry_service_name
          .unwrap_or(config.logging.opentelemetry_service_name),
      },
      allowed_ips: env
        .periphery_allowed_ips
        .unwrap_or(config.allowed_ips),
      passkeys: env.periphery_passkeys.unwrap_or(config.passkeys),
      secrets: config.secrets,
      git_providers: config.git_providers,
      docker_registries: config.docker_registries,
    }
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
  static DOCKER_REGISTRIES_RESPONSE: OnceLock<String> =
    OnceLock::new();
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
