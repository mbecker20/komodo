use anyhow::Context;
use monitor_client::entities::stack::{
  ComposeFile, ComposeService, Stack, StackServiceNames,
};

use crate::helpers::stack::remote::get_remote_compose_file;

/// Passing fresh will re-extract services from compose file, whether local or remote (repo)
pub async fn extract_services_from_stack(
  stack: &Stack,
  fresh: bool,
) -> anyhow::Result<Vec<StackServiceNames>> {
  if !fresh {
    if stack.info.deployed_services.is_empty() {
      return Ok(stack.info.latest_services.clone());
    } else {
      return Ok(stack.info.deployed_services.clone());
    }
  }

  let compose_contents = if stack.config.file_contents.is_empty() {
    let (res, _, _, _) =
      get_remote_compose_file(stack).await.context(
        "failed to get remote compose file to extract services",
      )?;
    res.context("failed to read remote compose file")?
  } else {
    stack.config.file_contents.clone()
  };

  extract_services(&stack.project_name(true), &compose_contents)
}

pub fn extract_services(
  project_name: &str,
  compose_contents: &str,
) -> anyhow::Result<Vec<StackServiceNames>> {
  let compose = serde_yaml::from_str::<ComposeFile>(compose_contents)
    .context("failed to parse service names from compose contents")?;

  let services = compose
    .services
    .into_iter()
    .map(|(service_name, ComposeService { container_name, .. })| {
      StackServiceNames {
        container_name: container_name.unwrap_or_else(|| {
          format!("{project_name}-{service_name}")
        }),
        service_name,
      }
    })
    .collect();

  Ok(services)
}
