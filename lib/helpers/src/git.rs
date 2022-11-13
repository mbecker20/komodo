use std::{path::PathBuf, str::FromStr};

use ::run_command::async_run_command;
use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use types::{Build, Deployment, GithubToken, Log};

use crate::run_monitor_command;

pub async fn clone_build_repo(
    Build {
        repo,
        branch,
        on_clone,
        ..
    }: &Build,
    destination: &str,
    access_token: Option<GithubToken>,
) -> anyhow::Result<Vec<Log>> {
    let repo = repo.as_ref().ok_or(anyhow!("build has no repo attached"))?;
    let clone_log = clone(repo, destination, branch, access_token).await;
    let mut logs = vec![clone_log];
    if let Some(command) = on_clone {
        let mut path = PathBuf::from_str(destination)
            .context("failed to parse destination path to pathbuf")?;
        path.push(&command.path);
        let on_clone_log = run_monitor_command(
            "on clone",
            format!("cd {} && {}", path.display(), command.command),
        )
        .await;
        logs.push(on_clone_log);
    }
    Ok(logs)
}

pub async fn clone_deployment_repo(
    Deployment {
        repo,
        branch,
        on_clone,
        .. 
    }: &Deployment,
    destination: &str,
    access_token: Option<GithubToken>,
) -> anyhow::Result<Vec<Log>> {
    let repo = repo.as_ref().ok_or(anyhow!("build has no repo attached"))?;
    let clone_log = clone(repo, destination, branch, access_token).await;
    let mut logs = vec![clone_log];
    if let Some(command) = on_clone {
        let mut path = PathBuf::from_str(destination)
            .context("failed to parse destination path to pathbuf")?;
        path.push(&command.path);
        let on_clone_log = run_monitor_command(
            "on clone",
            format!("cd {} && {}", path.display(), command.command),
        )
        .await;
        logs.push(on_clone_log);
    }
    Ok(logs)
}

async fn clone(
    repo: &str,
    destination: &str,
    branch: &Option<String>,
    access_token: Option<GithubToken>,
) -> Log {
    let _ = std::fs::remove_dir_all(destination);
    let access_token = match access_token {
        Some(token) => format!("{token}@"),
        None => String::new(),
    };
    let branch = match branch {
        Some(branch) => format!(" -b {branch}"),
        None => String::new(),
    };
    let repo_url = format!("https://{access_token}github.com/{repo}.git");
    let command = format!("git clone {repo_url} {destination}{branch}");
    let start_ts = unix_timestamp_ms() as i64;
    let output = async_run_command(&command).await;
    let command = if access_token.len() > 0 {
        command.replace(&access_token, "<TOKEN>")
    } else {
        command
    };
    Log {
        stage: "clone repo".to_string(),
        command,
        success: output.success(),
        stdout: output.stdout,
        stderr: output.stderr,
        start_ts,
        end_ts: unix_timestamp_ms() as i64,
    }
}
