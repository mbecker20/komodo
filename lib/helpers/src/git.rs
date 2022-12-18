use std::{path::PathBuf, str::FromStr};

use ::run_command::async_run_command;
use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use types::{monitor_timestamp, Build, Command, Deployment, GithubToken, GithubUsername, Log};

use crate::{run_monitor_command, to_monitor_name};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CloneArgs {
    name: String,
    repo: Option<String>,
    branch: Option<String>,
    on_clone: Option<Command>,
    pub github_account: Option<GithubUsername>,
}

impl From<&Deployment> for CloneArgs {
    fn from(d: &Deployment) -> Self {
        CloneArgs {
            name: d.name.clone(),
            repo: d.repo.clone(),
            branch: d.branch.clone(),
            on_clone: d.on_clone.clone(),
            github_account: d.github_account.clone(),
        }
    }
}

impl From<&Build> for CloneArgs {
    fn from(b: &Build) -> Self {
        CloneArgs {
            name: b.name.clone(),
            repo: b.repo.clone(),
            branch: b.branch.clone(),
            on_clone: b.on_clone.clone(),
            github_account: b.github_account.clone(),
        }
    }
}

pub async fn pull(path: &str, branch: &Option<String>) -> Log {
    let branch = match branch {
        Some(branch) => branch.to_owned(),
        None => "main".to_string(),
    };
    let command = format!("cd {path} && git pull origin {branch}");
    run_monitor_command("git pull", command).await
}

pub async fn clone_repo(
    clone_args: impl Into<CloneArgs>,
    repo_dir: &str,
    access_token: Option<GithubToken>,
) -> anyhow::Result<Vec<Log>> {
    let CloneArgs {
        name,
        repo,
        branch,
        on_clone,
        ..
    } = clone_args.into();
    let repo = repo.as_ref().ok_or(anyhow!("build has no repo attached"))?;
    let mut repo_dir = PathBuf::from_str(repo_dir)?;
    let name = to_monitor_name(&name);
    repo_dir.push(name);
    let destination = repo_dir.display().to_string();
    let clone_log = clone(repo, &destination, &branch, access_token).await;
    let mut logs = vec![clone_log];
    if let Some(command) = on_clone {
        repo_dir.push(&command.path);
        let on_clone_log = run_monitor_command(
            "on clone",
            format!("cd {} && {}", repo_dir.display(), command.command),
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
    let start_ts = monitor_timestamp();
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
        end_ts: monitor_timestamp(),
    }
}
