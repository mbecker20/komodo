use std::str::FromStr;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::write::*,
  entities::{
    builder::{
      Builder, PartialBuilderConfig, PartialServerBuilderConfig,
    },
    monitor_timestamp,
    permission::PermissionLevel,
    server::Server,
    update::{Log, ResourceTarget, Update},
    user::User,
    Operation,
  },
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  mongodb::bson::{doc, oid::ObjectId, to_bson},
};
use resolver_api::Resolve;

use crate::{
  db::db_client,
  helpers::{
    add_update, create_permission, make_update,
    remove_from_recently_viewed,
    resource::{delete_all_permissions_on_resource, StateResource},
  },
  state::State,
};

#[instrument]
async fn validate_config(
  config: &mut PartialBuilderConfig,
  user: &User,
) -> anyhow::Result<()> {
  match config {
    PartialBuilderConfig::Server(PartialServerBuilderConfig {
      server_id: Some(server_id),
    }) if !server_id.is_empty() => {
      let server = Server::get_resource_check_permissions(
        server_id,
        user,
        PermissionLevel::Write,
      )
      .await?;
      *server_id = server.id;
    }
    _ => {}
  }
  Ok(())
}

#[async_trait]
impl Resolve<CreateBuilder, User> for State {
  #[instrument(name = "CreateBuilder", skip(self))]
  async fn resolve(
    &self,
    CreateBuilder { name, mut config }: CreateBuilder,
    user: User,
  ) -> anyhow::Result<Builder> {
    let start_ts = monitor_timestamp();
    if ObjectId::from_str(&name).is_ok() {
      return Err(anyhow!("valid ObjectIds cannot be used as names"));
    }
    validate_config(&mut config, &user).await?;

    let builder = Builder {
      id: Default::default(),
      name,
      updated_at: start_ts,
      description: Default::default(),
      tags: Default::default(),
      config: config.into(),
      info: Default::default(),
    };
    let builder_id = db_client()
      .await
      .builders
      .insert_one(builder, None)
      .await
      .context("failed to add builder to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let builder = Builder::get_resource(&builder_id).await?;
    create_permission(&user, &builder, PermissionLevel::Write).await;
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
impl Resolve<CopyBuilder, User> for State {
  #[instrument(name = "CopyBuilder", skip(self))]
  async fn resolve(
    &self,
    CopyBuilder { name, id }: CopyBuilder,
    user: User,
  ) -> anyhow::Result<Builder> {
    let Builder {
      config,
      description,
      ..
    } = Builder::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;
    let start_ts = monitor_timestamp();
    let builder = Builder {
      id: Default::default(),
      name,
      updated_at: start_ts,
      description,
      tags: Default::default(),
      config,
      info: (),
    };
    let builder_id = db_client()
      .await
      .builders
      .insert_one(builder, None)
      .await
      .context("failed to add builder to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let builder = Builder::get_resource(&builder_id).await?;
    create_permission(&user, &builder, PermissionLevel::Write).await;
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
impl Resolve<DeleteBuilder, User> for State {
  #[instrument(name = "DeleteBuilder", skip(self))]
  async fn resolve(
    &self,
    DeleteBuilder { id }: DeleteBuilder,
    user: User,
  ) -> anyhow::Result<Builder> {
    let builder = Builder::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    db_client()
      .await
      .builds
      .update_many(
        doc! { "config.builder.params.builder_id": &id },
        mungos::update::Update::Set(
          doc! { "config.builder.params.builder_id": "" },
        ),
        None,
      )
      .await?;

    delete_one_by_id(&db_client().await.builders, &id, None)
      .await
      .context("failed to delete builder from database")?;

    delete_all_permissions_on_resource(&builder).await;

    let mut update =
      make_update(&builder, Operation::DeleteBuilder, &user);

    update.push_simple_log(
      "delete builder",
      format!("deleted builder {}", builder.name),
    );

    update.finalize();
    add_update(update).await?;

    remove_from_recently_viewed(&builder).await?;

    Ok(builder)
  }
}

#[async_trait]
impl Resolve<UpdateBuilder, User> for State {
  #[instrument(name = "UpdateBuilder", skip(self))]
  async fn resolve(
    &self,
    UpdateBuilder { id, mut config }: UpdateBuilder,
    user: User,
  ) -> anyhow::Result<Builder> {
    let builder = Builder::get_resource_check_permissions(
      &id,
      &user,
      PermissionLevel::Write,
    )
    .await?;

    validate_config(&mut config, &user).await?;

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
      &db_client().await.builders,
      &id,
      mungos::update::Update::FlattenSet(
        doc! { "config": to_bson(&config)? },
      ),
      None,
    )
    .await?;

    let builder = Builder::get_resource(&id).await?;

    update.finalize();
    add_update(update).await?;

    Ok(builder)
  }
}
