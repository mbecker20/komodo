#![allow(unused)]

use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use monitor_client::{
    types::{
        BuildBuilder, Command, Conversion, Deployment, DeploymentBuilder, DockerBuildArgs,
        DockerBuildArgsBuilder, DockerRunArgsBuilder, EnvironmentVar,
    },
    MonitorClient,
};

mod config;
mod tests;

use tests::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let monitor = config::load().await;

    println!("\nstarting tests\n");

    let start_ts = unix_timestamp_ms();

    let server = monitor
        .list_servers(None)
        .await?
        .pop()
        .ok_or(anyhow!("no servers"))?;

    let build = BuildBuilder::default()
        .name("monitor_core".into())
        .server_id(server.server.id.clone().into())
        .repo("mbecker20/monitor".to_string().into())
        .branch("main".to_string().into())
        .docker_build_args(
            DockerBuildArgs {
                build_path: ".".into(),
                dockerfile_path: "core/Dockerfile".to_string().into(),
                ..Default::default()
            }
            .into(),
        )
        .pre_build(
            Command {
                path: "frontend".into(),
                command: "yarn && yarn build".into(),
            }
            .into(),
        )
        .build()?;

    let build = monitor.create_full_build(&build).await?;

    println!("{build:#?}");

    let build_update = monitor.build(&build.id).await?;

    println!("{build_update:#?}");

    let deployment = DeploymentBuilder::default()
        .name("monitor_core_1".into())
        .server_id(server.server.id.clone())
        .build_id(build.id.clone().into())
        .docker_run_args(
            DockerRunArgsBuilder::default()
                .volumes(vec![Conversion {
                    local: "/home/max/.monitor/core.config.toml".into(),
                    container: "/config/config.toml".into(),
                }])
                .build()?,
        )
        .build()?;

    let deployment = monitor.create_full_deployment(&deployment).await?;

    println!("{deployment:#?}");

    let deploy_update = monitor.deploy_container(&deployment.id).await?;

    println!("{deploy_update:#?}");

    // let (server, deployment, build) = create_test_setup(&monitor, "test").await?;

    // let server_stats = get_server_stats(&monitor).await?;
    // println!("server stats:\n{server_stats:#?}\n");

    // subscribe_to_server_stats(&monitor).await?;

    // let (update, container) = deploy_mongo(&monitor).await?;
    // println!(
    //     "mongo deploy update:\n{update:#?}\n\ncontainer: {:#?}\n",
    //     container.container
    // );

    // let update = test_build(&monitor).await?;
    // println!("build update:\n{update:#?}");

    // test_updates(&monitor).await.unwrap();

    let update = test_aws_build(&monitor).await?;

    let end_ts = unix_timestamp_ms();
    let finished_in = (end_ts - start_ts) as f64 / 1000.0;
    println!("\nfinished in {finished_in} s");
    Ok(())
}
