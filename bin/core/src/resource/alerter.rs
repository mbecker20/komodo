use monitor_client::entities::{
  alerter::{
    Alerter, AlerterConfig, AlerterConfigDiff, AlerterConfigVariant,
    AlerterInfo, AlerterListItem, AlerterListItemInfo,
    AlerterQuerySpecifics, PartialAlerterConfig,
  },
  resource::Resource,
  update::{ResourceTargetVariant, Update},
  user::User,
  MergePartial, Operation,
};
use mungos::mongodb::{
  bson::{to_document, Document},
  Collection,
};

use crate::state::db_client;

impl super::MonitorResource for Alerter {
  type Config = AlerterConfig;
  type PartialConfig = PartialAlerterConfig;
  type ConfigDiff = AlerterConfigDiff;
  type Info = AlerterInfo;
  type ListItem = AlerterListItem;
  type QuerySpecifics = AlerterQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::Alerter
  }

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>> {
    &db_client().await.alerters
  }

  async fn to_list_item(
    alerter: Resource<Self::Config, Self::Info>,
  ) -> Self::ListItem {
    let (alerter_type, enabled) = match alerter.config {
      AlerterConfig::Custom(config) => {
        (AlerterConfigVariant::Custom.to_string(), config.enabled)
      }
      AlerterConfig::Slack(config) => {
        (AlerterConfigVariant::Slack.to_string(), config.enabled)
      }
    };
    AlerterListItem {
      name: alerter.name,
      id: alerter.id,
      tags: alerter.tags,
      resource_type: ResourceTargetVariant::Alerter,
      info: AlerterListItemInfo {
        alerter_type: alerter_type.to_string(),
        is_default: alerter.info.is_default,
        enabled,
      },
    }
  }

  async fn busy(_id: &String) -> anyhow::Result<bool> {
    Ok(false)
  }

  // CREATE

  fn create_operation() -> Operation {
    Operation::CreateAlerter
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

  async fn default_info() -> anyhow::Result<Self::Info> {
    let is_default = db_client()
      .await
      .alerters
      .find_one(None, None)
      .await?
      .is_none();
    Ok(Self::Info { is_default })
  }

  // UPDATE

  fn update_operation() -> Operation {
    Operation::UpdateAlerter
  }

  async fn validate_update_config(
    _id: &str,
    _config: &mut Self::PartialConfig,
    _user: &User,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  fn update_document(
    original: Resource<Self::Config, Self::Info>,
    config: Self::PartialConfig,
  ) -> Result<Document, mungos::mongodb::bson::ser::Error> {
    let config = original.config.merge_partial(config);
    to_document(&config)
  }

  async fn post_update(
    _updated: &Self,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  // DELETE

  fn delete_operation() -> Operation {
    Operation::DeleteAlerter
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
