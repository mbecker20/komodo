use monitor_client::entities::{
  resource::Resource,
  sync::{
    PartialResourceSyncConfig, ResourceSync, ResourceSyncConfig,
    ResourceSyncConfigDiff, ResourceSyncInfo, ResourceSyncListItem,
    ResourceSyncListItemInfo, ResourceSyncQuerySpecifics,
    ResourceSyncState,
  },
  update::{ResourceTargetVariant, Update},
  user::User,
  Operation,
};
use mungos::mongodb::Collection;

use crate::state::{action_states, db_client};

impl super::MonitorResource for ResourceSync {
  type Config = ResourceSyncConfig;
  type PartialConfig = PartialResourceSyncConfig;
  type ConfigDiff = ResourceSyncConfigDiff;
  type Info = ResourceSyncInfo;
  type ListItem = ResourceSyncListItem;
  type QuerySpecifics = ResourceSyncQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::ResourceSync
  }

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>> {
    &db_client().await.resource_syncs
  }

  async fn to_list_item(
    resource_sync: Resource<Self::Config, Self::Info>,
  ) -> Self::ListItem {
    ResourceSyncListItem {
      id: resource_sync.id,
      name: resource_sync.name,
      tags: resource_sync.tags,
      resource_type: ResourceTargetVariant::ResourceSync,
      info: ResourceSyncListItemInfo {
        repo: resource_sync.config.repo,
        branch: resource_sync.config.branch,
        last_sync_ts: 0,
        last_sync_hash: String::new(),
        last_sync_message: String::new(),
        state: ResourceSyncState::Unknown,
      },
    }
  }

  async fn busy(id: &String) -> anyhow::Result<bool> {
    action_states()
      .resource_sync
      .get(id)
      .await
      .unwrap_or_default()
      .busy()
  }

  // CREATE

  fn create_operation() -> Operation {
    Operation::CreateResourceSync
  }

  fn user_can_create(user: &User) -> bool {
    user.admin
  }

  async fn validate_create_config(
    _config: &mut Self::PartialConfig,
    _user: &User,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_create(
    _created: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  // UPDATE

  fn update_operation() -> Operation {
    Operation::UpdateResourceSync
  }

  async fn validate_update_config(
    _id: &str,
    _config: &mut Self::PartialConfig,
    _user: &User,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_update(
    _updated: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  // DELETE

  fn delete_operation() -> Operation {
    Operation::DeleteRepo
  }

  async fn pre_delete(
    _resource: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_delete(
    _resource: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }
}
