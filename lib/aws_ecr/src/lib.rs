use anyhow::{anyhow, Context};
use aws_config::{BehaviorVersion, Region};
use aws_sdk_ecr::Client as EcrClient;

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

/// Gets a token for the default registry only
#[tracing::instrument(skip_all)]
pub async fn get_ecr_token(
  client: &EcrClient,
) -> anyhow::Result<String> {
  let Some(tokens) = client
    .get_authorization_token()
    .send()
    .await
    .context("failed to get authorization token")?
    .authorization_data
  else {
    return Err(anyhow!("No authorization data"));
  };

  let token = tokens
    .into_iter()
    .next()
    .context("No tokens in response")?
    .authorization_token
    .context("no token on authorization token repsonse")?;

  Ok(token)
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
