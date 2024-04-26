use std::collections::HashMap;

use monitor_client::{
  api::write::{CreateBuilder, UpdateBuilder},
  entities::{
    builder::{Builder, BuilderListItemInfo, PartialBuilderConfig},
    resource::ResourceListItem,
    toml::ResourceToml,
    update::ResourceTarget,
  },
};

use crate::{maps::name_to_builder, monitor_client};

use super::ResourceSync;

impl ResourceSync for Builder {
  type PartialConfig = PartialBuilderConfig;
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
}
