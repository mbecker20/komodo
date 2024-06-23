use anyhow::{anyhow, Context};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_ecr::Client as EcrClient;
use run_command::async_run_command;

#[tracing::instrument(skip(access_key_id, secret_access_key))]
pub async fn make_ecr_client(
  region: String,
  access_key_id: &str,
  secret_access_key: &str,
) -> EcrClient {
  std::env::set_var("AWS_ACCESS_KEY_ID", access_key_id);
  std::env::set_var("AWS_SECRET_ACCESS_KEY", secret_access_key);
  let region = Region::new(region);
  let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
    .region(region)
    .load()
    .await;
  EcrClient::new(&config)
}

/// Gets a token docker login.
///
/// Requires the aws cli be installed on the host
#[tracing::instrument(skip(access_key_id, secret_access_key))]
pub async fn get_ecr_token(
  region: &str,
  access_key_id: &str,
  secret_access_key: &str,
) -> anyhow::Result<String> {
  let log = async_run_command(&format!(
    "AWS_ACCESS_KEY_ID={access_key_id} AWS_SECRET_ACCESS_KEY={secret_access_key} aws ecr get-login-password --region {region}"
  ))
  .await;

  if log.success() {
    Ok(log.stdout)
  } else {
    Err(
      anyhow!("stdout: {} | stderr: {}", log.stdout, log.stderr)
        .context("failed to get aws ecr login token"),
    )
  }
}

#[tracing::instrument(skip(client))]
pub async fn maybe_create_repo(
  client: &EcrClient,
  repo: &str,
) -> anyhow::Result<()> {
  let existing = client
    .describe_repositories()
    .send()
    .await
    .context("failed to describe existing repositories")?
    .repositories
    .unwrap_or_default();

  if existing.iter().any(|r| {
    if let Some(name) = r.repository_name() {
      name == repo
    } else {
      false
    }
  }) {
    return Ok(());
  };

  client
    .create_repository()
    .repository_name(repo)
    .send()
    .await
    .context("failed to create repository")?;

  Ok(())
}
