use std::collections::HashMap;

use monitor_client::{
  api::write::{CreateBuild, DeleteBuild, UpdateBuild},
  entities::{
    build::{
      Build, BuildConfig, BuildConfigDiff, BuildInfo,
      PartialBuildConfig,
    },
    resource::Resource,
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::PartialDiff;

use crate::{
  maps::{id_to_builder, name_to_build},
  state::monitor_client,
  sync::resource::ResourceSync,
};

impl ResourceSync for Build {
  type Config = BuildConfig;
  type Info = BuildInfo;
  type PartialConfig = PartialBuildConfig;
  type ConfigDiff = BuildConfigDiff;

  fn display() -> &'static str {
    "build"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Build(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, Resource<Self::Config, Self::Info>>
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

  fn get_diff(
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

  async fn delete(id: String) -> anyhow::Result<()> {
    monitor_client().write(DeleteBuild { id }).await?;
    Ok(())
  }
}
