use anyhow::anyhow;
use command::run_monitor_command;
use monitor_client::entities::{
  build::{CloudRegistryConfig, ImageRegistry},
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
      async_run_command(&format!(
        "docker login -u {account} -p {registry_token}",
      ))
      .await;
      Ok(true)
    }
    ImageRegistry::Ghcr(CloudRegistryConfig {
      account,
      ..
    }) => {
      if account.is_empty() {
        return Err(anyhow!(
          "Must configure account for GithubContainerRegistry"
        ));
      }
      let registry_token = match registry_token {
        Some(token) => token,
        None => get_github_token(account)?,
      };
      async_run_command(&format!(
        "docker login ghcr.io -u {account} -p {registry_token}",
      ))
      .await;
      Ok(true)
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
