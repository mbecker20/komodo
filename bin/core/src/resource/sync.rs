use monitor_client::entities::{
  resource::Resource,
  sync::{
    PartialResourceSyncConfig, ResourceSync, ResourceSyncConfig,
    ResourceSyncConfigDiff, ResourceSyncInfo, ResourceSyncListItem,
    ResourceSyncQuerySpecifics,
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
    todo!()
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
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()> {
    todo!()
  }

  async fn post_create(
    created: &Resource<Self::Config, Self::Info>,
    update: &mut Update,
  ) -> anyhow::Result<()> {
    todo!()
  }

  // UPDATE

  fn update_operation() -> Operation {
    Operation::UpdateResourceSync
  }

  async fn validate_update_config(
    id: &str,
    config: &mut Self::PartialConfig,
    user: &User,
  ) -> anyhow::Result<()> {
    todo!()
  }

  async fn post_update(
    updated: &Resource<Self::Config, Self::Info>,
    update: &mut Update,
  ) -> anyhow::Result<()> {
    todo!()
  }

  // DELETE

  fn delete_operation() -> Operation {
    Operation::DeleteRepo
  }

  async fn pre_delete(
    resource: &Resource<Self::Config, Self::Info>,
    update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_delete(
    resource: &Resource<Self::Config, Self::Info>,
    update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }
}
