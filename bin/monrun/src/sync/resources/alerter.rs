use std::collections::HashMap;

use async_trait::async_trait;
use monitor_client::{
  api::write::{CreateAlerter, UpdateAlerter},
  entities::{
    alerter::{Alerter, AlerterListItemInfo, PartialAlerterConfig},
    resource::{Resource, ResourceListItem},
    update::ResourceTarget,
  },
};

use crate::{
  maps::name_to_alerter, monitor_client, sync::ResourceSync,
};

#[async_trait]
impl ResourceSync for Alerter {
  type PartialConfig = PartialAlerterConfig;
  type ListItemInfo = AlerterListItemInfo;
  type ExtLookup = ();

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

  async fn init_lookup_data() -> Self::ExtLookup {
    ()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
    _: &(),
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
    resource: Resource<Self::PartialConfig>,
    _: &(),
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
