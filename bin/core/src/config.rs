use anyhow::Context;
use merge_config_files::parse_config_file;
use monitor_client::entities::config::{CoreConfig, LogLevel};
use serde::Deserialize;

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

impl Env {
  pub fn load() -> anyhow::Result<Env> {
    dotenv::dotenv().ok();
    envy::from_env().context("failed to parse environment")
  }
}

pub fn config_load(config_path: &str) -> CoreConfig {
  parse_config_file::<CoreConfig>(config_path).unwrap_or_else(|e| {
    panic!("failed at parsing config at {config_path} | {e:#?}")
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
