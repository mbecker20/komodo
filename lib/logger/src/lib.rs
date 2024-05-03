use anyhow::Context;
use monitor_client::entities::logger::{LogConfig, StdioLogMode};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
  layer::SubscriberExt, util::SubscriberInitExt,
};

mod opentelemetry;

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
      .try_init(),
    (StdioLogMode::Json, Some(endpoint)) => registry
      .with(tracing_subscriber::fmt::layer().json())
      .with(opentelemetry::layer(
        endpoint,
        config.opentelemetry_service_name.clone(),
      ))
      .try_init(),
    (StdioLogMode::None, Some(endpoint)) => registry
      .with(opentelemetry::layer(
        endpoint,
        config.opentelemetry_service_name.clone(),
      ))
      .try_init(),

    (StdioLogMode::Standard, None) => {
      registry.with(tracing_subscriber::fmt::layer()).try_init()
    }
    (StdioLogMode::Json, None) => registry
      .with(tracing_subscriber::fmt::layer().json())
      .try_init(),
    (StdioLogMode::None, None) => Ok(()),
  }
  .context("failed to init logger")
}
