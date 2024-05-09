use std::collections::HashMap;

use monitor_client::{
  api::{
    read::GetBuild,
    write::{CreateBuild, UpdateBuild},
  },
  entities::{
    build::{
      Build, BuildConfig, BuildConfigDiff, BuildInfo,
      BuildListItemInfo, PartialBuildConfig,
    },
    resource::{Resource, ResourceListItem},
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::PartialDiff;

use crate::{
  maps::{id_to_builder, name_to_build},
  monitor_client,
};

use super::ResourceSync;

impl ResourceSync for Build {
  type Config = BuildConfig;
  type Info = BuildInfo;
  type PartialConfig = PartialBuildConfig;
  type ConfigDiff = BuildConfigDiff;
  type ListItemInfo = BuildListItemInfo;

  fn display() -> &'static str {
    "build"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Build(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
  {
    name_to_build()
  }

  async fn create(
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(CreateBuild {
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
      .write(UpdateBuild {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }

  async fn get(
    id: String,
  ) -> anyhow::Result<Resource<Self::Config, Self::Info>> {
    monitor_client().read(GetBuild { build: id }).await
  }

  async fn get_diff(
    mut original: Self::Config,
    update: Self::PartialConfig,
  ) -> anyhow::Result<Self::ConfigDiff> {
    // need to replace the builder id with name
    original.builder_id = id_to_builder()
      .get(&original.builder_id)
      .map(|b| b.name.clone())
      .unwrap_or_default();

    Ok(original.partial_diff(update))
  }
}
