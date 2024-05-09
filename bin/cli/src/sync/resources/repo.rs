use std::collections::HashMap;

use monitor_client::{
  api::{
    read::GetRepo,
    write::{CreateRepo, UpdateRepo},
  },
  entities::{
    repo::{
      PartialRepoConfig, Repo, RepoConfig, RepoConfigDiff, RepoInfo,
      RepoListItemInfo,
    },
    resource::{Resource, ResourceListItem},
    toml::ResourceToml,
    update::ResourceTarget,
  },
};
use partial_derive2::PartialDiff;

use crate::{
  maps::{id_to_server, name_to_repo},
  monitor_client,
};

use super::ResourceSync;

impl ResourceSync for Repo {
  type Config = RepoConfig;
  type Info = RepoInfo;
  type PartialConfig = PartialRepoConfig;
  type ConfigDiff = RepoConfigDiff;
  type ListItemInfo = RepoListItemInfo;

  fn display() -> &'static str {
    "repo"
  }

  fn resource_target(id: String) -> ResourceTarget {
    ResourceTarget::Repo(id)
  }

  fn name_to_resource(
  ) -> &'static HashMap<String, ResourceListItem<Self::ListItemInfo>>
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

  async fn get(
    id: String,
  ) -> anyhow::Result<Resource<Self::Config, Self::Info>> {
    monitor_client().read(GetRepo { repo: id }).await
  }

  async fn get_diff(
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
}
