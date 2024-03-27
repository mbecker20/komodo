use std::collections::HashMap;

use monitor_client::{
  api::{
    read::ListBuilders,
    write::{CreateBuild, UpdateBuild},
  },
  entities::{
    build::{Build, BuildListItemInfo, PartialBuildConfig},
    resource::{Resource, ResourceListItem},
    update::ResourceTarget,
  },
};

use crate::{
  maps::name_to_build, monitor_client, sync::ResourceSync,
};

impl ResourceSync for Build {
  type PartialConfig = PartialBuildConfig;
  type ListItemInfo = BuildListItemInfo;
  /// Builder Name to Id
  type ExtLookup = HashMap<String, String>;

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

  async fn init_lookup_data() -> Self::ExtLookup {
    monitor_client()
      .read(ListBuilders::default())
      .await
      .expect("failed to get builders")
      .into_iter()
      .map(|b| (b.name, b.id))
      .collect::<HashMap<_, _>>()
  }

  async fn create(
    mut resource: Resource<Self::PartialConfig>,
    builder_name_to_id: &Self::ExtLookup,
  ) -> anyhow::Result<String> {
    // at this point the 'builder_id' is the name
    resource.config.builder_id = resource
      .config
      .builder_id
      .and_then(|id| builder_name_to_id.get(&id).cloned());

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
    mut resource: Resource<Self::PartialConfig>,
    builder_name_to_id: &Self::ExtLookup,
  ) -> anyhow::Result<()> {
    // at this point the 'builder_id' is the name
    resource.config.builder_id = resource
      .config
      .builder_id
      .and_then(|id| builder_name_to_id.get(&id).cloned());
    monitor_client()
      .write(UpdateBuild {
        id,
        config: resource.config,
      })
      .await?;
    Ok(())
  }
}
