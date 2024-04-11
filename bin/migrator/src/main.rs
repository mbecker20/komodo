#![allow(unused)]
#[macro_use]
extern crate tracing;

use logger::LogConfig;

use crate::state::State;

mod legacy;
mod migrate;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  logger::init(LogConfig {
    stdio: true,
    ..Default::default()
  })?;

  info!("starting migrator");

  let state = State::load().await?;

  state.migrate_all().await?;

  Ok(())
}
