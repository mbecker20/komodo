use std::path::PathBuf;

use anyhow::Context;
use monitor_client::entities::stack::{
  ComposeFile, ComposeService, StackServiceNames,
};

pub fn extract_services(
  compose_contents: &str,
  run_directory: &str,
  file_path: &str,
) -> anyhow::Result<Vec<StackServiceNames>> {
  let compose = serde_yaml::from_str::<ComposeFile>(compose_contents)
    .context("failed to parse service names from compose contents")?;

  let run_directory: PathBuf = run_directory
    .parse()
    .context("run directory is not valid path")?;
  let file = run_directory.join(file_path);

  let compose_name = match compose.name {
    Some(name) => name,
    None => {
      file
        .parent()
        .with_context(|| format!("cannot get compose file parent for default compose name | path: {file:?}"))?
        .file_name()
        .with_context(|| format!("cannot get compose file parent name for default compose name | path: {file:?}"))?
        .to_string_lossy()
        .to_string()
    }
  };

  let services = compose
    .services
    .into_iter()
    .map(|(service_name, ComposeService { container_name, .. })| {
      StackServiceNames {
        container_name: container_name.unwrap_or_else(|| {
          format!("{compose_name}-{service_name}")
        }),
        service_name,
      }
    })
    .collect();

  Ok(services)
}
