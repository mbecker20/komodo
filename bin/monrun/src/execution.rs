use std::path::Path;

use anyhow::{anyhow, Context};
use futures::future::join_all;
use monitor_client::api::execute;
use serde::Deserialize;
use strum::Display;

use crate::monitor_client;

pub async fn run_execution(path: &Path) -> anyhow::Result<()> {
  let ExecutionFile { name, stages } = crate::parse_toml_file(path)?;

  info!("EXECUTION: {name}");
  info!("path: {path:?}");
  println!("{stages:#?}");

  crate::wait_for_enter("EXECUTE")?;

  run_stages(stages)
    .await
    .context("failed during a stage. terminating run.")?;

  info!("finished successfully ✅");

  Ok(())
}

/// Specifies sequence of stages (build / deploy) on resources
#[derive(Debug, Clone, Deserialize)]
pub struct ExecutionFile {
  pub name: String,
  #[serde(rename = "stage")]
  pub stages: Vec<Stage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Stage {
  pub name: String,
  pub action: ExecutionType,
  /// resource names
  pub targets: Vec<String>,
}

#[derive(Debug, Clone, Copy, Deserialize, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ExecutionType {
  Build,
  Deploy,
  StartContainer,
  StopContainer,
  DestroyContainer,
}

pub async fn run_stages(stages: Vec<Stage>) -> anyhow::Result<()> {
  for Stage {
    name,
    action,
    targets,
  } in stages
  {
    info!("running {action} stage: {name}... ⏳");
    match action {
      ExecutionType::Build => {
        trigger_builds_in_parallel(&targets).await?;
      }
      ExecutionType::Deploy => {
        redeploy_deployments_in_parallel(&targets).await?;
      }
      ExecutionType::StartContainer => {
        start_containers_in_parallel(&targets).await?
      }
      ExecutionType::StopContainer => {
        stop_containers_in_parallel(&targets).await?
      }
      ExecutionType::DestroyContainer => {
        destroy_containers_in_parallel(&targets).await?;
      }
    }
    info!("finished {action} stage: {name} ✅");
  }
  Ok(())
}

async fn redeploy_deployments_in_parallel(
  deployments: &[String],
) -> anyhow::Result<()> {
  let futes = deployments.iter().map(|deployment| async move {
    monitor_client()
      .execute(execute::Deploy { deployment: deployment.to_string(), stop_signal: None, stop_time: None })
      .await
      .with_context(|| format!("failed to deploy {deployment}"))
      .and_then(|update| {
        if update.success {
          Ok(())
        } else {
          Err(anyhow!(
            "failed to deploy {deployment}. operation unsuccessful, see monitor update"
          ))
        }
      })
  });
  join_all(futes).await.into_iter().collect()
}

async fn start_containers_in_parallel(
  deployments: &[String],
) -> anyhow::Result<()> {
  let futes = deployments.iter().map(|deployment| async move {
    monitor_client()
    .execute(execute::StartContainer { deployment: deployment.to_string() })
      .await
      .with_context(|| format!("failed to start container {deployment}"))
      .and_then(|update| {
        if update.success {
          Ok(())
        } else {
          Err(anyhow!(
            "failed to start container {deployment}. operation unsuccessful, see monitor update"
          ))
        }
      })
  });
  join_all(futes).await.into_iter().collect()
}

async fn stop_containers_in_parallel(
  deployments: &[String],
) -> anyhow::Result<()> {
  let futes = deployments.iter().map(|deployment| async move {
    monitor_client()
      .execute(execute::StopContainer { deployment: deployment.to_string(), signal: None, time: None })
      .await
      .with_context(|| format!("failed to stop container {deployment}"))
      .and_then(|update| {
        if update.success {
          Ok(())
        } else {
          Err(anyhow!(
            "failed to stop container {deployment}. operation unsuccessful, see monitor update"
          ))
        }
      })
  });
  join_all(futes).await.into_iter().collect()
}

async fn destroy_containers_in_parallel(
  deployments: &[String],
) -> anyhow::Result<()> {
  let futes = deployments.iter().map(|deployment| async move {
    monitor_client()
      .execute(execute::RemoveContainer { deployment: deployment.to_string(), signal: None, time: None })
      .await
      .with_context(|| format!("failed to destroy container {deployment}"))
      .and_then(|update| {
        if update.success {
          Ok(())
        } else {
          Err(anyhow!(
            "failed to destroy container {deployment}. operation unsuccessful, see monitor update"
          ))
        }
      })
  });
  join_all(futes).await.into_iter().collect()
}

async fn trigger_builds_in_parallel(
  builds: &[String],
) -> anyhow::Result<()> {
  let futes = builds.iter().map(|build| async move {
    monitor_client()
      .execute(execute::RunBuild { build: build.to_string() })
      .await
      .with_context(|| format!("failed to build {build}"))
      .and_then(|update| {
        if update.success {
          Ok(())
        } else {
          Err(anyhow!(
            "failed to build {build}. operation unsuccessful, see monitor update"
          ))
        }
      })
  });
  join_all(futes).await.into_iter().collect()
}
