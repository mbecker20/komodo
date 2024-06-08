use std::fs;

use anyhow::{anyhow, Context};
use monitor_client::entities::{
  sync::ResourceSync, to_monitor_name, toml::ResourcesToml,
  update::Log, CloneArgs, LatestCommit,
};

use crate::config::core_config;

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
  let name = to_monitor_name(&sync.name);
  let clone_args: CloneArgs = sync.into();

  let config = core_config();

  let github_token = clone_args
    .github_account
    .as_ref()
    .map(|account| {
      config.github_accounts.get(account).ok_or_else(|| {
        anyhow!("did not find github token for account {account}")
      })
    })
    .transpose()?
    .cloned();

  fs::create_dir_all(&config.sync_directory)
    .context("failed to create sync directory")?;

  let mut logs =
    git::clone(clone_args, &config.sync_directory, github_token)
      .await
      .context("failed to clone resource repo")?;

  let repo_dir = config.sync_directory.join(&name);
  let LatestCommit { hash, message } =
    git::get_commit_hash_info(&repo_dir)
      .await
      .context("failed to get commit hash info")?;

  let repo_path = config.sync_directory.join(&sync.name);
  let resource_path = repo_path.join(&sync.config.resource_path);

  let res = super::file::read_resources(&resource_path).map(
    |(resources, log)| {
      logs.push(log);
      resources
    },
  );

  if let Err(e) = std::fs::remove_dir_all(&repo_path) {
    warn!("failed to remove sync repo directory | {e:?}")
  }

  Ok((res, logs, hash, message))
}
