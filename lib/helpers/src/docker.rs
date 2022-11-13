use std::sync::Arc;

use anyhow::{anyhow, Context};
use axum::Extension;
use bollard::{container::ListContainersOptions, Docker};
use run_command::async_run_command;
use types::{
    BasicContainerInfo, Build, Conversion, Deployment, DockerContainerStats, DockerRunArgs,
    EnvironmentVar, Log, RestartMode,
};

use crate::run_monitor_command;

pub type DockerExtension = Extension<Arc<DockerClient>>;

pub struct DockerClient {
    docker: Docker,
}

impl DockerClient {
    pub fn extension() -> DockerExtension {
        let client = DockerClient {
            docker: Docker::connect_with_local_defaults()
                .expect("failed to connect to docker daemon"),
        };
        Extension(Arc::new(client))
    }

    pub async fn list_containers(&self) -> anyhow::Result<Vec<BasicContainerInfo>> {
        let res = self
            .docker
            .list_containers(Some(ListContainersOptions::<String> {
                all: true,
                ..Default::default()
            }))
            .await?
            .into_iter()
            .map(|s| {
                let info = BasicContainerInfo {
                    id: s.id.unwrap_or_default(),
                    name: s
                        .names
                        .ok_or(anyhow!("no names on container"))?
                        .pop()
                        .ok_or(anyhow!("no names on container (empty vec)"))?
                        .replace("/", ""),
                    state: s.state.unwrap().parse().unwrap(),
                    status: s.status,
                };
                Ok::<_, anyhow::Error>(info)
            })
            .collect::<anyhow::Result<Vec<BasicContainerInfo>>>()?;
        Ok(res)
    }
}

// CONTAINER COMMANDS

pub fn parse_container_name(name: &str) -> String {
    name.to_lowercase().replace(" ", "_")
}

pub async fn container_log(container_name: &str, tail: Option<u64>) -> Log {
    let tail = match tail {
        Some(tail) => format!(" --tail {tail}"),
        None => String::new(),
    };
    let command = format!("docker logs {container_name}{tail}");
    run_monitor_command("get container log", command).await
}

pub async fn container_stats() -> anyhow::Result<Vec<DockerContainerStats>> {
    let command = "docker stats --no-stream --format \"{{json .}}\"";
    let output = async_run_command(command).await;
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
        Err(anyhow!("failed to get docker logs"))
    }
}

pub async fn prune_containers() -> Log {
    let command = format!("docker container prune -f");
    run_monitor_command("prune containers", command).await
}

pub async fn start_container(container_name: &str) -> Log {
    let container_name = parse_container_name(container_name);
    let command = format!("docker start {container_name}");
    run_monitor_command("docker start", command).await
}

pub async fn stop_container(container_name: &str) -> Log {
    let container_name = parse_container_name(container_name);
    let command = format!("docker stop {container_name}");
    run_monitor_command("docker stop", command).await
}

pub async fn stop_and_remove_container(container_name: &str) -> Log {
    let container_name = parse_container_name(container_name);
    let command = format!("docker stop {container_name} && docker container rm {container_name}");
    run_monitor_command("docker stop and remove", command).await
}

pub async fn deploy(deployment: &Deployment) -> Log {
    let _ = stop_and_remove_container(&parse_container_name(&deployment.name)).await;
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
    let name = parse_container_name(name);
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
        RestartMode::OnFailure => format!("on-failure:10"),
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

// BUILD COMMANDS

pub async fn build(_build: &Build) -> Log {
    todo!()
}

pub async fn prune_images() -> Log {
    let command = format!("docker image prune -a -f");
    run_monitor_command("prune images", command).await
}

// NETWORKS

pub async fn create_network(name: &str, driver: Option<&str>) -> Log {
    let driver = match driver {
        Some(driver) => format!(" -d {driver}"),
        None => String::new(),
    };
    let command = format!("docker network create{driver} {name}");
    run_monitor_command("create network", command).await
}

pub async fn delete_network(name: &str) -> Log {
    let command = format!("docker network rm {name}");
    run_monitor_command("delete network", command).await
}

pub async fn prune_networks() -> Log {
    let command = format!("docker network prune -f");
    run_monitor_command("prune networks", command).await
}
