use anyhow::Context;
use log::LevelFilter;
use simple_logger::SimpleLogger;

pub fn init(log_level: LevelFilter) -> anyhow::Result<()> {
  SimpleLogger::new()
    .with_level(log_level)
    .env()
    .with_colors(true)
    .with_utc_timestamps()
    .init()
    .context("failed to configure logger")
}
