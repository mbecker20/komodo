use std::{fs, path::Path};

use anyhow::{anyhow, Context};
use clap::ValueEnum;
use monitor_client::{
  api::write,
  entities::{
    build::PartialBuildConfig, deployment::PartialDeploymentConfig,
    resource::Resource, server::PartialServerConfig,
  },
};
use serde::Deserialize;

use crate::{maps::name_to_server, monitor_client, wait_for_enter};

pub async fn run_resource(
  action: SyncDirection,
  path: &Path,
) -> anyhow::Result<()> {
  info!("action: {action:?} | path: {path:?}");

  let resources = read_resources(path)?;

  match action {
    SyncDirection::Up => run_resource_up(resources).await,
    SyncDirection::Down => {
      todo!()
    }
  }
}

async fn run_resource_up(
  resources: ResourceFile,
) -> anyhow::Result<()> {
  let servers = name_to_server();

  // (name, partial config)
  let mut to_update =
    Vec::<(String, Resource<PartialServerConfig>)>::new();
  let mut to_create = Vec::<Resource<PartialServerConfig>>::new();

  for server in resources.servers {
    match servers.get(&server.name).map(|s| s.id.clone()) {
      Some(id) => {
        to_update.push((id, server));
      }
      None => {
        to_create.push(server);
      }
    }
  }

  if !to_create.is_empty() {
    println!(
      "\nTO CREATE: {}",
      to_create
        .iter()
        .map(|server| server.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
    );
  }

  if !to_update.is_empty() {
    println!(
      "\nTO UPDATE: {}",
      to_update
        .iter()
        .map(|(_, server)| server.name.as_str())
        .collect::<Vec<_>>()
        .join(", ")
    );
  }

  wait_for_enter("CONTINUE")?;

  for (id, server) in to_update {
    if let Err(e) = monitor_client()
      .write(write::UpdateServer {
        id,
        config: server.config,
      })
      .await
    {
      warn!("failed to update server {} | {e:#}", server.name)
    }
  }

  for server in to_create {
    if let Err(e) = monitor_client()
      .write(write::CreateServer {
        name: server.name.clone(),
        config: server.config,
      })
      .await
    {
      warn!("failed to create server {} | {e:#}", server.name)
    }
  }

  Ok(())
}

/// Specifies resources to sync on monitor
#[derive(Debug, Clone, Default, Deserialize)]
struct ResourceFile {
  #[serde(default, rename = "server")]
  servers: Vec<Resource<PartialServerConfig>>,
  #[serde(default, rename = "build")]
  builds: Vec<Resource<PartialBuildConfig>>,
  #[serde(default, rename = "deployment")]
  deployments: Vec<Resource<PartialDeploymentConfig>>,
  // #[serde(rename = "builder")]
  // builders: (),
  // #[serde(rename = "repo")]
  // repos: (),
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SyncDirection {
  /// Brings up resources / updates
  Up,
  /// Takes down / deletes resources
  Down,
}

fn read_resources(path: &Path) -> anyhow::Result<ResourceFile> {
  let mut res = ResourceFile::default();
  read_resources_recursive(path, &mut res)?;
  Ok(res)
}

fn read_resources_recursive(
  path: &Path,
  resources: &mut ResourceFile,
) -> anyhow::Result<()> {
  let res =
    fs::metadata(path).context("failed to get path metadata")?;
  if res.is_file() {
    if !path
      .extension()
      .map(|ext| ext == "toml")
      .unwrap_or_default()
    {
      return Ok(());
    }
    let more = match crate::parse_toml_file::<ResourceFile>(path) {
      Ok(res) => res,
      Err(e) => {
        warn!("failed to parse {:?}. skipping file | {e:#}", path);
        return Ok(());
      }
    };
    info!("adding resources from {path:?}");
    resources.servers.extend(more.servers);
    resources.builds.extend(more.builds);
    resources.deployments.extend(more.deployments);
    Ok(())
  } else if res.is_dir() {
    let directory = fs::read_dir(path)
      .context("failed to read directory contents")?;
    for entry in directory.into_iter().flatten() {
      if let Err(e) =
        read_resources_recursive(&entry.path(), resources)
      {
        warn!("failed to read additional resources at path | {e:#}");
      }
    }
    Ok(())
  } else {
    Err(anyhow!("resources path is neither file nor directory"))
  }
}
