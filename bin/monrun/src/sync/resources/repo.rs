use std::collections::HashMap;

use monitor_client::{
  api::write::{CreateRepo, UpdateRepo},
  entities::{
    repo::{PartialRepoConfig, Repo, RepoConfig, RepoListItemInfo},
    resource::ResourceListItem,
    toml::ResourceToml,
    update::ResourceTarget,
  },
};

use crate::{maps::name_to_repo, monitor_client};

use super::ResourceSync;

impl ResourceSync for Repo {
  type PartialConfig = PartialRepoConfig;
  type FullConfig = RepoConfig;
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
}
