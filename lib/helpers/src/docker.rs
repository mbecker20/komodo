use std::sync::Arc;

use ::run_command::async_run_command;
use anyhow::anyhow;
use async_timing_util::unix_timestamp_ms;
use axum::Extension;
use bollard::{container::ListContainersOptions, Docker};
use types::{
    BasicContainerInfo, Build, Conversion, Deployment, DockerRunArgs, EnvironmentVar, Log,
    RestartMode,
};

use crate::output_into_log;

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

pub async fn docker_start(container_name: &str) -> Log {
    let container_name = parse_container_name(container_name);
    let command = format!("docker start {container_name}");
    let start_ts = unix_timestamp_ms() as i64;
    let output = async_run_command(&command).await;
    output_into_log("docker start", command, start_ts, output)
}

pub async fn docker_stop(container_name: &str) -> Log {
    let container_name = parse_container_name(container_name);
    let command = format!("docker stop {container_name}");
    let start_ts = unix_timestamp_ms() as i64;
    let output = async_run_command(&command).await;
    output_into_log("docker stop", command, start_ts, output)
}

pub async fn docker_stop_and_remove(container_name: &str) -> Log {
    let container_name = parse_container_name(container_name);
    let command = format!("docker stop {container_name} && docker container rm {container_name}");
    let start_ts = unix_timestamp_ms() as i64;
    let output = async_run_command(&command).await;
    output_into_log("docker stop and remove", command, start_ts, output)
}

pub async fn deploy(deployment: &Deployment) -> Log {
    let _ = docker_stop_and_remove(&parse_container_name(&deployment.name)).await;
    let command = docker_run_command(deployment);
    let start_ts = unix_timestamp_ms() as i64;
    let output = async_run_command(&command).await;
    output_into_log("docker run", command, start_ts, output)
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

pub async fn docker_build(_build: &Build) -> Log {
    todo!()
}
