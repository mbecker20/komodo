use std::path::Path;

use anyhow::Context;
use async_timing_util::unix_timestamp_ms;
use monitor_client::entities::{
  monitor_timestamp, to_monitor_name, update::Log, CloneArgs,
  SystemCommand,
};
use run_command::async_run_command;

use crate::config::periphery_config;

use super::{get_github_token, run_monitor_command};

pub async fn pull(
  path: &Path,
  branch: &Option<String>,
  on_pull: &Option<SystemCommand>,
) -> Vec<Log> {
  let branch = match branch {
    Some(branch) => branch.to_owned(),
    None => "main".to_string(),
  };

  let command =
    format!("cd {} && git pull origin {branch}", path.display());

  let pull_log = run_monitor_command("git pull", command).await;

  if !pull_log.success {
    return vec![pull_log];
  }

  let commit_hash_log =
    get_commit_hash_log(path).await.unwrap_or(Log::simple(
      "latest commit",
      String::from("failed to get latest commit"),
    ));

  let mut logs = vec![pull_log, commit_hash_log];

  if let Some(on_pull) = on_pull {
    if !on_pull.path.is_empty() && !on_pull.command.is_empty() {
      let path = path.join(&on_pull.path);
      let on_pull_log = run_monitor_command(
        "on pull",
        format!("cd {} && {}", path.display(), on_pull.command),
      )
      .await;
      logs.push(on_pull_log);
    }
  }

  logs
}

#[instrument]
pub async fn clone<T>(clone_args: T) -> anyhow::Result<Vec<Log>>
where
  T: Into<CloneArgs> + std::fmt::Debug,
{
  let CloneArgs {
    name,
    repo,
    branch,
    on_clone,
    on_pull,
    github_account,
  } = clone_args.into();

  let access_token = get_github_token(&github_account)?;

  let repo = repo.as_ref().context("build has no repo attached")?;
  let name = to_monitor_name(&name);

  let repo_dir = periphery_config().repo_dir.join(name);

  let clone_log =
    clone_inner(repo, &repo_dir, &branch, access_token).await;

  if !clone_log.success {
    warn!("repo at {repo_dir:?} failed to clone");
    return Ok(vec![clone_log]);
  }

  info!("repo at {repo_dir:?} cloned with clone_inner");

  let commit_hash_log = get_commit_hash_log(&repo_dir).await?;

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
      info!(
        "run repo on_clone command | command: {} | cwd: {:?}",
        command.command, on_clone_path
      );
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
      info!(
        "run repo on_pull command | command: {} | cwd: {:?}",
        command.command, on_pull_path
      );
      logs.push(on_pull_log);
    }
  }
  Ok(logs)
}

#[instrument]
async fn clone_inner(
  repo: &str,
  destination: &Path,
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
  let command =
    format!("git clone {repo_url} {}{branch}", destination.display());
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

#[instrument]
async fn get_commit_hash_log(repo_dir: &Path) -> anyhow::Result<Log> {
  let start_ts = monitor_timestamp();
  let command = format!("cd {} && git rev-parse --short HEAD && git rev-parse HEAD && git log -1 --pretty=%B", repo_dir.display());
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
