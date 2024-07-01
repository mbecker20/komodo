use anyhow::{anyhow, Context};
use monitor_client::entities::{
  build::{CloudRegistryConfig, ImageRegistry},
  config::core::AwsEcrConfig,
  EnvironmentVar,
};
use run_command::async_run_command;

use crate::config::periphery_config;

pub fn get_github_token(
  github_account: &String,
) -> anyhow::Result<&'static String> {
  periphery_config()
    .github_accounts
    .get(github_account)
    .with_context(|| format!("did not find token in config for github account {github_account}"))
}

pub fn get_docker_token(
  docker_account: &String,
) -> anyhow::Result<&'static String> {
  periphery_config()
    .docker_accounts
    .get(docker_account)
    .with_context(|| format!("did not find token in config for docker account {docker_account}"))
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
  match registry {
    ImageRegistry::None(_) => Ok(false),
    ImageRegistry::DockerHub(CloudRegistryConfig {
      account, ..
    }) => {
      if account.is_empty() {
        return Err(anyhow!(
          "Must configure account for DockerHub registry"
        ));
      }
      let registry_token = match registry_token {
        Some(token) => token,
        None => get_docker_token(account)?,
      };
      let log = async_run_command(&format!(
        "docker login -u {account} -p {registry_token}",
      ))
      .await;
      if log.success() {
        Ok(true)
      } else {
        Err(anyhow!(
          "dockerhub login error: stdout: {} | stderr: {}",
          log.stdout,
          log.stderr
        ))
      }
    }
    ImageRegistry::Ghcr(CloudRegistryConfig { account, .. }) => {
      if account.is_empty() {
        return Err(anyhow!(
          "Must configure account for GithubContainerRegistry"
        ));
      }
      let registry_token = match registry_token {
        Some(token) => token,
        None => get_github_token(account)?,
      };
      let log = async_run_command(&format!(
        "docker login ghcr.io -u {account} -p {registry_token}",
      ))
      .await;
      if log.success() {
        Ok(true)
      } else {
        Err(anyhow!(
          "ghcr login error: stdout: {} | stderr: {}",
          log.stdout,
          log.stderr
        ))
      }
    }
    ImageRegistry::AwsEcr(label) => {
      let AwsEcrConfig { region, account_id } = aws_ecr
        .with_context(|| {
          format!("Could not find aws ecr config for label {label}")
        })?;
      let registry_token = registry_token
        .context("aws ecr build missing registry token from core")?;
      let log = async_run_command(&format!("docker login {account_id}.dkr.ecr.{region}.amazonaws.com -u AWS -p {registry_token}")).await;
      if log.success() {
        Ok(true)
      } else {
        Err(anyhow!(
          "aws ecr login error: stdout: {} | stderr: {}",
          log.stdout,
          log.stderr
        ))
      }
    }
    ImageRegistry::Custom(_) => todo!(),
  }
}
