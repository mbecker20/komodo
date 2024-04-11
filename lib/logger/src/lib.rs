use anyhow::Context;
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;
use tracing_loki::url::Url;
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

  /// Send tracing logs to loki
  pub loki_url: Option<String>,
}

pub fn init(config: &LogConfig) -> anyhow::Result<()> {
  let log_level: tracing::Level = config.level.into();

  let registry =
    tracing_subscriber::registry().with(LevelFilter::from(log_level));

  match (config.stdio, &config.loki_url) {
    (StdioLogMode::Standard, Some(loki_url)) => registry
      .with(loki_layer(loki_url)?)
      .with(tracing_subscriber::fmt::layer())
      .try_init()
      .context("failed to init logger"),
    (StdioLogMode::Json, Some(loki_url)) => registry
      .with(loki_layer(loki_url)?)
      .with(tracing_subscriber::fmt::layer().json())
      .try_init()
      .context("failed to init logger"),
    (StdioLogMode::None, Some(loki_url)) => registry
      .with(loki_layer(loki_url)?)
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

fn loki_layer(loki_url: &str) -> anyhow::Result<tracing_loki::Layer> {
  let (layer, task) = tracing_loki::builder()
    .label("host", "mine")?
    .build_url(Url::parse(loki_url)?)?;
  tokio::spawn(task);
  Ok(layer)
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
