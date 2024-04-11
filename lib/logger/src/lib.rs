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

  match config.stdio {
    StdioLogMode::Standard => tracing_subscriber::fmt()
      .with_max_level(LevelFilter::from(log_level))
      .init(),
    StdioLogMode::Json => tracing_subscriber::fmt()
      .with_max_level(LevelFilter::from(log_level))
      .json()
      .init(),
    StdioLogMode::None => {}
  }

  // Create loki subscriber
  if let Some(loki_url) = &config.loki_url {
    let (loki_layer, task) = tracing_loki::builder()
      .label("host", "mine")?
      .build_url(Url::parse(loki_url)?)?;
    tokio::spawn(task);
    tracing_subscriber::registry()
      .with(LevelFilter::from(log_level))
      .with(loki_layer)
      .try_init()
      .context("failed to init logger")?;
  }

  Ok(())
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
