use std::{fs, path::Path};

use anyhow::{anyhow, Context};
use monitor_client::entities::{
  alerter::PartialAlerterConfig, build::PartialBuildConfig,
  builder::PartialBuilderConfig, deployment::PartialDeploymentConfig,
  procedure::Procedure, repo::PartialRepoConfig, resource::Resource,
  server::PartialServerConfig,
};
use serde::Deserialize;

/// Specifies resources to sync on monitor
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ResourceFile {
  #[serde(default, rename = "server")]
  pub servers: Vec<Resource<PartialServerConfig>>,
  #[serde(default, rename = "build")]
  pub builds: Vec<Resource<PartialBuildConfig>>,
  #[serde(default, rename = "deployment")]
  pub deployments: Vec<Resource<PartialDeploymentConfig>>,
  #[serde(default, rename = "builder")]
  pub builders: Vec<Resource<PartialBuilderConfig>>,
  #[serde(default, rename = "repo")]
  pub repos: Vec<Resource<PartialRepoConfig>>,
  #[serde(default, rename = "alerter")]
  pub alerters: Vec<Resource<PartialAlerterConfig>>,
  #[serde(default, rename = "procedure")]
  pub procedures: Vec<Procedure>,
}

pub fn read_resources(path: &Path) -> anyhow::Result<ResourceFile> {
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
    resources.builders.extend(more.builders);
    resources.repos.extend(more.repos);
    resources.alerters.extend(more.alerters);
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
