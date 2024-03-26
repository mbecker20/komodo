use std::collections::HashMap;

use async_trait::async_trait;
use monitor_client::{
  api::write::{CreateRepo, UpdateRepo},
  entities::{
    repo::{PartialRepoConfig, Repo, RepoInfo},
    resource::{Resource, ResourceListItem},
    update::ResourceTarget,
  },
};

use crate::{maps::name_to_repo, monitor_client, sync::ResourceSync};

#[async_trait]
impl ResourceSync for Repo {
  type PartialConfig = PartialRepoConfig;
  type ListItemInfo = RepoInfo;
  type ExtLookup = ();

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

  async fn init_lookup_data() -> Self::ExtLookup {
    ()
  }

  async fn create(
    resource: Resource<Self::PartialConfig>,
    _: &(),
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
    resource: Resource<Self::PartialConfig>,
    _: &(),
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
