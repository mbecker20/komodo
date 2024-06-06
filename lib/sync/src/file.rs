use std::{fs, path::Path};

use anyhow::{anyhow, Context};
use colored::Colorize;
use monitor_client::entities::toml::ResourcesToml;
use serde::de::DeserializeOwned;

pub fn read_resources(path: &Path) -> anyhow::Result<ResourcesToml> {
  let mut res = ResourcesToml::default();
  read_resources_recursive(path, &mut res)?;
  Ok(res)
}

fn read_resources_recursive(
  path: &Path,
  resources: &mut ResourcesToml,
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
    let more = match parse_toml_file::<ResourcesToml>(path) {
      Ok(res) => res,
      Err(e) => {
        tracing::warn!(
          "failed to parse {:?}. skipping file | {e:#}",
          path
        );
        return Ok(());
      }
    };
    tracing::info!(
      "{} from {}",
      "adding resources".green().bold(),
      path.display().to_string().blue().bold()
    );
    resources.servers.extend(more.servers);
    resources.deployments.extend(more.deployments);
    resources.repos.extend(more.repos);
    resources.builds.extend(more.builds);
    resources.procedures.extend(more.procedures);
    resources.builders.extend(more.builders);
    resources.alerters.extend(more.alerters);
    resources.server_templates.extend(more.server_templates);
    resources.user_groups.extend(more.user_groups);
    resources.variables.extend(more.variables);
    Ok(())
  } else if res.is_dir() {
    let directory = fs::read_dir(path)
      .context("failed to read directory contents")?;
    for entry in directory.into_iter().flatten() {
      if let Err(e) =
        read_resources_recursive(&entry.path(), resources)
      {
        tracing::warn!(
          "failed to read additional resources at path | {e:#}"
        );
      }
    }
    Ok(())
  } else {
    Err(anyhow!("resources path is neither file nor directory"))
  }
}

pub fn parse_toml_file<T: DeserializeOwned>(
  path: impl AsRef<std::path::Path>,
) -> anyhow::Result<T> {
  let contents = std::fs::read_to_string(path)
    .context("failed to read file contents")?;
  toml::from_str(&contents).context("failed to parse toml contents")
}
