use anyhow::Context;
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
  layer::SubscriberExt, util::SubscriberInitExt,
};

mod opentelemetry;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LogConfig {
  /// The logging level. default: info
  #[serde(default)]
  pub level: LogLevel,

  /// Controls logging to stdout / stderr
  #[serde(default)]
  pub stdio: StdioLogMode,

  /// Enable opentelemetry exporting
  pub otlp_endpoint: Option<String>,

  #[serde(default = "default_opentelemetry_service_name")]
  pub opentelemetry_service_name: String,
}

fn default_opentelemetry_service_name() -> String {
  String::from("Monitor")
}

pub fn init(config: &LogConfig) -> anyhow::Result<()> {
  let log_level: tracing::Level = config.level.into();

  let registry =
    tracing_subscriber::registry().with(LevelFilter::from(log_level));

  match (config.stdio, &config.otlp_endpoint) {
    (StdioLogMode::Standard, Some(endpoint)) => registry
      .with(tracing_subscriber::fmt::layer())
      .with(opentelemetry::layer(
        endpoint,
        config.opentelemetry_service_name.clone(),
      ))
      .try_init()
      .context("failed to init logger"),
    (StdioLogMode::Json, Some(endpoint)) => registry
      .with(tracing_subscriber::fmt::layer().json())
      .with(opentelemetry::layer(
        endpoint,
        config.opentelemetry_service_name.clone(),
      ))
      .try_init()
      .context("failed to init logger"),
    (StdioLogMode::None, Some(endpoint)) => registry
      .with(opentelemetry::layer(
        endpoint,
        config.opentelemetry_service_name.clone(),
      ))
      .try_init()
      .context("failed to init logger"),

    (StdioLogMode::Standard, None) => registry
      .with(tracing_subscriber::fmt::layer())
      .try_init()
      .context("failed to init logger"),
    (StdioLogMode::Json, None) => registry
      .with(tracing_subscriber::fmt::layer().json())
      .try_init()
      .context("failed to init logger"),
    (StdioLogMode::None, None) => Ok(()),
  }
}

#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum LogLevel {
  Trace,
  Debug,
  #[default]
  Info,
  Warn,
  Error,
}

impl From<LogLevel> for tracing::Level {
  fn from(value: LogLevel) -> Self {
    match value {
      LogLevel::Trace => tracing::Level::TRACE,
      LogLevel::Debug => tracing::Level::DEBUG,
      LogLevel::Info => tracing::Level::INFO,
      LogLevel::Warn => tracing::Level::WARN,
      LogLevel::Error => tracing::Level::ERROR,
    }
  }
}

#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
  Eq,
  Hash,
  Serialize,
  Deserialize,
)]
#[serde(rename_all = "lowercase")]
pub enum StdioLogMode {
  #[default]
  Standard,
  Json,
  None,
}
