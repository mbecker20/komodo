use anyhow::{anyhow, Context};
use run_command::async_run_command;
use types::{
    Conversion, Deployment, DockerContainerStats, DockerRunArgs, EnvironmentVar, Log, RestartMode,
};

use crate::{run_monitor_command, to_monitor_name};

use super::docker_login;

pub async fn container_log(container_name: &str, tail: Option<u64>) -> Log {
    let tail = match tail {
        Some(tail) => format!(" --tail {tail}"),
        None => String::new(),
    };
    let command = format!("docker logs {container_name}{tail}");
    run_monitor_command("get container log", command).await
}

pub async fn container_stats(
    container_name: Option<String>,
) -> anyhow::Result<Vec<DockerContainerStats>> {
    let format = "--format \"{{ json . }}\"";
    let container_name = match container_name {
        Some(name) => format!(" {name}"),
        None => "".to_string(),
    };
    let command = format!("docker stats{container_name} --no-stream {format}");
    let output = async_run_command(&command).await;
    if output.success() {
        let res = output
            .stdout
            .split("\n")
            .filter(|e| e.len() > 0)
            .map(|e| {
                let parsed =
                    serde_json::from_str(e).context(format!("failed at parsing entry {e}"))?;
                Ok(parsed)
            })
            .collect::<anyhow::Result<Vec<DockerContainerStats>>>()?;
        Ok(res)
    } else {
        Err(anyhow!("{}", output.stderr.replace("\n", "")))
    }
}

pub async fn prune_containers() -> Log {
    let command = format!("docker container prune -f");
    run_monitor_command("prune containers", command).await
}

pub async fn start_container(container_name: &str) -> Log {
    let container_name = to_monitor_name(container_name);
    let command = format!("docker start {container_name}");
    run_monitor_command("docker start", command).await
}

pub async fn stop_container(container_name: &str) -> Log {
    let container_name = to_monitor_name(container_name);
    let command = format!("docker stop {container_name}");
    run_monitor_command("docker stop", command).await
}

pub async fn stop_and_remove_container(container_name: &str) -> Log {
    let container_name = to_monitor_name(container_name);
    let command = format!("docker stop {container_name} && docker container rm {container_name}");
    run_monitor_command("docker stop and remove", command).await
}

pub async fn deploy(deployment: &Deployment, docker_token: &Option<String>) -> Log {
    if let Err(e) = docker_login(&deployment.docker_run_args.docker_account, docker_token).await {
        return Log::error("docker login", format!("{e:#?}"));
    }
    let _ = stop_and_remove_container(&to_monitor_name(&deployment.name)).await;
    let command = docker_run_command(deployment);
    run_monitor_command("docker run", command).await
}

pub fn docker_run_command(
    Deployment {
        name,
        docker_run_args:
            DockerRunArgs {
                image,
                volumes,
                ports,
                network,
                container_user,
                post_image,
                restart,
                environment,
                ..
            },
        ..
    }: &Deployment,
) -> String {
    let name = to_monitor_name(name);
    let container_user = parse_container_user(container_user);
    let ports = parse_conversions(ports, "-p");
    let volumes = parse_conversions(volumes, "-v");
    let network = parse_network(network);
    let restart = parse_restart(restart);
    let environment = parse_environment(environment);
    let post_image = parse_post_image(post_image);
    format!("docker run -d --name {name}{container_user}{ports}{volumes}{network}{restart}{environment} {image}{post_image}")
}

fn parse_container_user(container_user: &Option<String>) -> String {
    if let Some(container_user) = container_user {
        format!(" -u {container_user}")
    } else {
        String::new()
    }
}

fn parse_conversions(conversions: &Vec<Conversion>, flag: &str) -> String {
    conversions
        .iter()
        .map(|p| format!(" {flag} {}:{}", p.local, p.container))
        .collect::<Vec<String>>()
        .join("")
}

fn parse_environment(environment: &Vec<EnvironmentVar>) -> String {
    environment
        .iter()
        .map(|p| format!(" --env {}={}", p.variable, p.value))
        .collect::<Vec<String>>()
        .join("")
}

fn parse_network(network: &Option<String>) -> String {
    if let Some(network) = network {
        format!(" --network {network}")
    } else {
        String::new()
    }
}

fn parse_restart(restart: &RestartMode) -> String {
    let restart = match restart {
        RestartMode::OnFailure => "on-failure:10".to_string(),
        _ => restart.to_string(),
    };
    format!(" --restart {restart}")
}

fn parse_post_image(post_image: &Option<String>) -> String {
    if let Some(post_image) = post_image {
        format!(" {post_image}")
    } else {
        String::new()
    }
}
