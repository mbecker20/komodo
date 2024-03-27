use std::collections::HashMap;

use monitor_client::{
  api::write::{CreateServer, UpdateServer},
  entities::{
    resource::{Resource, ResourceListItem},
    server::{PartialServerConfig, Server, ServerListItemInfo},
    update::ResourceTarget,
  },
};

use crate::{
  maps::name_to_server, monitor_client, sync::ResourceSync,
};

impl ResourceSync for Server {
  type ListItemInfo = ServerListItemInfo;
  type PartialConfig = PartialServerConfig;

  fn display() -> &'static str {
    "server"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Server(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_server()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
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
    resource: Resource<Self::PartialConfig>,
  ) -> anyhow::Result<()> {
    monitor_client()
      .write(UpdateServer {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }
}
