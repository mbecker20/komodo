use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use monitor_types::{
    entities::{
        build::{Build, BuildConfig},
        update::Log,
        EnvironmentVar, Version,
    },
    optional_string, to_monitor_name,
};

use crate::helpers::run_monitor_command;

use super::{docker_login, parse_extra_args};

pub async fn prune_images() -> Log {
    let command = String::from("docker image prune -a -f");
    run_monitor_command("prune images", command).await
}

pub async fn build(
    Build {
        name,
        config:
            BuildConfig {
                version,
                docker_account,
                docker_organization,
                skip_secret_interp,
                build_path,
                dockerfile_path,
                build_args,
                extra_args,
                use_buildx,
                ..
            },
        ..
    }: &Build,
    mut repo_dir: PathBuf,
    docker_token: Option<String>,
    secrets: &HashMap<String, String>,
) -> anyhow::Result<Vec<Log>> {
    let mut logs = Vec::new();
    let name = to_monitor_name(name);
    let using_account =
        docker_login(&optional_string(docker_account), &docker_token)
            .await
            .context("failed to login to docker")?;
    repo_dir.push(&name);
    let build_dir = repo_dir.join(build_path);
    let dockerfile_path = match optional_string(dockerfile_path) {
        Some(dockerfile_path) => dockerfile_path.to_owned(),
        None => "Dockerfile".to_owned(),
    };
    let build_args = parse_build_args(build_args);
    let extra_args = parse_extra_args(extra_args);
    let buildx = if *use_buildx { " buildx" } else { "" };
    let image_name = get_image_name(
        &name,
        &optional_string(docker_account),
        &optional_string(docker_organization),
    );
    let image_tags = image_tags(&image_name, version);
    let docker_push = if using_account {
        format!(" && docker image push --all-tags {image_name}")
    } else {
        String::new()
    };
    let command = format!(
        "cd {} && docker{buildx} build{build_args}{extra_args}{image_tags} -f {dockerfile_path} .{docker_push}",
        build_dir.display()
    );
    if *skip_secret_interp {
        let build_log =
            run_monitor_command("docker build", command).await;
        logs.push(build_log);
    } else {
        let (command, replacers) = svi::interpolate_variables(
            &command,
            secrets,
            svi::Interpolator::DoubleBrackets,
        )
        .context(
            "failed to interpolate secrets into docker build command",
        )?;
        let mut build_log =
            run_monitor_command("docker build", command).await;
        build_log.command =
            svi::replace_in_string(&build_log.command, &replacers);
        build_log.stdout =
            svi::replace_in_string(&build_log.stdout, &replacers);
        build_log.stderr =
            svi::replace_in_string(&build_log.stderr, &replacers);
        logs.push(build_log);
    }
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
            Some(docker_account) => {
                format!("{docker_account}/{name}")
            }
            None => name.to_string(),
        },
    }
}

fn get_version_image_name(
    image_name: &str,
    version: &Version,
) -> String {
    format!("{image_name}:{}", version.to_string())
}

fn get_latest_image_name(image_name: &str) -> String {
    format!("{image_name}:latest")
}

fn image_tags(image_name: &str, version: &Version) -> String {
    format!(
        " -t {} -t {}",
        get_version_image_name(image_name, version),
        get_latest_image_name(image_name)
    )
}

fn parse_build_args(build_args: &[EnvironmentVar]) -> String {
    let mut args = build_args
        .iter()
        .map(|p| format!(" --build-arg {}={}", p.variable, p.value))
        .collect::<Vec<String>>()
        .join("");
    if !args.is_empty() {
        args.push(' ');
    }
    args
}
