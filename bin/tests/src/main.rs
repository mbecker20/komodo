#[macro_use]
extern crate tracing;

use logger::LogConfig;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

mod core;
// mod periphery;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  logger::init(LogConfig::default())?;
  // periphery::tests().await?;
  core::tests().await?;

  Ok(())
}

fn random_string(length: usize) -> String {
  thread_rng()
    .sample_iter(&Alphanumeric)
    .take(length)
    .map(char::from)
    .collect()
}
