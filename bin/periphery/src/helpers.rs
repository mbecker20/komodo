use anyhow::{anyhow, Context};
use monitor_client::entities::{
  build::{StandardRegistryConfig, ImageRegistry},
  config::core::AwsEcrConfig,
  EnvironmentVar,
};
use run_command::async_run_command;

use crate::config::periphery_config;

pub fn get_git_token(
  domain: &str,
  account_username: &str,
) -> anyhow::Result<&'static String> {
  periphery_config()
    .git_providers
    .iter()
    .find(|_provider| _provider.domain == domain)
    .and_then(|provider| provider.accounts
        .iter()
          .find(|account| account.username == account_username).map(|account| &account.token))
    .with_context(|| format!("did not find token in config for git account {account_username} | domain {domain}"))
}

pub fn get_docker_token(
  domain: &str,
  account_username: &str,
) -> anyhow::Result<&'static String> {
  periphery_config()
    .docker_registries
    .iter().find(|registry| registry.domain == domain)
    .and_then(|registry| registry.accounts.iter().find(|account| account.username == account_username).map(|account| &account.token))
    .with_context(|| format!("did not find token in config for docker account {account_username} | domain {domain}"))
}

pub fn parse_extra_args(extra_args: &[String]) -> String {
  let args = extra_args.join(" ");
  if !args.is_empty() {
    format!(" {args}")
  } else {
    args
  }
}

pub fn parse_labels(labels: &[EnvironmentVar]) -> String {
  labels
    .iter()
    .map(|p| format!(" --label {}=\"{}\"", p.variable, p.value))
    .collect::<Vec<_>>()
    .join("")
}

/// Returns whether build result should be pushed after build
#[instrument(skip(registry_token))]
pub async fn docker_login(
  registry: &ImageRegistry,
  // For local token override from core.
  registry_token: Option<&str>,
  // For local config override from core.
  aws_ecr: Option<&AwsEcrConfig>,
) -> anyhow::Result<bool> {
  let (domain, account) = match registry {
    // Early return for no login
    ImageRegistry::None(_) => return Ok(false),
    // Early return because Ecr is different
    ImageRegistry::AwsEcr(label) => {
      let AwsEcrConfig { region, account_id } = aws_ecr
        .with_context(|| {
          if label.is_empty() {
            String::from("Could not find aws ecr config")
          } else {
            format!("Could not find aws ecr config for label {label}")
          }
        })?;
      let registry_token = registry_token
        .context("aws ecr build missing registry token from core")?;
      let command = format!("docker login {account_id}.dkr.ecr.{region}.amazonaws.com -u AWS -p {registry_token}");
      let log = async_run_command(&command).await;
      if log.success() {
        return Ok(true);
      } else {
        return Err(anyhow!(
          "aws ecr login error: stdout: {} | stderr: {}",
          log.stdout,
          log.stderr
        ));
      }
    }
    ImageRegistry::Standard(StandardRegistryConfig {
      domain,
      account,
      ..
    }) => (domain.as_str(), account),
  };
  if account.is_empty() {
    return Err(anyhow!("Must configure account for registry domain {domain}, got empty string"));
  }
  let registry_token = match registry_token {
    Some(token) => token,
    None => get_docker_token(domain, account)?,
  };
  let log = async_run_command(&format!(
    "docker login {domain} -u {account} -p {registry_token}",
  ))
  .await;
  if log.success() {
    Ok(true)
  } else {
    Err(anyhow!(
      "{domain} login error: stdout: {} | stderr: {}",
      log.stdout,
      log.stderr
    ))
  }
}
