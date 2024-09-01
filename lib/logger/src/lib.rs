use anyhow::Context;
use komodo_client::entities::logger::{LogConfig, StdioLogMode};
use tracing::level_filters::LevelFilter;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{
  layer::SubscriberExt, util::SubscriberInitExt, Registry,
};

mod otel;

pub fn init(config: &LogConfig) -> anyhow::Result<()> {
  let log_level: tracing::Level = config.level.into();

  let registry =
    Registry::default().with(LevelFilter::from(log_level));

  match (config.stdio, &config.otlp_endpoint) {
    (StdioLogMode::Standard, Some(endpoint)) => {
      let tracer = otel::tracer(
        endpoint,
        config.opentelemetry_service_name.clone(),
      );
      registry
        .with(tracing_subscriber::fmt::layer())
        .with(OpenTelemetryLayer::new(tracer))
        .try_init()
    }

    (StdioLogMode::Json, Some(endpoint)) => {
      let tracer = otel::tracer(
        endpoint,
        config.opentelemetry_service_name.clone(),
      );
      registry
        .with(tracing_subscriber::fmt::layer().json())
        .with(OpenTelemetryLayer::new(tracer))
        .try_init()
    }

    (StdioLogMode::None, Some(endpoint)) => {
      let tracer = otel::tracer(
        endpoint,
        config.opentelemetry_service_name.clone(),
      );
      registry.with(OpenTelemetryLayer::new(tracer)).try_init()
    }

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
