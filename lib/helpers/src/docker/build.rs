use std::{path::PathBuf, str::FromStr};

use anyhow::{anyhow, Context};
use types::{Build, DockerBuildArgs, Log, Version};

use crate::{git, run_monitor_command, to_monitor_name};

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
        branch,
        docker_account,
        pre_build,
        ..
    }: &Build,
    repo_dir: &str,
    docker_token: Option<String>,
) -> anyhow::Result<Vec<Log>> {
    let mut logs = Vec::new();
    let DockerBuildArgs {
        build_path,
        dockerfile_path,
    } = docker_build_args
        .as_ref()
        .ok_or(anyhow!("build missing docker build args"))?;
    let name = to_monitor_name(name);
    let using_account = docker_login(docker_account, &docker_token)
        .await
        .context("failed to login to docker")?;
    let repo_dir = PathBuf::from_str(repo_dir)
        .context(format!("invalid repo dir: {repo_dir}"))?
        .join(&name);
    let pull_log = git::pull(
        &repo_dir
            .to_str()
            .context(format!("invalid repo dir: {}", repo_dir.display()))?,
        branch,
    )
    .await;
    logs.push(pull_log);
    if let Some(command) = pre_build {
        let mut repo_dir = repo_dir.clone();
        repo_dir.push(&command.path);
        let pre_build_log = run_monitor_command(
            "pre build",
            format!("cd {} && {}", repo_dir.display(), command.command),
        )
        .await;
        logs.push(pre_build_log);
    }
    let build_dir = repo_dir.join(build_path);
    let cd = build_dir.display();
    let dockerfile_path = match dockerfile_path {
        Some(dockerfile_path) => dockerfile_path.to_owned(),
        None => "Dockerfile".to_owned(),
    };
    let image_name = get_image_name(docker_account, &name);
    let image_tags = image_tags(&image_name, &version);
    let docker_push = if using_account {
        format!(" && docker image push --all-tags {image_name}")
    } else {
        String::new()
    };
    let command =
        format!("cd {cd} && docker build {image_tags} -f {dockerfile_path} .{docker_push}");
    let build_log = run_monitor_command("docker build", command).await;
    logs.push(build_log);
    Ok(logs)
}

fn get_image_name(docker_account: &Option<String>, name: &str) -> String {
    match docker_account {
        Some(docker_account) => format!("{docker_account}/{name}"),
        None => name.to_string(),
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
