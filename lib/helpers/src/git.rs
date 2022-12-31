use std::path::PathBuf;

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
    on_pull: Option<Command>,
    pub github_account: Option<GithubUsername>,
}

impl From<&Deployment> for CloneArgs {
    fn from(d: &Deployment) -> Self {
        CloneArgs {
            name: d.name.clone(),
            repo: d.repo.clone(),
            branch: d.branch.clone(),
            on_clone: d.on_clone.clone(),
            on_pull: d.on_pull.clone(),
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
            on_pull: None,
            github_account: b.github_account.clone(),
        }
    }
}

pub async fn pull(
    mut path: PathBuf,
    branch: &Option<String>,
    on_pull: &Option<Command>,
) -> Vec<Log> {
    let branch = match branch {
        Some(branch) => branch.to_owned(),
        None => "main".to_string(),
    };
    let command = format!("cd {} && git pull origin {branch}", path.display());
    let mut logs = Vec::new();
    let pull_log = run_monitor_command("git pull", command).await;
    if !pull_log.success {
        logs.push(pull_log);
        return logs;
    }
    logs.push(pull_log);
    if let Some(on_pull) = on_pull {
        path.push(&on_pull.path);
        let path = path.display().to_string();
        let on_pull_log = run_monitor_command(
            "on pull command",
            format!("cd {path} && {}", on_pull.command),
        )
        .await;
        logs.push(on_pull_log);
    }
    logs
}

pub async fn clone_repo(
    clone_args: impl Into<CloneArgs>,
    mut repo_dir: PathBuf,
    access_token: Option<GithubToken>,
) -> anyhow::Result<Vec<Log>> {
    let CloneArgs {
        name,
        repo,
        branch,
        on_clone,
        on_pull,
        ..
    } = clone_args.into();
    let repo = repo.as_ref().ok_or(anyhow!("build has no repo attached"))?;
    let name = to_monitor_name(&name);
    repo_dir.push(name);
    let destination = repo_dir.display().to_string();
    let clone_log = clone(repo, &destination, &branch, access_token).await;
    let mut logs = vec![clone_log];
    if let Some(command) = on_clone {
        let on_clone_path = repo_dir.join(&command.path);
        let on_clone_log = run_monitor_command(
            "on clone",
            format!("cd {} && {}", on_clone_path.display(), command.command),
        )
        .await;
        logs.push(on_clone_log);
    }
    if let Some(command) = on_pull {
        let on_pull_path = repo_dir.join(&command.path);
        let on_pull_log = run_monitor_command(
            "on pull",
            format!("cd {} && {}", on_pull_path.display(), command.command),
        )
        .await;
        logs.push(on_pull_log);
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
