use std::path::Path;

use anyhow::Context;
use command::run_komodo_command;
use formatting::format_serror;
use komodo_client::entities::{all_logs_success, update::Log};
use tokio::fs;

use crate::{get_commit_hash_log, GitRes};

/// Write file, add, commit, force push.
/// Repo must be cloned.
pub async fn write_commit_file(
  repo_dir: &Path,
  // relative to repo root
  file: &Path,
  contents: &str,
) -> anyhow::Result<GitRes> {
  let path = repo_dir.join(file);

  if let Some(parent) = path.parent() {
    let _ = fs::create_dir_all(&parent).await;
  }

  fs::write(&path, contents).await.with_context(|| {
    format!("Failed to write contents to {path:?}")
  })?;

  Ok(commit_file(repo_dir, file).await)
}

/// Add file, commit, force push.
/// Repo must be cloned.
pub async fn commit_file(
  repo_dir: &Path,
  // relative to repo root
  file: &Path,
) -> GitRes {
  let mut res = GitRes::default();

  let add_log = run_komodo_command(
    "add files",
    repo_dir,
    format!("git add {}", file.display()),
  )
  .await;
  res.logs.push(add_log);
  if !all_logs_success(&res.logs) {
    return res;
  }

  let commit_log = run_komodo_command(
    "commit",
    repo_dir,
    format!("git commit -m \"Komodo: update {file:?}\""),
  )
  .await;
  res.logs.push(commit_log);
  if !all_logs_success(&res.logs) {
    return res;
  }

  match get_commit_hash_log(repo_dir).await {
    Ok((log, hash, message)) => {
      res.logs.push(log);
      res.hash = Some(hash);
      res.message = Some(message);
    }
    Err(e) => {
      res.logs.push(Log::error(
        "get commit hash",
        format_serror(&e.into()),
      ));
      return res;
    }
  };

  let push_log =
    run_komodo_command("push", repo_dir, format!("git push -f"))
      .await;
  res.logs.push(push_log);

  res
}

/// Add, commit, and force push.
/// Repo must be cloned.
pub async fn commit_all(repo_dir: &Path, message: &str) -> GitRes {
  let mut res = GitRes::default();

  let add_log =
    run_komodo_command("add files", repo_dir, "git add -A").await;
  res.logs.push(add_log);
  if !all_logs_success(&res.logs) {
    return res;
  }

  let commit_log = run_komodo_command(
    "commit",
    repo_dir,
    format!("git commit -m \"{message}\""),
  )
  .await;
  res.logs.push(commit_log);
  if !all_logs_success(&res.logs) {
    return res;
  }

  match get_commit_hash_log(repo_dir).await {
    Ok((log, hash, message)) => {
      res.logs.push(log);
      res.hash = Some(hash);
      res.message = Some(message);
    }
    Err(e) => {
      res.logs.push(Log::error(
        "get commit hash",
        format_serror(&e.into()),
      ));
      return res;
    }
  };

  let push_log =
    run_komodo_command("push", repo_dir, format!("git push -f"))
      .await;
  res.logs.push(push_log);

  res
}
