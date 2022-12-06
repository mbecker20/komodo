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

    let mut builds = monitor.list_builds().await?;
    let mut build = builds.pop().unwrap();

    println!("{build:#?}");

    // build.name = format!("{}_1", build.name);
    // build.repo = Some("mbecker20/monitor".to_string());
    // build.branch = Some("next".to_string());

    // let build = monitor.update_build(build).await?;

    let update = monitor.reclone_build(&build.id).await?;

    println!("{update:#?}");

    // let mut deployments = monitor.list_deployments().await?;
    // let mut deployment = deployments.pop().unwrap();

    // println!("{deployment:#?}");

    // deployment.name = format!("{}_1", deployment.name);
    // // deployment.docker_run_args.image = "test_mongo".to_string();

    // let deployment = monitor.update_deployment(deployment).await?;

    // println!("{deployment:#?}");

    let end_ts = unix_timestamp_ms();
    let finished_in = (end_ts - start_ts) as f64 / 1000.0;
    println!("\nfinished in {finished_in} s");
    Ok(())
}
