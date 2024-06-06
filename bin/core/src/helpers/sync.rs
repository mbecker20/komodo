use anyhow::{anyhow, Context};
use monitor_client::entities::{
  sync::ResourceSync, toml::ResourcesToml, update::Log, CloneArgs,
};
use sync::file::read_resources;

use crate::config::core_config;

pub async fn get_remote_resources(
  sync: &ResourceSync,
) -> anyhow::Result<(anyhow::Result<ResourcesToml>, Vec<Log>)> {
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

  let clone_logs =
    git::clone(clone_args, &config.sync_directory, github_token)
      .await
      .context("failed to clone resource repo")?;

  let repo_path = config.sync_directory.join(&sync.name);
  let resource_path = repo_path.join(&sync.config.resource_path);

  let res = read_resources(&resource_path);

  if let Err(e) = std::fs::remove_dir_all(&repo_path) {
    warn!("failed to remove sync repo directory | {e:?}")
  }

  Ok((res, clone_logs))
}
