use anyhow::Context;
use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;
use tracing_loki::url::Url;
use tracing_subscriber::{
  layer::SubscriberExt, util::SubscriberInitExt,
};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct LogConfig {
  /// Whether to log to stdout / stderr with tracing_subscriber::fmt().init(). default: true.
  #[serde(default = "default_stdio")]
  pub stdio: bool,

  /// The logging level. default: info
  #[serde(default)]
  pub level: LogLevel,

  /// Send tracing logs to loki
  pub loki_url: Option<String>,
}

fn default_stdio() -> bool {
  true
}

pub fn init(config: LogConfig) -> anyhow::Result<()> {
  let log_level: tracing::Level = config.level.into();

  match (config.stdio, config.loki_url) {
    // Both loki and stdio
    (true, Some(loki_url)) => {
      let (loki_layer, task) = tracing_loki::builder()
        .label("host", "mine")?
        .build_url(Url::parse(&loki_url)?)?;
      tokio::spawn(task);

      tracing_subscriber::registry()
        .with(LevelFilter::from(log_level))
        .with(tracing_subscriber::fmt::Layer::new())
        .with(loki_layer)
        .try_init()
        .context("failed to init logger")
    }

    // Just stdio
    (true, None) => tracing_subscriber::registry()
      .with(LevelFilter::from(log_level))
      .with(tracing_subscriber::fmt::Layer::new())
      .try_init()
      .context("failed to init logger"),

    // Just loki
    (false, Some(loki_url)) => {
      let (loki_layer, task) = tracing_loki::builder()
        .label("host", "mine")?
        .build_url(Url::parse(&loki_url)?)?;
      tokio::spawn(task);
      tracing_subscriber::registry()
        .with(LevelFilter::from(log_level))
        .with(loki_layer)
        .try_init()
        .context("failed to init logger")
    }

    // Neither (not recommended)
    (false, None) => tracing_subscriber::registry()
      .with(LevelFilter::from(log_level))
      .try_init()
      .context("failed to init logger"),
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
