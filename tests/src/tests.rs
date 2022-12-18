use anyhow::{anyhow, Context};
use monitor_client::{
    types::{
        Build, Command, Conversion, Deployment, DeploymentWithContainer, DockerBuildArgs, Server,
        SystemStats, Update,
    },
    MonitorClient,
};

pub async fn create_test_setup(
    monitor: &MonitorClient,
    group_name: &str,
) -> anyhow::Result<(Server, Deployment, Build)> {
    let mut servers = monitor.list_servers(None).await?;
    let server = if servers.is_empty() {
        monitor
            .create_server(
                &format!("{group_name}_server"),
                "http://periphery-full:8000",
            )
            .await
            .context("failed at create server")?
    } else {
        servers.pop().unwrap()
    };
    let mut deployments = monitor.list_deployments(None).await?;
    let deployment = if deployments.is_empty() {
        monitor
            .create_deployment(&format!("{group_name}_deployment"), &server.id)
            .await
            .context("failed at create deployment")?
    } else {
        deployments.pop().unwrap().deployment
    };
    let mut builds = monitor.list_builds(None).await?;
    let build = if builds.is_empty() {
        monitor
            .create_build(&format!("{group_name}_build"), &server.id)
            .await
            .context("failed at create build")?
    } else {
        builds.pop().unwrap()
    };
    Ok((server, deployment, build))
}

pub async fn get_server_stats(monitor: &MonitorClient) -> anyhow::Result<SystemStats> {
    let servers = monitor
        .list_servers(None)
        .await
        .context("failed at list servers")?;
    let server = servers.get(0).ok_or(anyhow!("no servers"))?;
    let stats = monitor
        .get_server_stats(&server.id)
        .await
        .context("failed at get server stats")?;
    Ok(stats)
}

pub async fn deploy_mongo(
    monitor: &MonitorClient,
) -> anyhow::Result<(Update, DeploymentWithContainer)> {
    let servers = monitor
        .list_servers(None)
        .await
        .context("failed at list servers")?;
    let server = servers.get(0).ok_or(anyhow!("no servers"))?;
    let mut deployment = monitor.create_deployment("mongo_test", &server.id).await?;
    println!("created deployment");
    deployment.docker_run_args.image = "mongo".to_string();
    deployment.docker_run_args.ports.push(Conversion {
        local: "27020".to_string(),
        container: "27017".to_string(),
    });
    let deployment = monitor.update_deployment(deployment).await?;
    println!("updated deployment");
    let update = monitor.deploy_container(&deployment.id).await?;
    let container = monitor.get_deployment(&deployment.id).await?;
    Ok((update, container))
}

pub async fn test_build(monitor: &MonitorClient) -> anyhow::Result<Update> {
    let servers = monitor
        .list_servers(None)
        .await
        .context("failed at list servers")?;
    let server = servers.get(0).ok_or(anyhow!("no servers"))?;
    let mut build = monitor.create_build("old_periphery", &server.id).await?;
    println!("created build. updating...");
    build.repo = Some("mbecker20/monitor".to_string());
    // build.branch = Some("");
    build.on_clone = Some(Command {
        path: ".".to_string(),
        command: "yarn".to_string(),
    });
    build.pre_build = Some(Command {
        path: "periphery".to_string(),
        command: "yarn build".to_string(),
    });
    build.docker_build_args = Some(DockerBuildArgs {
        build_path: "periphery".to_string(),
        dockerfile_path: None,
        build_args: Vec::new(),
    });
    let build = monitor.update_build(build).await?;
    println!("updated build.");
    let update = monitor.build(&build.id).await?;
    Ok(update)
}
