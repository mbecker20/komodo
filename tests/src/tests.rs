use anyhow::{anyhow, Context};
use monitor_client::{
    types::{Build, Deployment, Server, SystemStats},
    MonitorClient,
};

pub async fn create_test_setup(
    monitor: &MonitorClient,
    group_name: &str,
) -> anyhow::Result<(Server, Deployment, Build)> {
    let server = monitor
        .create_server(&format!("{group_name}_server"), "http://localhost:9001")
        .await
        .context("failed at create server")?;
    let deployment = monitor
        .create_deployment(&format!("{group_name}"), &server.id)
        .await
        .context("failed at create deployment")?;
    let build = monitor
        .create_build(&format!("{group_name}_build"), &server.id)
        .await
        .context("failed at create build")?;
    Ok((server, deployment, build))
}

pub async fn get_server_stats(monitor: &MonitorClient) -> anyhow::Result<SystemStats> {
    let servers = monitor
        .list_servers()
        .await
        .context("failed at list servers")?;
    let server = servers.get(0).ok_or(anyhow!("no servers"))?;
    let stats = monitor
        .get_server_stats(&server.id)
        .await
        .context("failed at get server stats")?;
    Ok(stats)
}
