use std::fs;

use anyhow::{anyhow, Context};
use monitor_client::entities::{
  stack::Stack, update::Log, CloneArgs,
};

use crate::{auth::random_string, config::core_config};

/// Return Result<(Result<contents>, logs, short hash, commit message)>
pub async fn get_remote_compose_file(
  stack: &Stack,
) -> anyhow::Result<(
  anyhow::Result<String>,
  Vec<Log>,
  // commit short hash
  Option<String>,
  // commit message
  Option<String>,
)> {
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

  let repo_path = config.stack_directory.join(random_string(10));
  clone_args.destination = Some(repo_path.display().to_string());

  let (logs, hash, message) =
    git::clone(clone_args, &config.stack_directory, access_token)
      .await
      .context("failed to clone stack repo")?;

  let file_path = repo_path
    .join(&stack.config.run_directory)
    .join(&stack.config.file_path);

  let res = fs::read_to_string(file_path)
    .context("failed to read file contents");

  if repo_path.exists() {
    if let Err(e) = std::fs::remove_dir_all(&repo_path) {
      warn!("failed to remove stack repo directory | {e:?}")
    }
  }

  Ok((res, logs, hash, message))
}
