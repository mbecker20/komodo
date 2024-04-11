use std::time::Duration;

use anyhow::Context;
use opentelemetry_otlp::WithExportConfig;
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
  layer::SubscriberExt, util::SubscriberInitExt,
};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LogConfig {
  /// The logging level. default: info
  #[serde(default)]
  pub level: LogLevel,

  /// Controls logging to stdout / stderr
  #[serde(default)]
  pub stdio: StdioLogMode,

  /// Enable opentelemetry experting
  pub otlp_endpoint: Option<String>,
}

macro_rules! opentelemetry_layer {
  ($endpoint:expr) => {{
    let tracer = opentelemetry_otlp::new_pipeline()
      .tracing()
      .with_exporter(
        opentelemetry_otlp::new_exporter()
          .tonic()
          .with_endpoint($endpoint)
          .with_timeout(Duration::from_secs(3)),
      )
      .install_batch(opentelemetry_sdk::runtime::Tokio)?;
    tracing_opentelemetry::layer().with_tracer(tracer)
  }};
}

pub fn init(config: &LogConfig) -> anyhow::Result<()> {
  let log_level: tracing::Level = config.level.into();

  let registry =
    tracing_subscriber::registry().with(LevelFilter::from(log_level));

  match (config.stdio, &config.otlp_endpoint) {
    (StdioLogMode::Standard, Some(endpoint)) => registry
      .with(tracing_subscriber::fmt::layer())
      .with(opentelemetry_layer!(endpoint))
      .try_init()
      .context("failed to init logger"),
    (StdioLogMode::Json, Some(endpoint)) => registry
      .with(tracing_subscriber::fmt::layer().json())
      .with(opentelemetry_layer!(endpoint))
      .try_init()
      .context("failed to init logger"),
    (StdioLogMode::None, Some(endpoint)) => registry
      .with(opentelemetry_layer!(endpoint))
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
