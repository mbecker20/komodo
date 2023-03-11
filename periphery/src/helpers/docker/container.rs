use std::{collections::HashMap, path::PathBuf};

use anyhow::{anyhow, Context};
use helpers::to_monitor_name;
use run_command::async_run_command;
use types::{
    Conversion, Deployment, DockerContainerStats, DockerRunArgs, EnvironmentVar, Log, RestartMode,
};

use crate::helpers::run_monitor_command;

use super::docker_login;

pub async fn container_log(container_name: &str, tail: Option<u64>) -> Log {
    let command = format!(
        "docker logs {container_name} --tail {}",
        tail.unwrap_or(1000)
    );
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

pub async fn pull_image(image: &str) -> Log {
    let command = format!("docker pull {image}");
    run_monitor_command("docker pull", command).await
}

pub async fn deploy(
    deployment: &Deployment,
    docker_token: &Option<String>,
    repo_dir: PathBuf,
    secrets: &HashMap<String, String>,
) -> Log {
    if let Err(e) = docker_login(&deployment.docker_run_args.docker_account, docker_token).await {
        return Log::error("docker login", format!("{e:#?}"));
    }
    let _ = pull_image(&deployment.docker_run_args.image).await;
    let _ = stop_and_remove_container(&to_monitor_name(&deployment.name)).await;
    let command = docker_run_command(deployment, repo_dir);
    if deployment.skip_secret_interp {
        run_monitor_command("docker run", command).await
    } else {
        let command =
            svi::interpolate_variables(&command, secrets, svi::Interpolator::DoubleBrackets)
                .context("failed to interpolate secrets into docker run command");
        if let Err(e) = command {
            return Log::error("docker run", format!("{e:?}"));
        }
        let (command, replacers) = command.unwrap();
        let mut log = run_monitor_command("docker run", command).await;
        log.command = svi::replace_in_string(&log.command, &replacers);
        log.stdout = svi::replace_in_string(&log.stdout, &replacers);
        log.stderr = svi::replace_in_string(&log.stderr, &replacers);
        log
    }
}

pub fn docker_run_command(
    Deployment {
        name,
        repo_mount,
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
                extra_args,
                ..
            },
        ..
    }: &Deployment,
    mut repo_dir: PathBuf,
) -> String {
    let name = to_monitor_name(name);
    let container_user = parse_container_user(container_user);
    let ports = parse_conversions(ports, "-p");
    let mut volumes = volumes.to_owned();
    if let Some(repo_mount) = repo_mount {
        repo_dir.push(&name);
        repo_dir.push(&repo_mount.local);
        let repo_mount = Conversion {
            local: repo_dir.display().to_string(),
            container: repo_mount.container.clone(),
        };
        volumes.push(repo_mount);
    }
    let volumes = parse_conversions(&volumes, "-v");
    let network = parse_network(network);
    let restart = parse_restart(restart);
    let environment = parse_environment(environment);
    let post_image = parse_post_image(post_image);
    let extra_args = parse_extra_args(extra_args);
    format!("docker run -d --name {name}{container_user}{ports}{volumes}{network}{restart}{environment}{extra_args} {image}{post_image}")
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

fn parse_network(network: &str) -> String {
    format!(" --network {network}")
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

fn parse_extra_args(extra_args: &Vec<String>) -> String {
    let args = extra_args.join(" ");
    if args.len() > 0 {
        format!(" {args}")
    } else {
        args
    }
}
