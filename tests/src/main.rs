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

    // let stats = get_server_stats(&monitor).await?;
    // println!("{stats:#?}");

    let (server, deployment, build) = create_test_setup(&monitor, "test").await?;

    let server_stats = get_server_stats(&monitor).await?;
    println!("{server_stats:#?}\n");

    let (update, container) = deploy_mongo(&monitor).await?;
    println!("{update:#?}\n{container:#?}\n");

    let update = test_build(&monitor).await?;
    println!("{update:#?}");

    let end_ts = unix_timestamp_ms();
    let finished_in = (end_ts - start_ts) as f64 / 1000.0;
    println!("\nfinished in {finished_in} s");
    Ok(())
}
