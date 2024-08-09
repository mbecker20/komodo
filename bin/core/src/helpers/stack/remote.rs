use std::{fs, path::Path};

use anyhow::{anyhow, Context};
use formatting::format_serror;
use monitor_client::entities::{
  stack::{ComposeContents, Stack},
  update::Log,
  CloneArgs,
};

use crate::{config::core_config, helpers::{git_token, random_string}};

/// Returns Result<(read paths, error paths, logs, short hash, commit message)>
pub async fn get_remote_compose_contents(
  stack: &Stack,
  // Collect any files which are missing in the repo.
  mut missing_files: Option<&mut Vec<String>>,
) -> anyhow::Result<(
  // Successful contents
  Vec<ComposeContents>,
  // error contents
  Vec<ComposeContents>,
  // logs
  Vec<Log>,
  // commit short hash
  Option<String>,
  // commit message
  Option<String>,
)> {
  let repo_path =
    core_config().stack_directory.join(random_string(10));

  let (logs, hash, message) = clone_remote_repo(&repo_path, stack)
    .await
    .context("failed to clone stack repo")?;

  let run_directory = repo_path.join(&stack.config.run_directory);

  let mut oks = Vec::new();
  let mut errs = Vec::new();

  for path in stack.file_paths() {
    let file_path = run_directory.join(path);
    if !file_path.exists() {
      if let Some(missing_files) = &mut missing_files {
        missing_files.push(path.to_string());
      }
    }
    // If file does not exist, will show up in err case so the log is handled
    match fs::read_to_string(&file_path).with_context(|| {
      format!("failed to read file contents from {file_path:?}")
    }) {
      Ok(contents) => oks.push(ComposeContents {
        path: path.to_string(),
        contents,
      }),
      Err(e) => errs.push(ComposeContents {
        path: path.to_string(),
        contents: format_serror(&e.into()),
      }),
    }
  }

  Ok((oks, errs, logs, hash, message))
}

/// Returns (logs, hash, message)
pub async fn clone_remote_repo(
  repo_path: &Path,
  stack: &Stack,
) -> anyhow::Result<(Vec<Log>, Option<String>, Option<String>)> {
  let mut clone_args: CloneArgs = stack.into();

  let config = core_config();

  let access_token = match (&clone_args.account, &clone_args.provider)
  {
    (None, _) => None,
    (Some(_), None) => {
      return Err(anyhow!(
        "Account is configured, but provider is empty"
      ))
    }
    (Some(username), Some(provider)) => {
      git_token(provider, username, |https| {
        clone_args.https = https
      })
      .await
      .with_context(
        || format!("Failed to get git token in call to db. Stopping run. | {provider} | {username}"),
      )?
    }
  };

  clone_args.destination = Some(repo_path.display().to_string());

  git::clone(clone_args, &config.stack_directory, access_token)
    .await
    .context("failed to clone stack repo")
}
