use std::path::PathBuf;

use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use monitor_types::{
  entities::{update::Log, CloneArgs, SystemCommand},
  monitor_timestamp, to_monitor_name,
};
use run_command::async_run_command;

use super::run_monitor_command;

pub async fn pull(
  mut path: PathBuf,
  branch: &Option<String>,
  on_pull: &Option<SystemCommand>,
) -> Vec<Log> {
  let branch = match branch {
    Some(branch) => branch.to_owned(),
    None => "main".to_string(),
  };
  let command =
    format!("cd {} && git pull origin {branch}", path.display());
  let mut logs = Vec::new();
  let pull_log = run_monitor_command("git pull", command).await;
  if !pull_log.success {
    logs.push(pull_log);
    return logs;
  }
  logs.push(pull_log);
  if let Some(on_pull) = on_pull {
    if !on_pull.path.is_empty() && !on_pull.command.is_empty() {
      path.push(&on_pull.path);
      let path = path.display().to_string();
      let on_pull_log = run_monitor_command(
        "on pull",
        format!("cd {path} && {}", on_pull.command),
      )
      .await;
      logs.push(on_pull_log);
    }
  }
  logs
}

pub async fn clone(
  clone_args: impl Into<CloneArgs>,
  mut repo_dir: PathBuf,
  access_token: Option<String>,
) -> anyhow::Result<Vec<Log>> {
  let CloneArgs {
    name,
    repo,
    branch,
    on_clone,
    on_pull,
    ..
  } = clone_args.into();
  let repo =
    repo.as_ref().ok_or(anyhow!("build has no repo attached"))?;
  let name = to_monitor_name(&name);
  repo_dir.push(name);
  let destination = repo_dir.display().to_string();
  let clone_log =
    clone_inner(repo, &destination, &branch, access_token).await;
  if !clone_log.success {
    return Ok(vec![clone_log]);
  }
  let commit_hash_log = get_commit_hash_log(&destination).await?;
  let mut logs = vec![clone_log, commit_hash_log];
  if let Some(command) = on_clone {
    if !command.path.is_empty() && !command.command.is_empty() {
      let on_clone_path = repo_dir.join(&command.path);
      let on_clone_log = run_monitor_command(
        "on clone",
        format!(
          "cd {} && {}",
          on_clone_path.display(),
          command.command
        ),
      )
      .await;
      logs.push(on_clone_log);
    }
  }
  if let Some(command) = on_pull {
    if !command.path.is_empty() && !command.command.is_empty() {
      let on_pull_path = repo_dir.join(&command.path);
      let on_pull_log = run_monitor_command(
        "on pull",
        format!(
          "cd {} && {}",
          on_pull_path.display(),
          command.command
        ),
      )
      .await;
      logs.push(on_pull_log);
    }
  }
  Ok(logs)
}

async fn clone_inner(
  repo: &str,
  destination: &str,
  branch: &Option<String>,
  access_token: Option<String>,
) -> Log {
  let _ = std::fs::remove_dir_all(destination);
  let access_token_at = match &access_token {
    Some(token) => format!("{token}@"),
    None => String::new(),
  };
  let branch = match branch {
    Some(branch) => format!(" -b {branch}"),
    None => String::new(),
  };
  let repo_url =
    format!("https://{access_token_at}github.com/{repo}.git");
  let command = format!("git clone {repo_url} {destination}{branch}");
  let start_ts = unix_timestamp_ms() as i64;
  let output = async_run_command(&command).await;
  let success = output.success();
  let (command, stderr) = if !access_token_at.is_empty() {
    let access_token = access_token.unwrap();
    (
      command.replace(&access_token, "<TOKEN>"),
      output.stderr.replace(&access_token, "<TOKEN>"),
    )
  } else {
    (command, output.stderr)
  };
  Log {
    stage: "clone repo".to_string(),
    command,
    success,
    stdout: output.stdout,
    stderr,
    start_ts,
    end_ts: unix_timestamp_ms() as i64,
  }
}

async fn get_commit_hash_log(repo_dir: &str) -> anyhow::Result<Log> {
  let start_ts = monitor_timestamp();
  let command = format!("cd {repo_dir} && git rev-parse --short HEAD && git rev-parse HEAD && git log -1 --pretty=%B");
  let output = async_run_command(&command).await;
  let mut split = output.stdout.split('\n');
  let (short, _, msg) = (
    split.next().context("failed to get short commit hash")?,
    split.next().context("failed to get long commit hash")?,
    split.next().context("failed to get commit message")?,
  );
  let log = Log {
    stage: "latest commit".into(),
    command,
    stdout: format!("hash: {short}\nmessage: {msg}"),
    stderr: String::new(),
    success: true,
    start_ts,
    end_ts: monitor_timestamp(),
  };
  Ok(log)
}
