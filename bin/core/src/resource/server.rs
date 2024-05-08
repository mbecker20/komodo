use anyhow::Context;
use monitor_client::entities::{
  monitor_timestamp,
  resource::Resource,
  server::{
    PartialServerConfig, Server, ServerConfig, ServerConfigDiff,
    ServerListItem, ServerListItemInfo, ServerQuerySpecifics,
  },
  update::{ResourceTargetVariant, Update},
  user::User,
  Operation,
};
use mungos::mongodb::{bson::doc, Collection};

use crate::{
  monitor::update_cache_for_server,
  state::{action_states, db_client, server_status_cache},
};

impl super::MonitorResource for Server {
  type Config = ServerConfig;
  type PartialConfig = PartialServerConfig;
  type ConfigDiff = ServerConfigDiff;
  type Info = ();
  type ListItem = ServerListItem;
  type QuerySpecifics = ServerQuerySpecifics;

  fn resource_type() -> ResourceTargetVariant {
    ResourceTargetVariant::Server
  }

  async fn coll(
  ) -> &'static Collection<Resource<Self::Config, Self::Info>> {
    &db_client().await.servers
  }

  async fn to_list_item(
    server: Resource<Self::Config, Self::Info>,
  ) -> anyhow::Result<Self::ListItem> {
    let status = server_status_cache().get(&server.id).await;
    Ok(ServerListItem {
      name: server.name,
      id: server.id,
      tags: server.tags,
      resource_type: ResourceTargetVariant::Server,
      info: ServerListItemInfo {
        status: status.map(|s| s.status).unwrap_or_default(),
        region: server.config.region,
        send_unreachable_alerts: server
          .config
          .send_unreachable_alerts,
        send_cpu_alerts: server.config.send_cpu_alerts,
        send_mem_alerts: server.config.send_mem_alerts,
        send_disk_alerts: server.config.send_disk_alerts,
      },
    })
  }

  async fn busy(id: &String) -> anyhow::Result<bool> {
    action_states()
      .server
      .get(id)
      .await
      .unwrap_or_default()
      .busy()
  }

  // CREATE

  fn create_operation() -> Operation {
    Operation::CreateServer
  }

  fn user_can_create(user: &User) -> bool {
    user.admin || user.create_server_permissions
  }

  async fn validate_create_config(
    _config: &mut Self::PartialConfig,
    _user: &User,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_create(
    created: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    update_cache_for_server(created).await;
    Ok(())
  }

  // UPDATE

  fn update_operation() -> Operation {
    Operation::UpdateServer
  }

  async fn validate_update_config(
    _id: &str,
    _config: &mut Self::PartialConfig,
    _user: &User,
  ) -> anyhow::Result<()> {
    Ok(())
  }

  async fn post_update(
    updated: &Self,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    update_cache_for_server(updated).await;
    Ok(())
  }

  // DELETE

  fn delete_operation() -> Operation {
    Operation::DeleteServer
  }

  async fn pre_delete(
    resource: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    let db = db_client().await;

    let id = &resource.id;

    db.builders
      .update_many(
        doc! { "config.params.server_id": &id },
        doc! { "$set": { "config.params.server_id": "" } },
        None,
      )
      .await
      .context("failed to detach server from builders")?;

    db.deployments
      .update_many(
        doc! { "config.server_id": &id },
        doc! { "$set": { "config.server_id": "" } },
        None,
      )
      .await
      .context("failed to detach server from deployments")?;

    db.repos
      .update_many(
        doc! { "config.server_id": &id },
        doc! { "$set": { "config.server_id": "" } },
        None,
      )
      .await
      .context("failed to detach server from repos")?;

    db.alerts
      .update_many(
        doc! { "target.type": "Server", "target.id": &id },
        doc! { "$set": {
          "resolved": true,
          "resolved_ts": monitor_timestamp()
        } },
        None,
      )
      .await
      .context("failed to detach server from repos")?;

    Ok(())
  }

  async fn post_delete(
    resource: &Resource<Self::Config, Self::Info>,
    _update: &mut Update,
  ) -> anyhow::Result<()> {
    server_status_cache().remove(&resource.id).await;
    Ok(())
  }
}
