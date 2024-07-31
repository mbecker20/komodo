use std::fs;

use anyhow::{anyhow, Context};
use monitor_client::entities::{
  stack::Stack, to_monitor_name, update::Log, CloneArgs, LatestCommit,
};

use crate::{config::core_config, state::stack_lock_cache};

pub async fn get_remote_compose_file(
  stack: &Stack,
) -> anyhow::Result<(
  anyhow::Result<String>,
  Vec<Log>,
  // commit short hash
  String,
  // commit message
  String,
)> {
  let name = to_monitor_name(&stack.name);
  let mut clone_args: CloneArgs = stack.into();

  let config = core_config();

  let access_token = match (&clone_args.account, &clone_args.provider) {
    (None, _) => None,
    (Some(_), None) => return Err(anyhow!("Account is configured, but provider is empty")),
    (Some(username), Some(provider)) => config
      .git_providers
      .iter()
      .find(|_provider| {
        &_provider.domain == provider
      })
      .and_then(|provider| {
        clone_args.https = provider.https;
        provider.accounts.iter().find(|account| &account.username == username).map(|account| &account.token)
      })
      .with_context(|| format!("did not find git token for account {username} | provider: {provider}"))?
      .to_owned()
      .into(),
  };

  fs::create_dir_all(&config.stack_directory)
    .context("failed to create stack directory")?;

  // lock simultaneous access to same directory
  let lock =
    stack_lock_cache().get_or_insert_default(&stack.id).await;
  let _lock = lock.lock().await;

  // delete anything at the pat

  let logs =
    git::clone(clone_args, &config.stack_directory, access_token)
      .await
      .context("failed to clone stack repo")?;

  let repo_dir = config.stack_directory.join(&name);
  let LatestCommit { hash, message } =
    git::get_commit_hash_info(&repo_dir)
      .await
      .context("failed to get commit hash info")?;

  let repo_path = config.stack_directory.join(&stack.name);
  let file_path = repo_path.join(&stack.config.file_path);

  let res = fs::read_to_string(file_path)
    .context("failed to read file contents");

  if let Err(e) = std::fs::remove_dir_all(&repo_path) {
    warn!("failed to remove stack repo directory | {e:?}")
  }

  Ok((res, logs, hash, message))
}
