#![allow(unused)]

use anyhow::Context;
use async_timing_util::unix_timestamp_ms;
use monitor_client::{types::Conversion, MonitorClient};

mod config;
mod tests;

use tests::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let monitor = config::load().await;

    println!("\nstarting tests\n");

    let start_ts = unix_timestamp_ms();

    // let (server, deployment, build) = create_test_setup(&monitor, "test").await?;

    // let update = deploy_mongo(&monitor).await?;
    // println!("{update:#?}");

    let update = test_build(&monitor).await?;
    println!("{update:#?}");

    let end_ts = unix_timestamp_ms();
    let finished_in = (end_ts - start_ts) as f64 / 1000.0;
    println!("\nfinished in {finished_in} s");
    Ok(())
}
