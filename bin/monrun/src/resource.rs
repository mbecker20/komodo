use std::{fs, path::Path};

use anyhow::{anyhow, Context};
use clap::ValueEnum;
use monitor_client::entities::{
  build::Build, deployment::Deployment, server::Server,
};
use serde::Deserialize;

pub async fn run_resource(
  action: SyncDirection,
  path: &Path,
) -> anyhow::Result<()> {
  info!("action: {action:?} | path: {path:?}");

  let resources = read_resources(path)?;

  println!("{resources:#?}");

  Ok(())
}

/// Specifies resources to sync on monitor
#[derive(Debug, Clone, Default, Deserialize)]
struct ResourceFile {
  #[serde(default, rename = "server")]
  servers: Vec<Server>,
  #[serde(default, rename = "build")]
  builds: Vec<Build>,
  #[serde(default, rename = "deployment")]
  deployments: Vec<Deployment>,
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
