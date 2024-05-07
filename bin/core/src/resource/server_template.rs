use std::str::FromStr;

use monitor_client::entities::{
  resource::Resource,
  server_template::{
    PartialServerTemplateConfig, ServerTemplate,
    ServerTemplateConfig, ServerTemplateConfigVariant,
    ServerTemplateListItem, ServerTemplateListItemInfo,
    ServerTemplateQuerySpecifics,
  },
  update::{ResourceTargetVariant, Update},
  user::User,
  Operation,
};
use mungos::mongodb::{bson::oid::ObjectId, Collection};

use crate::state::db_client;

impl super::MonitorResource for ServerTemplate {
  type Config = ServerTemplateConfig;
  type PartialConfig = PartialServerTemplateConfig;
  type Info = ();
  type ListItem = ServerTemplateListItem;
  type QuerySpecifics = ServerTemplateQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::ServerTemplate
  }

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>> {
    &db_client().await.server_templates
  }

  async fn to_list_item(
    server_template: Resource<Self::Config, Self::Info>,
  ) -> anyhow::Result<Self::ListItem> {
    let (template_type, instance_type) = match server_template.config
    {
      ServerTemplateConfig::Aws(config) => (
        ServerTemplateConfigVariant::Aws.to_string(),
        Some(config.instance_type),
      ),
    };
    Ok(ServerTemplateListItem {
      name: server_template.name,
      created_at: ObjectId::from_str(&server_template.id)?
        .timestamp()
        .timestamp_millis(),
      id: server_template.id,
      tags: server_template.tags,
      resource_type: ResourceTargetVariant::ServerTemplate,
      info: ServerTemplateListItemInfo {
        provider: template_type.to_string(),
        instance_type,
      },
    })
  }

  async fn busy(_id: &String) -> anyhow::Result<bool> {
    Ok(false)
  }

  // CREATE

  fn create_operation() -> Operation {
    Operation::CreateServerTemplate
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
    Operation::UpdateServerTemplate
  }

  async fn validate_update_config(
    _original: Resource<Self::Config, Self::Info>,
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
    Operation::DeleteServerTemplate
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
