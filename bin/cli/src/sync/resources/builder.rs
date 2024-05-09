use std::collections::HashMap;

use monitor_client::{
  api::{
    read::GetBuilder,
    write::{CreateBuilder, UpdateBuilder},
  },
  entities::{
    builder::{
      Builder, BuilderConfig, BuilderConfigDiff, BuilderListItemInfo,
      PartialBuilderConfig,
    },
    resource::{Resource, ResourceListItem},
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::PartialDiff;

use crate::{
  maps::{id_to_server, name_to_builder},
  monitor_client,
};

use super::ResourceSync;

impl ResourceSync for Builder {
  type Config = BuilderConfig;
  type Info = ();
  type PartialConfig = PartialBuilderConfig;
  type ConfigDiff = BuilderConfigDiff;
  type ListItemInfo = BuilderListItemInfo;

  fn display() -> &'static str {
    "builder"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Builder(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_builder()
  }

  async fn create(
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(CreateBuilder {
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
      .write(UpdateBuilder {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  async fn get(
    id: String,
  ) -> anyhow::Result<Resource<Self::Config, Self::Info>> {
    monitor_client().read(GetBuilder { builder: id }).await
  }

  async fn get_diff(
    mut original: Self::Config,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::ConfigDiff> {
    // need to replace server builder id with name
    if let BuilderConfig::Server(config) = &mut original {
      config.server_id = id_to_server()
        .get(&config.server_id)
        .map(|s| s.name.clone())
        .unwrap_or_default();
    }

    Ok(original.partial_diff(update))
  }
}
