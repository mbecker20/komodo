use serde::{Deserialize, Serialize};
use tracing::level_filters::LevelFilter;

pub fn init(log_level: impl Into<tracing::Level>) {
  let log_level: tracing::Level = log_level.into();
  tracing_subscriber::fmt()
    .with_max_level(LevelFilter::from(log_level))
    .init()
}

#[derive(
  Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,
)]
pub enum LogLevel {
  Trace,
  Debug,
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
