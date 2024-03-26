use std::collections::HashMap;

use async_trait::async_trait;
use monitor_client::{
  api::write::{CreateBuilder, UpdateBuilder},
  entities::{
    builder::{Builder, BuilderListItemInfo, PartialBuilderConfig},
    resource::{Resource, ResourceListItem},
    update::ResourceTarget,
  },
};

use crate::{
  maps::name_to_builder, monitor_client, sync::ResourceSync,
};

#[async_trait]
impl ResourceSync for Builder {
  type PartialConfig = PartialBuilderConfig;
  type ListItemInfo = BuilderListItemInfo;
  type ExtLookup = ();

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

  async fn init_lookup_data() -> Self::ExtLookup {
    ()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
    _: &(),
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
    resource: Resource<Self::PartialConfig>,
    _: &(),
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
