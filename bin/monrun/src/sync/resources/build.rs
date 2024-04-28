use std::collections::HashMap;

use monitor_client::{
  api::write::{CreateBuild, UpdateBuild},
  entities::{
    build::{
      Build, BuildConfig, BuildListItemInfo, PartialBuildConfig,
    },
    resource::ResourceListItem,
    toml::ResourceToml,
    update::ResourceTarget,
  },
};

use crate::{maps::name_to_build, monitor_client};

use super::ResourceSync;

impl ResourceSync for Build {
  type PartialConfig = PartialBuildConfig;
  type FullConfig = BuildConfig;
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
}
