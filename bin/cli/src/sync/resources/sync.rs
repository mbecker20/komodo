use std::collections::HashMap;

use monitor_client::{
  api::write::{
    CreateResourceSync, DeleteResourceSync, UpdateResourceSync,
  },
  entities::{
    self,
    resource::Resource,
    sync::{
      PartialResourceSyncConfig, ResourceSyncConfig,
      ResourceSyncConfigDiff, ResourceSyncInfo,
    },
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::PartialDiff;

use crate::{
  maps::name_to_resource_sync, state::monitor_client,
  sync::resource::ResourceSync,
};

impl ResourceSync for entities::sync::ResourceSync {
  type Config = ResourceSyncConfig;
  type Info = ResourceSyncInfo;
  type PartialConfig = PartialResourceSyncConfig;
  type ConfigDiff = ResourceSyncConfigDiff;

  fn display() -> &'static str {
    "resource sync"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::ResourceSync(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, Resource<Self::Config, Self::Info>>
  {
    name_to_resource_sync()
  }

  async fn create(
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(CreateResourceSync {
        name: resource.name,
        config: resource.config,
      })
      .await
      .map(|res| res.id)
  }

  async fn update(
    id: String,
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(UpdateResourceSync {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  fn get_diff(
    original: Self::Config,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::ConfigDiff> {
    Ok(original.partial_diff(update))
  }

  async fn delete(id: String) -> anyhow::Result<()> {
    monitor_client().write(DeleteResourceSync { id }).await?;
    Ok(())
  }
}
