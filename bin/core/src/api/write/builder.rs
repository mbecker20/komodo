use anyhow::Context;
use async_trait::async_trait;
use monitor_client::{
  api::write::*,
  entities::{
    builder::Builder,
    monitor_timestamp,
    update::{Log, ResourceTarget, Update},
    Operation, PermissionLevel,
  },
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  mongodb::bson::{doc, to_bson},
};
use resolver_api::Resolve;

use crate::{
  auth::RequestUser,
  db::db_client,
  helpers::{
    add_update, remove_from_recently_viewed, resource::StateResource,
  },
  state::State,
};

#[async_trait]
impl Resolve<CreateBuilder, RequestUser> for State {
  async fn resolve(
    &self,
    CreateBuilder { name, config }: CreateBuilder,
    user: RequestUser,
  ) -> anyhow::Result<Builder> {
    let start_ts = monitor_timestamp();
    let builder = Builder {
      id: Default::default(),
      name,
      updated_at: start_ts,
      permissions: [(user.id.clone(), PermissionLevel::Update)]
        .into_iter()
        .collect(),
      description: Default::default(),
      tags: Default::default(),
      config: config.into(),
      info: Default::default(),
    };
    let builder_id = db_client()
      .builders
      .insert_one(builder, None)
      .await
      .context("failed to add builder to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let builder: Builder = self.get_resource(&builder_id).await?;
    let update = Update {
      target: ResourceTarget::Builder(builder_id),
      operation: Operation::CreateBuilder,
      start_ts,
      end_ts: Some(monitor_timestamp()),
      operator: user.id.clone(),
      success: true,
      logs: vec![
        Log::simple(
          "create builder",
          format!(
            "created builder\nid: {}\nname: {}",
            builder.id, builder.name
          ),
        ),
        Log::simple("config", format!("{:#?}", builder.config)),
      ],
      ..Default::default()
    };

    add_update(update).await?;

    Ok(builder)
  }
}

#[async_trait]
impl Resolve<CopyBuilder, RequestUser> for State {
  async fn resolve(
    &self,
    CopyBuilder { name, id }: CopyBuilder,
    user: RequestUser,
  ) -> anyhow::Result<Builder> {
    let Builder {
      config,
      description,
      ..
    } = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;
    let start_ts = monitor_timestamp();
    let builder = Builder {
      id: Default::default(),
      name,
      updated_at: start_ts,
      permissions: [(user.id.clone(), PermissionLevel::Update)]
        .into_iter()
        .collect(),
      description,
      tags: Default::default(),
      config,
      info: (),
    };
    let builder_id = db_client()
      .builders
      .insert_one(builder, None)
      .await
      .context("failed to add builder to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let builder: Builder = self.get_resource(&builder_id).await?;
    let update = Update {
      target: ResourceTarget::Builder(builder_id),
      operation: Operation::CreateBuilder,
      start_ts,
      end_ts: Some(monitor_timestamp()),
      operator: user.id.clone(),
      success: true,
      logs: vec![
        Log::simple(
          "create builder",
          format!(
            "created builder\nid: {}\nname: {}",
            builder.id, builder.name
          ),
        ),
        Log::simple("config", format!("{:#?}", builder.config)),
      ],
      ..Default::default()
    };

    add_update(update).await?;

    Ok(builder)
  }
}

#[async_trait]
impl Resolve<DeleteBuilder, RequestUser> for State {
  async fn resolve(
    &self,
    DeleteBuilder { id }: DeleteBuilder,
    user: RequestUser,
  ) -> anyhow::Result<Builder> {
    let builder: Builder = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    let start_ts = monitor_timestamp();

    db_client()
      .builds
      .update_many(
        doc! { "config.builder.params.builder_id": &id },
        mungos::update::Update::Set(
          doc! { "config.builder.params.builder_id": "" },
        ),
        None,
      )
      .await?;

    delete_one_by_id(&db_client().builders, &id, None)
      .await
      .context("failed to delete builder from database")?;

    let mut update = Update {
      target: (&builder).into(),
      operation: Operation::DeleteBuilder,
      start_ts,
      operator: user.id.clone(),
      logs: vec![Log::simple(
        "delete builder",
        format!("deleted builder {}", builder.name),
      )],
      ..Default::default()
    };

    update.finalize();
    add_update(update).await?;

    remove_from_recently_viewed(&builder).await?;

    Ok(builder)
  }
}

#[async_trait]
impl Resolve<UpdateBuilder, RequestUser> for State {
  async fn resolve(
    &self,
    UpdateBuilder { id, config }: UpdateBuilder,
    user: RequestUser,
  ) -> anyhow::Result<Builder> {
    let builder: Builder = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    let mut update = Update {
      target: ResourceTarget::Builder(id.clone()),
      operation: Operation::UpdateBuilder,
      start_ts: monitor_timestamp(),
      logs: vec![Log::simple(
        "builder update",
        serde_json::to_string_pretty(&config).unwrap(),
      )],
      operator: user.id.clone(),
      ..Default::default()
    };

    let config = builder.config.merge_partial(config);

    update_one_by_id(
      &db_client().builders,
      &id,
      mungos::update::Update::FlattenSet(
        doc! { "config": to_bson(&config)? },
      ),
      None,
    )
    .await?;

    let builder: Builder = self.get_resource(&id).await?;

    update.finalize();
    add_update(update).await?;

    Ok(builder)
  }
}
