use std::path::PathBuf;

use anyhow::{anyhow, Context};
use helpers::to_monitor_name;
use types::{Build, DockerBuildArgs, EnvironmentVar, Log, Version};

use crate::helpers::run_monitor_command;

use super::docker_login;

pub async fn prune_images() -> Log {
    let command = format!("docker image prune -a -f");
    run_monitor_command("prune images", command).await
}

pub async fn build(
    Build {
        name,
        version,
        docker_build_args,
        docker_account,
        docker_organization,
        ..
    }: &Build,
    mut repo_dir: PathBuf,
    docker_token: Option<String>,
) -> anyhow::Result<Vec<Log>> {
    let mut logs = Vec::new();
    let DockerBuildArgs {
        build_path,
        dockerfile_path,
        build_args,
    } = docker_build_args
        .as_ref()
        .ok_or(anyhow!("build missing docker build args"))?;
    let name = to_monitor_name(name);
    let using_account = docker_login(docker_account, &docker_token)
        .await
        .context("failed to login to docker")?;
    repo_dir.push(&name);
    let build_dir = repo_dir.join(build_path);
    let dockerfile_path = match dockerfile_path {
        Some(dockerfile_path) => dockerfile_path.to_owned(),
        None => "Dockerfile".to_owned(),
    };
    let build_args = parse_build_args(build_args);
    let image_name = get_image_name(&name, docker_account, docker_organization);
    let image_tags = image_tags(&image_name, &version);
    let docker_push = if using_account {
        format!(" && docker image push --all-tags {image_name}")
    } else {
        String::new()
    };
    let command = format!(
        "cd {} && docker build {build_args}{image_tags} -f {dockerfile_path} .{docker_push}",
        build_dir.display()
    );
    let build_log = run_monitor_command("docker build", command).await;
    logs.push(build_log);
    Ok(logs)
}

fn get_image_name(
    name: &str,
    docker_account: &Option<String>,
    docker_organization: &Option<String>,
) -> String {
    match docker_organization {
        Some(docker_org) => format!("{docker_org}/{name}"),
        None => match docker_account {
            Some(docker_account) => format!("{docker_account}/{name}"),
            None => name.to_string(),
        },
    }
}

fn get_version_image_name(image_name: &str, version: &Version) -> String {
    format!("{image_name}:{}", version.to_string())
}

fn get_latest_image_name(image_name: &str) -> String {
    format!("{image_name}:latest")
}

fn image_tags(image_name: &str, version: &Version) -> String {
    format!(
        "-t {} -t {}",
        get_version_image_name(image_name, version),
        get_latest_image_name(image_name)
    )
}

fn parse_build_args(build_args: &Vec<EnvironmentVar>) -> String {
    let mut args = build_args
        .iter()
        .map(|p| format!(" --build-arg {}={}", p.variable, p.value))
        .collect::<Vec<String>>()
        .join("");
    if args.len() > 0 {
        args.push(' ');
    }
    args
}
