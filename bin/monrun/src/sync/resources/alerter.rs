use std::collections::HashMap;

use monitor_client::{
  api::{
    read::GetAlerter,
    write::{CreateAlerter, UpdateAlerter},
  },
  entities::{
    alerter::{
      Alerter, AlerterConfig, AlerterInfo, AlerterListItemInfo,
      PartialAlerterConfig,
    },
    resource::{Resource, ResourceListItem},
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::PartialDiff;

use crate::{maps::name_to_alerter, monitor_client};

use super::ResourceSync;

impl ResourceSync for Alerter {
  type PartialConfig = PartialAlerterConfig;
  type FullConfig = AlerterConfig;
  type FullInfo = AlerterInfo;
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

  async fn get(
    id: String,
  ) -> anyhow::Result<Resource<Self::FullConfig, Self::FullInfo>> {
    monitor_client().read(GetAlerter { alerter: id }).await
  }

  async fn minimize_update(
    original: Self::FullConfig,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::PartialConfig> {
    Ok(original.partial_diff(update).into())
  }
}
