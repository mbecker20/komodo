use std::{
  path::{Path, PathBuf},
  str::FromStr,
};

use anyhow::Context;
use command::run_monitor_command;
use formatting::{bold, format_serror, muted};
use monitor_client::entities::{
  all_logs_success, monitor_timestamp, to_monitor_name, update::Log,
  CloneArgs, LatestCommit, SystemCommand,
};
use run_command::async_run_command;
use tracing::instrument;

/// Return (logs, commit hash, commit msg)
#[tracing::instrument(level = "debug")]
pub async fn pull(
  path: &Path,
  branch: &Option<String>,
  commit: &Option<String>,
  on_pull: &Option<SystemCommand>,
) -> (Vec<Log>, Option<String>, Option<String>) {
  let branch = match branch {
    Some(branch) => branch.to_owned(),
    None => "main".to_string(),
  };

  let command =
    format!("cd {} && git pull origin {branch}", path.display());

  let pull_log = run_monitor_command("git pull", command).await;

  let mut logs = vec![pull_log];

  if !logs[0].success {
    return (logs, None, None);
  }

  if let Some(commit) = commit {
    let reset_log = run_monitor_command(
      "set commit",
      format!("cd {} && git reset --hard {commit}", path.display()),
    )
    .await;
    logs.push(reset_log);
  }

  let (hash, message) = match get_commit_hash_log(path).await {
    Ok((log, hash, message)) => {
      logs.push(log);
      (Some(hash), Some(message))
    }
    Err(e) => {
      logs.push(Log::simple(
        "latest commit",
        format_serror(
          &e.context("failed to get latest commit").into(),
        ),
      ));
      (None, None)
    }
  };

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

  (logs, hash, message)
}

/// return (logs, commit hash, commit message)
#[tracing::instrument(level = "debug", skip(access_token))]
pub async fn clone<T>(
  clone_args: T,
  repo_dir: &Path,
  access_token: Option<String>,
) -> anyhow::Result<(Vec<Log>, Option<String>, Option<String>)>
where
  T: Into<CloneArgs> + std::fmt::Debug,
{
  let CloneArgs {
    name,
    provider,
    https,
    repo,
    branch,
    commit,
    destination,
    on_clone,
    on_pull,
    ..
  } = clone_args.into();

  let provider = provider
    .as_ref()
    .context("resource has no provider attached")?;
  let repo =
    repo.as_ref().context("resource has no repo attached")?;
  let name = to_monitor_name(&name);

  let repo_dir = match destination {
    Some(destination) => PathBuf::from_str(&destination)
      .context("destination is not valid path")?,
    None => repo_dir.join(name),
  };

  let mut logs = clone_inner(
    provider,
    https,
    repo,
    &branch,
    &commit,
    &repo_dir,
    access_token,
  )
  .await;

  if !all_logs_success(&logs) {
    tracing::warn!("failed to clone repo at {repo_dir:?}");
    return Ok((logs, None, None));
  }

  tracing::debug!("repo at {repo_dir:?} cloned");

  let (hash, message) = match get_commit_hash_log(&repo_dir).await {
    Ok((log, hash, message)) => {
      logs.push(log);
      (Some(hash), Some(message))
    }
    Err(e) => {
      logs.push(Log::simple(
        "latest commit",
        format_serror(
          &e.context("failed to get latest commit").into(),
        ),
      ));
      (None, None)
    }
  };

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
      tracing::debug!(
        "run repo on_clone command | command: {} | cwd: {:?}",
        command.command,
        on_clone_path
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
      tracing::debug!(
        "run repo on_pull command | command: {} | cwd: {:?}",
        command.command,
        on_pull_path
      );
      logs.push(on_pull_log);
    }
  }
  Ok((logs, hash, message))
}

#[tracing::instrument(
  level = "debug",
  skip(destination, access_token)
)]
async fn clone_inner(
  provider: &str,
  https: bool,
  repo: &str,
  branch: &Option<String>,
  commit: &Option<String>,
  destination: &Path,
  access_token: Option<String>,
) -> Vec<Log> {
  let _ = std::fs::remove_dir_all(destination);
  let access_token_at = match &access_token {
    Some(token) => format!("{token}@"),
    None => String::new(),
  };
  let branch = match branch {
    Some(branch) => format!(" -b {branch}"),
    None => String::new(),
  };
  let protocol = if https { "https" } else { "http" };
  let repo_url =
    format!("{protocol}://{access_token_at}{provider}/{repo}.git");
  let command =
    format!("git clone {repo_url} {}{branch}", destination.display());
  let start_ts = monitor_timestamp();
  let output = async_run_command(&command).await;
  let success = output.success();
  let (command, stderr) = if !access_token_at.is_empty() {
    // know that access token can't be none if access token non-empty
    let access_token = access_token.unwrap();
    (
      command.replace(&access_token, "<TOKEN>"),
      output.stderr.replace(&access_token, "<TOKEN>"),
    )
  } else {
    (command, output.stderr)
  };
  let mut logs = vec![Log {
    stage: "clone repo".to_string(),
    command,
    success,
    stdout: output.stdout,
    stderr,
    start_ts,
    end_ts: monitor_timestamp(),
  }];

  if !logs[0].success {
    return logs;
  }

  if let Some(commit) = commit {
    let reset_log = run_monitor_command(
      "set commit",
      format!(
        "cd {} && git reset --hard {commit}",
        destination.display()
      ),
    )
    .await;
    logs.push(reset_log);
  }

  logs
}

#[instrument(level = "debug")]
pub async fn get_commit_hash_info(
  repo_dir: &Path,
) -> anyhow::Result<LatestCommit> {
  let command = format!("cd {} && git rev-parse --short HEAD && git rev-parse HEAD && git log -1 --pretty=%B", repo_dir.display());
  let output = async_run_command(&command).await;
  let mut split = output.stdout.split('\n');
  let (hash, _, message) = (
    split
      .next()
      .context("failed to get short commit hash")?
      .to_string(),
    split.next().context("failed to get long commit hash")?,
    split
      .next()
      .context("failed to get commit message")?
      .to_string(),
  );
  Ok(LatestCommit { hash, message })
}

/// returns (Log, commit hash, commit message)
#[instrument(level = "debug")]
pub async fn get_commit_hash_log(
  repo_dir: &Path,
) -> anyhow::Result<(Log, String, String)> {
  let start_ts = monitor_timestamp();
  let command = format!("cd {} && git rev-parse --short HEAD && git rev-parse HEAD && git log -1 --pretty=%B", repo_dir.display());
  let output = async_run_command(&command).await;
  let mut split = output.stdout.split('\n');
  let (short_hash, _, msg) = (
    split
      .next()
      .context("failed to get short commit hash")?
      .to_string(),
    split.next().context("failed to get long commit hash")?,
    split
      .next()
      .context("failed to get commit message")?
      .to_string(),
  );
  let log = Log {
    stage: "latest commit".into(),
    command,
    stdout: format!(
      "{} {}\n{} {}",
      muted("hash:"),
      bold(&short_hash),
      muted("message:"),
      bold(&msg),
    ),
    stderr: String::new(),
    success: true,
    start_ts,
    end_ts: monitor_timestamp(),
  };
  Ok((log, short_hash, msg))
}
