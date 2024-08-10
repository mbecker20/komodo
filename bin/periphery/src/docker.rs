use std::sync::OnceLock;

use anyhow::{anyhow, Context};
use bollard::{container::ListContainersOptions, Docker};
use command::run_monitor_command;
use monitor_client::entities::{
  build::{ImageRegistry, StandardRegistryConfig},
  config::core::AwsEcrConfig,
  deployment::{
    ContainerSummary, DockerContainerStats, TerminationSignal,
  },
  server::{
    docker_image::ImageSummary, docker_network::DockerNetwork,
  },
  to_monitor_name,
  update::Log,
};
use run_command::async_run_command;

pub fn docker_client() -> &'static DockerClient {
  static DOCKER_CLIENT: OnceLock<DockerClient> = OnceLock::new();
  DOCKER_CLIENT.get_or_init(Default::default)
}

pub struct DockerClient {
  docker: Docker,
}

impl Default for DockerClient {
  fn default() -> DockerClient {
    DockerClient {
      docker: Docker::connect_with_local_defaults()
        .expect("failed to connect to docker daemon"),
    }
  }
}

impl DockerClient {
  pub async fn list_containers(
    &self,
  ) -> anyhow::Result<Vec<ContainerSummary>> {
    let res = self
      .docker
      .list_containers(Some(ListContainersOptions::<String> {
        all: true,
        ..Default::default()
      }))
      .await?
      .into_iter()
      .map(|container| {
        let info = ContainerSummary {
          id: container.id.unwrap_or_default(),
          name: container
            .names
            .context("no names on container")?
            .pop()
            .context("no names on container (empty vec)")?
            .replace('/', ""),
          image: container.image.unwrap_or(String::from("unknown")),
          state: container
            .state
            .context("no container state")?
            .parse()
            .context("failed to parse container state")?,
          status: container.status,
          labels: container.labels.unwrap_or_default(),
          network_mode: container
            .host_config
            .and_then(|config| config.network_mode),
          networks: container.network_settings.and_then(|settings| {
            settings
              .networks
              .map(|networks| networks.into_keys().collect())
          }),
        };
        Ok::<_, anyhow::Error>(info)
      })
      .collect::<anyhow::Result<Vec<ContainerSummary>>>()?;
    Ok(res)
  }

  pub async fn list_networks(
    &self,
  ) -> anyhow::Result<Vec<DockerNetwork>> {
    let networks = self
      .docker
      .list_networks::<String>(None)
      .await?
      .into_iter()
      .map(|network| network.into())
      .collect();
    Ok(networks)
  }

  pub async fn list_images(
    &self,
  ) -> anyhow::Result<Vec<ImageSummary>> {
    let images = self
      .docker
      .list_images::<String>(None)
      .await?
      .into_iter()
      .map(|i| i.into())
      .collect();
    Ok(images)
  }
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
    None => crate::helpers::registry_token(domain, account)?,
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

#[instrument]
pub async fn pull_image(image: &str) -> Log {
  let command = format!("docker pull {image}");
  run_monitor_command("docker pull", command).await
}

pub fn stop_container_command(
  container_name: &str,
  signal: Option<TerminationSignal>,
  time: Option<i32>,
) -> String {
  let container_name = to_monitor_name(container_name);
  let signal = signal
    .map(|signal| format!(" --signal {signal}"))
    .unwrap_or_default();
  let time = time
    .map(|time| format!(" --time {time}"))
    .unwrap_or_default();
  format!("docker stop{signal}{time} {container_name}")
}

pub async fn container_stats(
  container_name: Option<String>,
) -> anyhow::Result<Vec<DockerContainerStats>> {
  let format = "--format \"{{ json . }}\"";
  let container_name = match container_name {
    Some(name) => format!(" {name}"),
    None => "".to_string(),
  };
  let command =
    format!("docker stats{container_name} --no-stream {format}");
  let output = async_run_command(&command).await;
  if output.success() {
    let res = output
      .stdout
      .split('\n')
      .filter(|e| !e.is_empty())
      .map(|e| {
        let parsed = serde_json::from_str(e)
          .context(format!("failed at parsing entry {e}"))?;
        Ok(parsed)
      })
      .collect::<anyhow::Result<Vec<DockerContainerStats>>>()?;
    Ok(res)
  } else {
    Err(anyhow!("{}", output.stderr.replace('\n', "")))
  }
}
