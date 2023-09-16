use std::collections::HashMap;

use anyhow::{anyhow, Context};
use monitor_types::{
    entities::{
        deployment::{
            Conversion, Deployment, DeploymentConfig,
            DeploymentImage, DockerContainerStats, RestartMode,
            TerminationSignal,
        },
        update::Log,
        EnvironmentVar,
    },
    optional_string, to_monitor_name,
};
use run_command::async_run_command;

use crate::helpers::{docker::parse_extra_args, run_monitor_command};

use super::docker_login;

pub async fn container_log(container_name: &str, tail: u64) -> Log {
    let command =
        format!("docker logs {container_name} --tail {tail}");
    run_monitor_command("get container log", command).await
}

pub async fn container_log_search(
    container_name: &str,
    search: &str,
) -> Log {
    let command =
        format!("docker logs {container_name} | grep {search}");
    run_monitor_command("get container log grep", command).await
}

pub async fn container_stats(
    container_name: Option<String>,
) -> anyhow::Result<Vec<DockerContainerStats>> {
    let format = "--format \"{{ json . }}\"";
    let container_name = match container_name {
        Some(name) => format!(" {name}"),
        None => "".to_string(),
    };
    let command =
        format!("docker stats{container_name} --no-stream {format}");
    let output = async_run_command(&command).await;
    if output.success() {
        let res = output
            .stdout
            .split('\n')
            .filter(|e| !e.is_empty())
            .map(|e| {
                let parsed = serde_json::from_str(e).context(
                    format!("failed at parsing entry {e}"),
                )?;
                Ok(parsed)
            })
            .collect::<anyhow::Result<Vec<DockerContainerStats>>>()?;
        Ok(res)
    } else {
        Err(anyhow!("{}", output.stderr.replace('\n', "")))
    }
}

pub async fn prune_containers() -> Log {
    let command = String::from("docker container prune -f");
    run_monitor_command("prune containers", command).await
}

pub async fn start_container(container_name: &str) -> Log {
    let container_name = to_monitor_name(container_name);
    let command = format!("docker start {container_name}");
    run_monitor_command("docker start", command).await
}

pub async fn stop_container(
    container_name: &str,
    signal: Option<TerminationSignal>,
    time: Option<i32>,
) -> Log {
    let command =
        stop_container_command(container_name, signal, time);
    let log = run_monitor_command("docker stop", command).await;
    if log.stderr.contains("unknown flag: --signal") {
        let command =
            stop_container_command(container_name, None, time);
        let mut log =
            run_monitor_command("docker stop", command).await;
        log.stderr = format!(
            "old docker version: unable to use --signal flag{}",
            if !log.stderr.is_empty() {
                format!("\n\n{}", log.stderr)
            } else {
                String::new()
            }
        );
        log
    } else {
        log
    }
}

pub async fn stop_and_remove_container(
    container_name: &str,
    signal: Option<TerminationSignal>,
    time: Option<i32>,
) -> Log {
    let stop_command =
        stop_container_command(container_name, signal, time);
    let command = format!(
        "{stop_command} && docker container rm {container_name}"
    );
    let log =
        run_monitor_command("docker stop and remove", command).await;
    if log.stderr.contains("unknown flag: --signal") {
        let stop_command =
            stop_container_command(container_name, None, time);
        let command = format!(
            "{stop_command} && docker container rm {container_name}"
        );
        let mut log =
            run_monitor_command("docker stop", command).await;
        log.stderr = format!(
            "old docker version: unable to use --signal flag{}",
            if !log.stderr.is_empty() {
                format!("\n\n{}", log.stderr)
            } else {
                String::new()
            }
        );
        log
    } else {
        log
    }
}

fn stop_container_command(
    container_name: &str,
    signal: Option<TerminationSignal>,
    time: Option<i32>,
) -> String {
    let container_name = to_monitor_name(container_name);
    let signal = signal
        .map(|signal| format!(" --signal {signal}"))
        .unwrap_or_default();
    let time = time
        .map(|time| format!(" --time {time}"))
        .unwrap_or_default();
    format!("docker stop{signal}{time} {container_name}")
}

pub async fn rename_container(
    curr_name: &str,
    new_name: &str,
) -> Log {
    let curr = to_monitor_name(curr_name);
    let new = to_monitor_name(new_name);
    let command = format!("docker rename {curr} {new}");
    run_monitor_command("docker rename", command).await
}

async fn pull_image(image: &str) -> Log {
    let command = format!("docker pull {image}");
    run_monitor_command("docker pull", command).await
}

pub async fn deploy(
    deployment: &Deployment,
    docker_token: &Option<String>,
    secrets: &HashMap<String, String>,
    stop_signal: Option<TerminationSignal>,
    stop_time: Option<i32>,
) -> Log {
    if let Err(e) = docker_login(
        &optional_string(&deployment.config.docker_account),
        docker_token,
    )
    .await
    {
        return Log::error("docker login", format!("{e:#?}"));
    }
    let image = if let DeploymentImage::Image { image } =
        &deployment.config.image
    {
        if image.is_empty() {
            return Log::error(
                "get image",
                String::from(
                    "deployment does not have image attached",
                ),
            );
        }
        image
    } else {
        return Log::error(
            "get image",
            String::from("deployment does not have image attached"),
        );
    };
    let _ = pull_image(image).await;
    let _ = stop_and_remove_container(
        &deployment.name,
        stop_signal,
        stop_time,
    )
    .await;
    let command = docker_run_command(deployment, image);
    if deployment.config.skip_secret_interp {
        run_monitor_command("docker run", command).await
    } else {
        let command = svi::interpolate_variables(
            &command,
            secrets,
            svi::Interpolator::DoubleBrackets,
        )
        .context(
            "failed to interpolate secrets into docker run command",
        );
        if let Err(e) = command {
            return Log::error("docker run", format!("{e:?}"));
        }
        let (command, replacers) = command.unwrap();
        let mut log =
            run_monitor_command("docker run", command).await;
        log.command =
            svi::replace_in_string(&log.command, &replacers);
        log.stdout = svi::replace_in_string(&log.stdout, &replacers);
        log.stderr = svi::replace_in_string(&log.stderr, &replacers);
        log
    }
}

pub fn docker_run_command(
    Deployment {
        name,
        config:
            DeploymentConfig {
                volumes,
                ports,
                network,
                container_user,
                process_args,
                restart,
                environment,
                extra_args,
                ..
            },
        ..
    }: &Deployment,
    image: &str,
) -> String {
    let name = to_monitor_name(name);
    let container_user = parse_container_user(container_user);
    let ports = parse_conversions(ports, "-p");
    let volumes = volumes.to_owned();
    let volumes = parse_conversions(&volumes, "-v");
    let network = parse_network(network);
    let restart = parse_restart(restart);
    let environment = parse_environment(environment);
    let process_args = parse_process_args(process_args);
    let extra_args = parse_extra_args(extra_args);
    format!("docker run -d --name {name}{container_user}{ports}{volumes}{network}{restart}{environment}{extra_args} {image}{process_args}")
}

fn parse_container_user(container_user: &String) -> String {
    if container_user.is_empty() {
        String::new()
    } else {
        format!(" -u {container_user}")
    }
}

fn parse_conversions(
    conversions: &[Conversion],
    flag: &str,
) -> String {
    conversions
        .iter()
        .map(|p| format!(" {flag} {}:{}", p.local, p.container))
        .collect::<Vec<String>>()
        .join("")
}

fn parse_environment(environment: &[EnvironmentVar]) -> String {
    environment
        .iter()
        .map(|p| format!(" --env {}=\"{}\"", p.variable, p.value))
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

fn parse_process_args(process_args: &String) -> String {
    if process_args.is_empty() {
        String::new()
    } else {
        format!(" {process_args}")
    }
}
