use std::fs;

use anyhow::{anyhow, Context};
use monitor_client::entities::{
  sync::ResourceSync, toml::ResourcesToml, update::Log, CloneArgs,
};

use crate::{
  config::core_config, helpers::git_token,
  state::resource_sync_lock_cache,
};

pub async fn get_remote_resources(
  sync: &ResourceSync,
) -> anyhow::Result<(
  anyhow::Result<ResourcesToml>,
  Vec<Log>,
  // commit short hash
  String,
  // commit message
  String,
)> {
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

  fs::create_dir_all(&config.sync_directory)
    .context("failed to create sync directory")?;

  // lock simultaneous access to same directory
  let lock = resource_sync_lock_cache()
    .get_or_insert_default(&sync.id)
    .await;
  let _lock = lock.lock().await;

  let (mut logs, hash, message, _) = git::clone(
    clone_args,
    &config.sync_directory,
    access_token,
    &[],
    "",
    None,
  )
  .await
  .context("failed to clone resource repo")?;

  let hash = hash.context("failed to get commit hash")?;
  let message =
    message.context("failed to get commit hash message")?;

  let repo_path = config.sync_directory.join(&sync.name);
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

  Ok((res, logs, hash, message))
}
