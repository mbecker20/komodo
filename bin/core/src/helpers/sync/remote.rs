use std::{fs, path::PathBuf};

use anyhow::{anyhow, Context};
use komodo_client::entities::{
  sync::ResourceSync, toml::ResourcesToml, update::Log, CloneArgs,
};

use crate::{
  config::core_config,
  helpers::{git_token, random_string},
  state::resource_sync_lock_cache,
};

pub async fn get_remote_resources(
  sync: &ResourceSync,
) -> anyhow::Result<(
  anyhow::Result<ResourcesToml>,
  Vec<Log>,
  // commit short hash
  Option<String>,
  // commit message
  Option<String>,
)> {
  if sync.config.files_on_host {
    // =============
    // FILES ON HOST
    // =============
    let path = sync
      .config
      .resource_path
      .parse::<PathBuf>()
      .context("Resource path is not valid path")?;
    let mut logs = Vec::new();
    let res =
      super::file::read_resources(&path).map(|(resources, log)| {
        logs.push(log);
        resources
      });
    return Ok((res, logs, None, None));
  } else if !sync.config.file_contents.is_empty() {
    // ==========
    // UI DEFINED
    // ==========
    let res =
      toml::from_str::<ResourcesToml>(&sync.config.file_contents)
        .context("Failed to parse UI defined resources");
    return Ok((
      res,
      vec![Log::simple(
        "Read from database",
        "Resources added from database file".to_string(),
      )],
      None,
      None,
    ));
  }

  // ===============
  // REPO BASED SYNC
  // ===============

  let mut clone_args: CloneArgs = sync.into();

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
      git_token(provider, username, |https| clone_args.https = https)
        .await
        .with_context(
          || format!("Failed to get git token in call to db. Stopping run. | {provider} | {username}"),
        )?
    }
  };

  fs::create_dir_all(&config.repo_directory)
    .context("failed to create sync directory")?;

  // lock simultaneous access to same directory
  let lock = resource_sync_lock_cache()
    .get_or_insert_default(&sync.id)
    .await;
  let _lock = lock.lock().await;

  let repo_path = config.repo_directory.join(random_string(10));
  // This overrides any other method of determining clone path.
  clone_args.destination = Some(repo_path.display().to_string());

  // Don't want to run these on core.
  clone_args.on_clone = None;
  clone_args.on_pull = None;

  let (mut logs, hash, message, _) = git::clone(
    clone_args,
    &config.repo_directory,
    access_token,
    &[],
    "",
    None,
    &[],
  )
  .await
  .context("failed to clone resource repo")?;

  let hash = hash.context("failed to get commit hash")?;
  let message =
    message.context("failed to get commit hash message")?;

  let resource_path = repo_path.join(&sync.config.resource_path);

  let res = super::file::read_resources(&resource_path).map(
    |(resources, log)| {
      logs.push(log);
      resources
    },
  );

  if repo_path.exists() {
    if let Err(e) = std::fs::remove_dir_all(&repo_path) {
      warn!("failed to remove sync repo directory | {e:?}")
    }
  }

  Ok((res, logs, Some(hash), Some(message)))
}
