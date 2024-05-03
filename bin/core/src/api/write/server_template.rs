use std::str::FromStr;

use anyhow::{anyhow, Context};
use axum::async_trait;
use monitor_client::{
  api::write::{
    CopyServerTemplate, CreateServerTemplate, DeleteServerTemplate,
    UpdateServerTemplate,
  },
  entities::{
    monitor_timestamp, permission::PermissionLevel,
    server_template::ServerTemplate, user::User, Operation,
  },
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  mongodb::bson::{doc, oid::ObjectId, to_document},
};
use resolver_api::Resolve;

use crate::{
  helpers::{
    create_permission, remove_from_recently_viewed,
    resource::{delete_all_permissions_on_resource, StateResource},
    update::{add_update, make_update},
  },
  state::{db_client, State},
};

#[async_trait]
impl Resolve<CreateServerTemplate, User> for State {
  async fn resolve(
    &self,
    CreateServerTemplate { name, config }: CreateServerTemplate,
    user: User,
  ) -> anyhow::Result<ServerTemplate> {
    if !user.admin {
      return Err(anyhow!("only admins can create server templates"));
    }
    if ObjectId::from_str(&name).is_ok() {
      return Err(anyhow!("valid ObjectIds cannot be used as names"));
    }
    let server_template = ServerTemplate {
      id: Default::default(),
      name,
      updated_at: monitor_timestamp(),
      description: Default::default(),
      tags: Default::default(),
      config: config.into(),
      info: (),
    };
    let server_template_id = db_client()
      .await
      .server_templates
      .insert_one(server_template, None)
      .await
      .context("failed to add server_template to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let server_template =
      ServerTemplate::get_resource(&server_template_id).await?;
    create_permission(
      &user,
      &server_template,
      PermissionLevel::Write,
    )
    .await;
    let mut update = make_update(
      &server_template,
      Operation::CreateServerTemplate,
      &user,
    );
    update.push_simple_log(
      "create server template",
      format!(
        "created server template\nid: {}\nname: {}",
        server_template.id, server_template.name
      ),
    );
    update.push_simple_log(
      "config",
      format!("{:#?}", server_template.config),
    );
    update.finalize();

    add_update(update).await?;

    Ok(server_template)
  }
}

#[async_trait]
impl Resolve<CopyServerTemplate, User> for State {
  async fn resolve(
    &self,
    CopyServerTemplate { name, id }: CopyServerTemplate,
    user: User,
  ) -> anyhow::Result<ServerTemplate> {
    let ServerTemplate {
      config,
      description,
      ..
    } = ServerTemplate::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;
    let server_template = ServerTemplate {
      id: Default::default(),
      name,
      updated_at: monitor_timestamp(),
      description,
      tags: Default::default(),
      config,
      info: (),
    };
    let server_template_id = db_client()
      .await
      .server_templates
      .insert_one(server_template, None)
      .await
      .context("failed to add server_template to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let server_template =
      ServerTemplate::get_resource(&server_template_id).await?;
    create_permission(
      &user,
      &server_template,
      PermissionLevel::Write,
    )
    .await;
    let mut update = make_update(
      &server_template,
      Operation::CreateServerTemplate,
      &user,
    );
    update.push_simple_log(
      "create server template",
      format!(
        "created server template\nid: {}\nname: {}",
        server_template.id, server_template.name
      ),
    );
    update.push_simple_log(
      "config",
      format!("{:#?}", server_template.config),
    );
    update.finalize();

    add_update(update).await?;

    Ok(server_template)
  }
}

#[async_trait]
impl Resolve<DeleteServerTemplate, User> for State {
  async fn resolve(
    &self,
    DeleteServerTemplate { id }: DeleteServerTemplate,
    user: User,
  ) -> anyhow::Result<ServerTemplate> {
    let server_template =
      ServerTemplate::get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Write,
      )
      .await?;

    delete_one_by_id(&db_client().await.server_templates, &id, None)
      .await
      .context("failed to delete server templates from database")?;

    delete_all_permissions_on_resource(&server_template).await;

    let mut update = make_update(
      &server_template,
      Operation::DeleteServerTemplate,
      &user,
    );

    update.push_simple_log(
      "delete server template",
      format!("deleted server template {}", server_template.name),
    );

    update.finalize();
    add_update(update).await?;

    remove_from_recently_viewed(&server_template).await?;

    Ok(server_template)
  }
}

#[async_trait]
impl Resolve<UpdateServerTemplate, User> for State {
  async fn resolve(
    &self,
    UpdateServerTemplate { id, config }: UpdateServerTemplate,
    user: User,
  ) -> anyhow::Result<ServerTemplate> {
    let server_template =
      ServerTemplate::get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Write,
      )
      .await?;

    let mut update = make_update(
      &server_template,
      Operation::UpdateServerTemplate,
      &user,
    );

    update.push_simple_log(
      "server template update",
      serde_json::to_string_pretty(&config)
        .context("failed to serialize config update")?,
    );

    let config = server_template.config.merge_partial(config);
    let config = to_document(&config)
      .context("failed to serialize update to bson document")?;

    update_one_by_id(
      &db_client().await.server_templates,
      &id,
      mungos::update::Update::FlattenSet(doc! { "config": config }),
      None,
    )
    .await?;

    let server_template = ServerTemplate::get_resource(&id).await?;

    update.finalize();
    add_update(update).await?;

    Ok(server_template)
  }
}
