use std::{
  fs,
  path::{Path, PathBuf},
};

use anyhow::{anyhow, Context};
use formatting::{colored, format_serror, muted, Color};
use komodo_client::entities::{
  toml::{ResourceToml, ResourcesToml},
  update::Log,
  FileContents,
};

pub fn read_resources(
  root_path: &Path,
  resource_path: &str,
  match_tags: &[String],
  logs: &mut Vec<Log>,
  files: &mut Vec<FileContents>,
  file_errors: &mut Vec<FileContents>,
) -> anyhow::Result<ResourcesToml> {
  let resource_path = resource_path
    .parse::<PathBuf>()
    .context("Invalid resource path")?;
  let full_path = root_path
    .join(&resource_path)
    .components()
    .collect::<PathBuf>();
  let mut res = ResourcesToml::default();
  let mut log = format!(
    "{}: reading resources from {full_path:?}",
    muted("INFO")
  );
  if let Err(e) = read_resources_recursive(
    root_path,
    &resource_path,
    match_tags,
    &mut res,
    &mut log,
    files,
    file_errors,
  )
  .with_context(|| {
    format!("failed to read resources from {full_path:?}")
  }) {
    file_errors.push(FileContents {
      path: resource_path.display().to_string(),
      contents: format_serror(&e.into()),
    });
    logs.push(Log::error("read remote resources", log));
  } else {
    logs.push(Log::simple("read remote resources", log));
  };
  Ok(res)
}

fn read_resources_recursive(
  root_path: &Path,
  // relative to root path.
  resource_path: &Path,
  match_tags: &[String],
  resources: &mut ResourcesToml,
  log: &mut String,
  files: &mut Vec<FileContents>,
  file_errors: &mut Vec<FileContents>,
) -> anyhow::Result<()> {
  let full_path = root_path.join(resource_path);
  let metadata = fs::metadata(&full_path)
    .context("failed to get path metadata")?;
  if metadata.is_file() {
    if !full_path
      .extension()
      .map(|ext| ext == "toml")
      .unwrap_or_default()
    {
      return Ok(());
    }
    let contents = std::fs::read_to_string(&full_path)
      .context("failed to read file contents")?;

    files.push(FileContents {
      path: resource_path.display().to_string(),
      contents: contents.clone(),
    });
    let more = toml::from_str::<ResourcesToml>(&contents)
      // the error without this comes through with multiple lines (\n) and looks bad
      .map_err(|e| anyhow!("{e:#}"))
      .context("failed to parse resource file contents")?;

    log.push('\n');
    log.push_str(&format!(
      "{}: {} from {}",
      muted("INFO"),
      colored("adding resources", Color::Green),
      colored(resource_path.display(), Color::Blue)
    ));

    extend_resources(resources, more, match_tags);

    Ok(())
  } else if metadata.is_dir() {
    let directory = fs::read_dir(&full_path).with_context(|| {
      format!("Failed to read directory contents at {full_path:?}")
    })?;
    for entry in directory.into_iter().flatten() {
      let path = entry.path();
      if let Err(e) = read_resources_recursive(
        root_path,
        &path,
        match_tags,
        resources,
        log,
        files,
        file_errors,
      )
      .with_context(|| {
        format!("failed to read resources from {path:?}")
      }) {
        file_errors.push(FileContents {
          path: resource_path.display().to_string(),
          contents: format_serror(&e.into()),
        });
        log.push('\n');
        log.push_str(&format!(
          "{}: {} from {}",
          colored("ERROR", Color::Red),
          colored("adding resources", Color::Green),
          colored(path.display(), Color::Blue)
        ));
      }
    }
    Ok(())
  } else {
    Err(anyhow!("resources path is neither file nor directory"))
  }
}

pub fn extend_resources(
  resources: &mut ResourcesToml,
  more: ResourcesToml,
  match_tags: &[String],
) {
  resources
    .servers
    .extend(filter_by_tag(more.servers, match_tags));
  resources
    .stacks
    .extend(filter_by_tag(more.stacks, match_tags));
  resources
    .deployments
    .extend(filter_by_tag(more.deployments, match_tags));
  resources
    .builds
    .extend(filter_by_tag(more.builds, match_tags));
  resources
    .repos
    .extend(filter_by_tag(more.repos, match_tags));
  resources
    .procedures
    .extend(filter_by_tag(more.procedures, match_tags));
  resources
    .alerters
    .extend(filter_by_tag(more.alerters, match_tags));
  resources
    .builders
    .extend(filter_by_tag(more.builders, match_tags));
  resources
    .server_templates
    .extend(filter_by_tag(more.server_templates, match_tags));
  resources
    .resource_syncs
    .extend(filter_by_tag(more.resource_syncs, match_tags));
  resources.user_groups.extend(more.user_groups);
  resources.variables.extend(more.variables);
}

fn filter_by_tag<T: Default>(
  resources: Vec<ResourceToml<T>>,
  match_tags: &[String],
) -> Vec<ResourceToml<T>> {
  resources
    .into_iter()
    .filter(|resource| {
      match_tags.iter().all(|tag| resource.tags.contains(tag))
    })
    .collect()
}
