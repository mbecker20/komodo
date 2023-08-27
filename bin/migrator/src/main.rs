#![allow(unused)]

use crate::config::State;

#[macro_use]
extern crate log;

mod config;
mod legacy;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init(log::LevelFilter::Info)?;

    info!("starting migrator");

    let state = State::load().await?;

    Ok(())
}
