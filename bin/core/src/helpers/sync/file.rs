use std::{fs, path::Path};

use anyhow::{anyhow, Context};
use formatting::{colored, muted, Color};
use monitor_client::entities::{toml::ResourcesToml, update::Log};
use serde::de::DeserializeOwned;

pub fn read_resources(
  path: &Path,
) -> anyhow::Result<(ResourcesToml, Log)> {
  let mut res = ResourcesToml::default();
  let mut log =
    format!("{}: reading resources from {path:?}", muted("INFO"));
  read_resources_recursive(path, &mut res, &mut log).with_context(
    || format!("failed to read resources from {path:?}"),
  )?;
  Ok((res, Log::simple("read remote resources", log)))
}

fn read_resources_recursive(
  path: &Path,
  resources: &mut ResourcesToml,
  log: &mut String,
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
    let more = parse_toml_file::<ResourcesToml>(path)
      .context("failed to parse resource file")?;

    log.push('\n');
    log.push_str(&format!(
      "{}: {} from {}",
      muted("INFO"),
      colored("adding resources", Color::Green),
      colored(path.display(), Color::Blue)
    ));

    resources.servers.extend(more.servers);
    resources.deployments.extend(more.deployments);
    resources.builds.extend(more.builds);
    resources.repos.extend(more.repos);
    resources.procedures.extend(more.procedures);
    resources.builders.extend(more.builders);
    resources.alerters.extend(more.alerters);
    resources.server_templates.extend(more.server_templates);
    resources.resource_syncs.extend(more.resource_syncs);
    resources.user_groups.extend(more.user_groups);
    resources.variables.extend(more.variables);
    Ok(())
  } else if res.is_dir() {
    let directory = fs::read_dir(path)
      .context("failed to read directory contents")?;
    for entry in directory.into_iter().flatten() {
      let path = entry.path();
      read_resources_recursive(&path, resources, log).with_context(
        || format!("failed to read resources from {path:?}"),
      )?;
    }
    Ok(())
  } else {
    Err(anyhow!("resources path is neither file nor directory"))
  }
}

fn parse_toml_file<T: DeserializeOwned>(
  path: impl AsRef<std::path::Path>,
) -> anyhow::Result<T> {
  let contents = std::fs::read_to_string(path)
    .context("failed to read file contents")?;
  toml::from_str(&contents)
    // the error without this comes through with multiple lines (\n) and looks bad
    .map_err(|e| anyhow!("{e:#}"))
    .context("failed to parse toml contents")
}
