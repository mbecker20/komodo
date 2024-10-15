use komodo_client::entities::{
  action::{
    Action, ActionConfig, ActionConfigDiff, ActionInfo,
    ActionListItem, ActionListItemInfo, ActionQuerySpecifics,
    ActionState, PartialActionConfig,
  },
  resource::Resource,
  update::Update,
  user::User,
  Operation, ResourceTargetVariant,
};
use mungos::mongodb::Collection;

use crate::state::{action_states, db_client};

impl super::KomodoResource for Action {
  type Config = ActionConfig;
  type PartialConfig = PartialActionConfig;
  type ConfigDiff = ActionConfigDiff;
  type Info = ActionInfo;
  type ListItem = ActionListItem;
  type QuerySpecifics = ActionQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::Action
  }

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>> {
    &db_client().actions
  }

  async fn to_list_item(
    action: Resource<Self::Config, Self::Info>,
  ) -> Self::ListItem {
    // let status = action_status_cache().get(&action.id).await;
    ActionListItem {
      name: action.name,
      id: action.id,
      tags: action.tags,
      resource_type: ResourceTargetVariant::Action,
      info: ActionListItemInfo {
        // state: status.map(|s| s.state).unwrap_or_default(),
        state: ActionState::Unknown,
        last_run_at: action.info.last_run_at,
      },
    }
  }

  async fn busy(id: &String) -> anyhow::Result<bool> {
    action_states()
      .action
      .get(id)
      .await
      .unwrap_or_default()
      .busy()
  }

  // CREATE

  fn create_operation() -> Operation {
    Operation::CreateAction
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
    Operation::UpdateAction
  }

  async fn validate_update_config(
    _id: &str,
    _config: &mut Self::PartialConfig,
    _user: &User,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_update(
    _updated: &Self,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  // DELETE

  fn delete_operation() -> Operation {
    Operation::DeleteAction
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
