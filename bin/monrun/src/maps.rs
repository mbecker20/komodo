use std::{collections::HashMap, sync::OnceLock};

use anyhow::Context;
use monitor_client::{
  api::read,
  entities::{
    build::BuildListItem, deployment::DeploymentListItem,
    resource::ResourceListItem, server::ServerListItem,
  },
};

use crate::monitor_client;

pub fn names_to_ids<T>(
  names: &[String],
  map: &'static HashMap<String, ResourceListItem<T>>,
) -> anyhow::Result<Vec<&'static String>> {
  names
    .iter()
    .map(|name| {
      map
        .get(name)
        .with_context(|| format!("no item found with name {name}"))
        .map(|item| &item.id)
    })
    .collect::<anyhow::Result<_>>()
}

pub fn name_to_build() -> &'static HashMap<String, BuildListItem> {
  static NAME_TO_BUILD: OnceLock<HashMap<String, BuildListItem>> =
    OnceLock::new();
  NAME_TO_BUILD.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListBuilds::default()),
    )
    .expect("failed to get builds from monitor")
    .into_iter()
    .map(|build| (build.name.clone(), build))
    .collect()
  })
}

pub fn name_to_deployment(
) -> &'static HashMap<String, DeploymentListItem> {
  static NAME_TO_DEPLOYMENT: OnceLock<
    HashMap<String, DeploymentListItem>,
  > = OnceLock::new();
  NAME_TO_DEPLOYMENT.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListDeployments::default()),
    )
    .expect("failed to get deployments from monitor")
    .into_iter()
    .map(|deployment| (deployment.name.clone(), deployment))
    .collect()
  })
}

pub fn name_to_server() -> &'static HashMap<String, ServerListItem> {
  static NAME_TO_SERVER: OnceLock<HashMap<String, ServerListItem>> =
    OnceLock::new();
  NAME_TO_SERVER.get_or_init(|| {
    futures::executor::block_on(
      monitor_client().read(read::ListServers::default()),
    )
    .expect("failed to get servers from monitor")
    .into_iter()
    .map(|server| (server.name.clone(), server))
    .collect()
  })
}
