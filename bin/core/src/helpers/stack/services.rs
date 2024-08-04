use std::collections::HashMap;

use anyhow::Context;
use monitor_client::entities::stack::StackServiceNames;
use serde::Deserialize;

pub fn extract_services(
  compose_contents: &str,
) -> anyhow::Result<Vec<StackServiceNames>> {
  serde_yaml::from_str::<ComposeFile>(compose_contents)
    .context("failed to parse service names from compose contents")
    .map(|file| {
      file
        .services
        .into_iter()
        .map(|(service_name, ComposeService { container_name })| {
          StackServiceNames {
            service_name,
            container_name,
          }
        })
        .collect()
    })
}

#[derive(Deserialize)]
pub struct ComposeFile {
  #[serde(default)]
  pub services: HashMap<String, ComposeService>,
}

#[derive(Deserialize)]
pub struct ComposeService {
  pub container_name: Option<String>,
}
