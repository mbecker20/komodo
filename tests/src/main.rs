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

    // let server_stats = get_server_stats(&monitor).await?;
    // println!("server stats:\n{server_stats:#?}\n");

    // subscribe_to_server_stats(&monitor).await?;

    // let (update, container) = deploy_mongo(&monitor).await?;
    // println!(
    //     "mongo deploy update:\n{update:#?}\n\ncontainer: {:#?}\n",
    //     container.container
    // );

    let update = test_build(&monitor).await?;
    println!("build update:\n{update:#?}");

    // test_updates(&monitor).await.unwrap();

    let end_ts = unix_timestamp_ms();
    let finished_in = (end_ts - start_ts) as f64 / 1000.0;
    println!("\nfinished in {finished_in} s");
    Ok(())
}
