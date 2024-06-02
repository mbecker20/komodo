use std::collections::HashMap;

use monitor_client::{
  api::write::{
    CreateServerTemplate, DeleteServerTemplate, UpdateServerTemplate,
  },
  entities::{
    resource::Resource,
    server_template::{
      PartialServerTemplateConfig, ServerTemplate,
      ServerTemplateConfig, ServerTemplateConfigDiff,
    },
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::PartialDiff;

use crate::{
  maps::name_to_server_template, state::monitor_client,
  sync::resource::ResourceSync,
};

impl ResourceSync for ServerTemplate {
  type Config = ServerTemplateConfig;
  type Info = ();
  type PartialConfig = PartialServerTemplateConfig;
  type ConfigDiff = ServerTemplateConfigDiff;

  fn display() -> &'static str {
    "server template"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::ServerTemplate(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, Resource<Self::Config, Self::Info>>
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

  fn get_diff(
    original: Self::Config,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::ConfigDiff> {
    Ok(original.partial_diff(update))
  }

  async fn delete(id: String) -> anyhow::Result<()> {
    monitor_client().write(DeleteServerTemplate { id }).await?;
    Ok(())
  }
}
