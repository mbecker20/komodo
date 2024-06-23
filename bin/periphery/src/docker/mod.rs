use anyhow::{anyhow, Context};
use command::run_monitor_command;
use monitor_client::entities::{
  build::{CloudRegistryConfig, ImageRegistry},
  config::core::AwsEcrConfig,
  update::Log,
  EnvironmentVar,
};
use run_command::async_run_command;

use crate::helpers::{get_docker_token, get_github_token};

pub mod build;
pub mod client;
pub mod container;
pub mod network;

/// Returns whether build result should be pushed after build
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
      let AwsEcrConfig {
        region,
        account_id,
        access_key_id,
        secret_access_key,
      } = aws_ecr.with_context(|| {
        format!("Could not find aws ecr config for label {label}")
      })?;
      let registry_token = match registry_token {
        Some(token) => token.to_string(),
        None => aws_ecr::get_ecr_token(
          region,
          access_key_id,
          secret_access_key,
        )
        .await
        .with_context(|| {
          format!("failed to get aws ecr token for {label}")
        })?,
      };
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

#[instrument]
pub async fn prune_system() -> Log {
  let command = String::from("docker system prune -a -f");
  run_monitor_command("prune system", command).await
}
