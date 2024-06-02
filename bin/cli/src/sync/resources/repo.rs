use std::collections::HashMap;

use monitor_client::{
  api::write::{CreateRepo, DeleteRepo, UpdateRepo},
  entities::{
    repo::{
      PartialRepoConfig, Repo, RepoConfig, RepoConfigDiff, RepoInfo,
    },
    resource::Resource,
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::PartialDiff;

use crate::{
  maps::{id_to_server, name_to_repo},
  state::monitor_client,
  sync::resource::ResourceSync,
};

impl ResourceSync for Repo {
  type Config = RepoConfig;
  type Info = RepoInfo;
  type PartialConfig = PartialRepoConfig;
  type ConfigDiff = RepoConfigDiff;

  fn display() -> &'static str {
    "repo"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Repo(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, Resource<Self::Config, Self::Info>>
  {
    name_to_repo()
  }

  async fn create(
    resource: ResourceToml<Self::PartialConfig>,
  ) -> anyhow::Result<String> {
    monitor_client()
      .write(CreateRepo {
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
      .write(UpdateRepo {
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
    // Need to replace server id with name
    original.server_id = id_to_server()
      .get(&original.server_id)
      .map(|s| s.name.clone())
      .unwrap_or_default();

    Ok(original.partial_diff(update))
  }

  async fn delete(id: String) -> anyhow::Result<()> {
    monitor_client().write(DeleteRepo { id }).await?;
    Ok(())
  }
}
