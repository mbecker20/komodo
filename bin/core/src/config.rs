use std::sync::OnceLock;

use merge_config_files::parse_config_file;
use monitor_client::entities::config::{CoreConfig, LogLevel};
use serde::Deserialize;

pub fn env() -> &'static Env {
  static ENV: OnceLock<Env> = OnceLock::new();
  ENV.get_or_init(|| {
    envy::from_env().expect("failed to parse environment")
  })
}

#[derive(Deserialize, Debug)]
pub struct Env {
  #[serde(default = "default_config_path")]
  pub config_path: String,
  #[serde(default = "default_frontend_path")]
  pub frontend_path: String,
  pub port: Option<u16>,
}

fn default_config_path() -> String {
  "/config/config.toml".to_string()
}

fn default_frontend_path() -> String {
  "/frontend".to_string()
}

pub fn core_config() -> &'static CoreConfig {
  static CORE_CONFIG: OnceLock<CoreConfig> = OnceLock::new();
  CORE_CONFIG.get_or_init(|| {
    let env = &env();
    let config_path = &env.config_path;
    let mut config =
      parse_config_file::<CoreConfig>(config_path.as_str())
        .unwrap_or_else(|e| {
          panic!("failed at parsing config at {config_path} | {e:#?}")
        });
    if let Some(port) = env.port {
      config.port = port;
    }
    config
  })
}

pub fn into_log_level(value: LogLevel) -> tracing::Level {
  match value {
    LogLevel::Error => tracing::Level::ERROR,
    LogLevel::Warn => tracing::Level::WARN,
    LogLevel::Info => tracing::Level::INFO,
    LogLevel::Debug => tracing::Level::DEBUG,
    LogLevel::Trace => tracing::Level::TRACE,
  }
}
