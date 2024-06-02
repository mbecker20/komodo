use std::collections::HashMap;

use monitor_client::{
  api::write::{CreateBuilder, DeleteBuilder, UpdateBuilder},
  entities::{
    builder::{
      Builder, BuilderConfig, BuilderConfigDiff, PartialBuilderConfig,
    },
    resource::Resource,
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::PartialDiff;

use crate::{
  maps::{id_to_server, name_to_builder},
  state::monitor_client,
  sync::resource::ResourceSync,
};

impl ResourceSync for Builder {
  type Config = BuilderConfig;
  type Info = ();
  type PartialConfig = PartialBuilderConfig;
  type ConfigDiff = BuilderConfigDiff;

  fn display() -> &'static str {
    "builder"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Builder(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, Resource<Self::Config, Self::Info>>
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

  fn get_diff(
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

  async fn delete(id: String) -> anyhow::Result<()> {
    monitor_client().write(DeleteBuilder { id }).await?;
    Ok(())
  }
}
