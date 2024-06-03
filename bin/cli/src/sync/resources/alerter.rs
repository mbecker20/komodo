use partial_derive2::PartialDiff;
use std::collections::HashMap;

use monitor_client::{
  api::write::{CreateAlerter, DeleteAlerter, UpdateAlerter},
  entities::{
    alerter::{
      Alerter, AlerterConfig, AlerterConfigDiff, PartialAlerterConfig,
    },
    resource::Resource,
    toml::ResourceToml,
    update::ResourceTarget,
  },
};

use crate::{
  maps::name_to_alerter, state::monitor_client,
  sync::resource::ResourceSync,
};

impl ResourceSync for Alerter {
  type Config = AlerterConfig;
  type Info = ();
  type PartialConfig = PartialAlerterConfig;
  type ConfigDiff = AlerterConfigDiff;

  fn display() -> &'static str {
    "alerter"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Alerter(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, Resource<Self::Config, Self::Info>>
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

  fn get_diff(
    original: Self::Config,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::ConfigDiff> {
    Ok(original.partial_diff(update))
  }

  async fn delete(id: String) -> anyhow::Result<()> {
    monitor_client().write(DeleteAlerter { id }).await?;
    Ok(())
  }
}
