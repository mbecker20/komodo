use std::{fs, path::Path};

use anyhow::{anyhow, Context};
use formatting::{colored, format_serror, muted, Color};
use komodo_client::entities::{
  toml::ResourcesToml, update::Log, FileContents,
};

pub fn read_resources(
  path: &Path,
  logs: &mut Vec<Log>,
  files: &mut Vec<FileContents>,
  file_errors: &mut Vec<FileContents>,
) -> anyhow::Result<ResourcesToml> {
  let mut res = ResourcesToml::default();
  let mut log =
    format!("{}: reading resources from {path:?}", muted("INFO"));
  if let Err(e) = read_resources_recursive(
    path,
    &mut res,
    &mut log,
    files,
    file_errors,
  )
  .with_context(|| format!("failed to read resources from {path:?}"))
  {
    file_errors.push(FileContents {
      path: path.display().to_string(),
      contents: format_serror(&e.into()),
    });
    logs.push(Log::error("read remote resources", log));
  } else {
    logs.push(Log::simple("read remote resources", log));
  };
  Ok(res)
}

fn read_resources_recursive(
  path: &Path,
  resources: &mut ResourcesToml,
  log: &mut String,
  files: &mut Vec<FileContents>,
  file_errors: &mut Vec<FileContents>,
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
    let contents = std::fs::read_to_string(path)
      .context("failed to read file contents")?;
    files.push(FileContents {
      path: path.display().to_string(),
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
      colored(path.display(), Color::Blue)
    ));

    resources.servers.extend(more.servers);
    resources.deployments.extend(more.deployments);
    resources.stacks.extend(more.stacks);
    resources.builds.extend(more.builds);
    resources.repos.extend(more.repos);
    resources.procedures.extend(more.procedures);
    resources.alerters.extend(more.alerters);
    resources.builders.extend(more.builders);
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
      if let Err(e) = read_resources_recursive(
        &path,
        resources,
        log,
        files,
        file_errors,
      )
      .with_context(|| {
        format!("failed to read resources from {path:?}")
      }) {
        file_errors.push(FileContents {
          path: path.display().to_string(),
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
