use std::collections::HashMap;

use monitor_client::{
  api::write::{CreateAlerter, UpdateAlerter},
  entities::{
    alerter::{Alerter, AlerterListItemInfo, PartialAlerterConfig},
    resource::ResourceListItem,
    toml::ResourceToml,
    update::ResourceTarget,
  },
};

use crate::{maps::name_to_alerter, monitor_client};

use super::ResourceSync;

impl ResourceSync for Alerter {
  type PartialConfig = PartialAlerterConfig;
  type ListItemInfo = AlerterListItemInfo;

  fn display() -> &'static str {
    "alerter"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Alerter(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_alerter()
  }

  async fn create(
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(CreateAlerter {
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
      .write(UpdateAlerter {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }
}
