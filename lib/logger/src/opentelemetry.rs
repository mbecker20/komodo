use std::time::Duration;

use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{
  trace::{self, RandomIdGenerator, Sampler, Tracer},
  Resource,
};
use tracing_opentelemetry::OpenTelemetryLayer;

pub fn layer<S>(
  endpoint: &str,
  service_name: String,
) -> OpenTelemetryLayer<S, Tracer>
where
  S: tracing::Subscriber,
  for<'span> S: tracing_subscriber::registry::LookupSpan<'span>,
{
  let tracer = opentelemetry_otlp::new_pipeline()
    .tracing()
    .with_exporter(
      opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(endpoint)
        .with_timeout(Duration::from_secs(3)),
    )
    .with_trace_config(
      trace::config()
        .with_sampler(Sampler::AlwaysOn)
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(Resource::new(vec![KeyValue::new(
          "service.name",
          service_name,
        )])),
    )
    .install_batch(opentelemetry_sdk::runtime::Tokio)
    .expect("failed to init opentelemetry tracer");
  tracing_opentelemetry::layer().with_tracer(tracer)
}
