use std::collections::HashMap;

use anyhow::Context;
use monitor_client::entities::stack::StackServiceNames;
use serde::Deserialize;

pub fn extract_services(
  json_compose: &str,
) -> anyhow::Result<Vec<StackServiceNames>> {
  serde_json::from_str::<ComposeFile>(json_compose)
    .context("failed to parse compose json")
    .map(|file| {
      file
        .services
        .into_iter()
        .map(|(service_name, ComposeService { container_name })| {
          StackServiceNames {
            container_name: container_name
              .unwrap_or_else(|| service_name.clone()),
            service_name,
          }
        })
        .collect()
    })
}

#[derive(Deserialize)]
pub struct ComposeFile {
  pub services: HashMap<String, ComposeService>,
}

#[derive(Deserialize)]
pub struct ComposeService {
  pub container_name: Option<String>,
}
