use std::{
  collections::HashMap,
  path::{Path, PathBuf},
  str::FromStr,
};

use anyhow::Context;
use command::run_monitor_command;
use formatting::{bold, format_serror, muted};
use monitor_client::entities::{
  all_logs_success, environment_vars_to_string, monitor_timestamp,
  to_monitor_name, update::Log, CloneArgs, EnvironmentVar,
  LatestCommit, SystemCommand,
};
use run_command::async_run_command;
use tokio::fs;
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
    format!("cd {} && git pull -f origin {branch}", path.display());

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

/// (logs, commit hash, commit message, env_file_path)
pub type CloneRes =
  (Vec<Log>, Option<String>, Option<String>, Option<PathBuf>);

/// returns (logs, commit hash, commit message, env_file_path)
#[tracing::instrument(level = "debug", skip(access_token))]
pub async fn clone<T>(
  clone_args: T,
  repo_dir: &Path,
  access_token: Option<String>,
  environment: &[EnvironmentVar],
  env_file_path: &str,
  // if skip_secret_interp is none, make sure to pass None here
  secrets: Option<&HashMap<String, String>>,
) -> anyhow::Result<CloneRes>
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
    tracing::warn!("failed to clone repo at {repo_dir:?} | {logs:?}");
    return Ok((logs, None, None, None));
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

  let Ok(env_file_path) = write_environment_file(
    environment,
    env_file_path,
    secrets,
    &repo_dir,
    &mut logs,
  )
  .await
  else {
    return Ok((logs, hash, message, None));
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

  Ok((logs, hash, message, env_file_path))
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

/// If the environment was written and needs to be passed to the compose command,
/// will return the env file PathBuf
pub async fn write_environment_file(
  environment: &[EnvironmentVar],
  env_file_path: &str,
  secrets: Option<&HashMap<String, String>>,
  folder: &Path,
  logs: &mut Vec<Log>,
) -> Result<Option<PathBuf>, ()> {
  if environment.is_empty() {
    return Ok(None);
  }

  let contents = environment_vars_to_string(environment);

  let contents = if let Some(secrets) = secrets {
    let res = svi::interpolate_variables(
      &contents,
      secrets,
      svi::Interpolator::DoubleBrackets,
      true,
    )
    .context("failed to interpolate secrets into environment");

    let (contents, replacers) = match res {
      Ok(res) => res,
      Err(e) => {
        logs.push(Log::error(
          "interpolate periphery secrets",
          format_serror(&e.into()),
        ));
        return Err(());
      }
    };

    if !replacers.is_empty() {
      logs.push(Log::simple(
        "interpolate periphery secrets",
        replacers
            .iter()
            .map(|(_, variable)| format!("<span class=\"text-muted-foreground\">replaced:</span> {variable}"))
            .collect::<Vec<_>>()
            .join("\n"),
      ))
    }

    contents
  } else {
    contents
  };

  let file = folder.join(env_file_path);

  if let Err(e) =
    fs::write(&file, contents).await.with_context(|| {
      format!("failed to write environment file to {file:?}")
    })
  {
    logs.push(Log::error(
      "write environment file",
      format_serror(&e.into()),
    ));
    return Err(());
  }

  logs.push(Log::simple(
    "write environment file",
    format!("environment written to {file:?}"),
  ));

  Ok(Some(file))
}
