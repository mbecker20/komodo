use std::time::Duration;

use opentelemetry::{global, trace::TracerProvider, KeyValue};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
  runtime,
  trace::{BatchConfig, Sampler, Tracer},
  Resource,
};
use opentelemetry_semantic_conventions::{
  resource::{SERVICE_NAME, SERVICE_VERSION},
  SCHEMA_URL,
};

fn resource(service_name: String) -> Resource {
  Resource::from_schema_url(
    [
      KeyValue::new(SERVICE_NAME, service_name),
      KeyValue::new(SERVICE_VERSION, env!("CARGO_PKG_VERSION")),
    ],
    SCHEMA_URL,
  )
}

pub fn tracer(endpoint: &str, service_name: String) -> Tracer {
  let provider = opentelemetry_otlp::new_pipeline()
    .tracing()
    .with_trace_config(
      opentelemetry_sdk::trace::Config::default()
        .with_sampler(Sampler::AlwaysOn)
        .with_resource(resource(service_name.clone())),
    )
    .with_batch_config(BatchConfig::default())
    .with_exporter(
      opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(endpoint)
        .with_timeout(Duration::from_secs(3)),
    )
    .install_batch(runtime::Tokio)
    .unwrap();
  global::set_tracer_provider(provider.clone());
  provider.tracer(service_name)
}
