use std::collections::HashMap;

use monitor_client::{
  api::write::{CreateServer, UpdateServer},
  entities::{
    resource::Resource,
    server::{
      PartialServerConfig, Server, ServerConfig, ServerConfigDiff,
    },
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::PartialDiff;

use crate::{maps::name_to_server, monitor_client};

use super::ResourceSync;

impl ResourceSync for Server {
  type Config = ServerConfig;
  type Info = ();
  type PartialConfig = PartialServerConfig;
  type ConfigDiff = ServerConfigDiff;

  fn display() -> &'static str {
    "server"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Server(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, Resource<Self::Config, Self::Info>>
  {
    name_to_server()
  }

  async fn create(
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(CreateServer {
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
      .write(UpdateServer {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  async fn get_diff(
    original: Self::Config,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::ConfigDiff> {
    Ok(original.partial_diff(update))
  }
}
