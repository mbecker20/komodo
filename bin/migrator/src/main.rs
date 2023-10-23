#![allow(unused)]

use crate::state::State;

#[macro_use]
extern crate log;

mod legacy;
mod migrate;
mod state;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  logger::init(log::LevelFilter::Info)?;

  info!("starting migrator");

  let state = State::load().await?;

  Ok(())
}
