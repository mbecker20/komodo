use tracing::level_filters::LevelFilter;

pub fn init(log_level: tracing::Level) {
  tracing_subscriber::fmt()
    .with_max_level(LevelFilter::from(log_level))
    .init()
}
