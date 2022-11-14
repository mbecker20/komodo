use std::{path::PathBuf, str::FromStr};

use anyhow::{anyhow, Context};
use run_command::async_run_command;
use types::{Build, DockerBuildArgs, Log};

use crate::{git, run_monitor_command, to_monitor_name};

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
        ..
    }: &Build,
    repo_dir: &str,
    docker_token: Option<String>,
) -> anyhow::Result<Vec<Log>> {
    let DockerBuildArgs {
        build_path,
        dockerfile_path,
    } = docker_build_args
        .as_ref()
        .ok_or(anyhow!("build missing docker build args"))?;
    let name = to_monitor_name(name);
    let docker_account_pw = get_docker_username_pw(docker_account, &docker_token)?;
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
    if let Some((username, password)) = &docker_account_pw {
        let login = format!("docker login -u {username} -p {password}");
        async_run_command(&login).await;
    }
    let build_dir = repo_dir.join(build_path);
    let cd = build_dir.display();
    let dockerfile_path = match dockerfile_path {
        Some(dockerfile_path) => dockerfile_path.to_owned(),
        None => "Dockerfile".to_owned(),
    };
    let image_name = get_image_name(docker_account, &name);
    let docker_push = if docker_account_pw.is_some() {
        format!(" && docker push {image_name}")
    } else {
        String::new()
    };
    let command =
        format!("cd {cd} && docker build -t {image_name} -f {dockerfile_path} .{docker_push}");
    let build_log = run_monitor_command("docker build", command).await;
    Ok(vec![pull_log, build_log])
}

fn get_docker_username_pw(
    docker_account: &Option<String>,
    docker_token: &Option<String>,
) -> anyhow::Result<Option<(String, String)>> {
    match docker_account {
        Some(docker_account) => match docker_token {
            Some(docker_token) => Ok(Some((docker_account.to_owned(), docker_token.to_owned()))),
            None => Err(anyhow!(
                "docker token for account {docker_account} has not been configured on this client"
            )),
        },
        None => Ok(None),
    }
}

fn get_image_name(docker_account: &Option<String>, name: &str) -> String {
    match docker_account {
        Some(docker_account) => format!("{docker_account}/{name}"),
        None => name.to_string(),
    }
}
