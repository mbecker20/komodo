use std::collections::HashMap;

use monitor_client::{
  api::{
    read::GetServerTemplate,
    write::{CreateServerTemplate, UpdateServerTemplate},
  },
  entities::{
    resource::{Resource, ResourceListItem},
    server_template::{
      PartialServerTemplateConfig, ServerTemplate,
      ServerTemplateConfig, ServerTemplateListItemInfo,
    },
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::PartialDiff;

use crate::{maps::name_to_server_template, monitor_client};

use super::ResourceSync;

impl ResourceSync for ServerTemplate {
  type PartialConfig = PartialServerTemplateConfig;
  type FullConfig = ServerTemplateConfig;
  type FullInfo = ();
  type ListItemInfo = ServerTemplateListItemInfo;

  fn display() -> &'static str {
    "server template"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::ServerTemplate(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_server_template()
  }

  async fn create(
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(CreateServerTemplate {
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
      .write(UpdateServerTemplate {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  async fn get(
    id: String,
  ) -> anyhow::Result<Resource<Self::FullConfig, Self::FullInfo>> {
    monitor_client()
      .read(GetServerTemplate {
        server_template: id,
      })
      .await
  }

  async fn minimize_update(
    original: Self::FullConfig,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::PartialConfig> {
    Ok(original.partial_diff(update).into())
  }
}
