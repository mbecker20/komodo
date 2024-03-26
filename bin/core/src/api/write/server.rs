use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_client::{
  api::write::*,
  entities::{
    monitor_timestamp,
    permission::PermissionLevel,
    server::Server,
    update::{Log, ResourceTarget, Update, UpdateStatus},
    user::User,
    Operation,
  },
};
use mungos::{
  by_id::{delete_one_by_id, update_one_by_id},
  mongodb::bson::{doc, to_bson},
};
use periphery_client::api;
use resolver_api::Resolve;

use crate::{
  db::db_client,
  helpers::{
    add_update, cache::server_status_cache, create_permission,
    make_update, periphery_client, remove_from_recently_viewed,
    resource::StateResource, update_update,
  },
  monitor::update_cache_for_server,
  state::{action_states, State},
};

#[async_trait]
impl Resolve<CreateServer, User> for State {
  async fn resolve(
    &self,
    CreateServer { name, config }: CreateServer,
    user: User,
  ) -> anyhow::Result<Server> {
    if !user.admin && !user.create_server_permissions {
      return Err(anyhow!(
        "user does not have create server permissions"
      ));
    }
    let start_ts = monitor_timestamp();
    let server = Server {
      id: Default::default(),
      name,
      updated_at: start_ts,
      description: Default::default(),
      tags: Default::default(),
      config: config.into(),
      info: (),
    };
    let server_id = db_client()
      .await
      .servers
      .insert_one(&server, None)
      .await
      .context("failed to add server to db")?
      .inserted_id
      .as_object_id()
      .context("inserted_id is not ObjectId")?
      .to_string();
    let server: Server = self.get_resource(&server_id).await?;
    create_permission(&user, &server, PermissionLevel::Update).await;
    let update = Update {
      target: ResourceTarget::Server(server_id),
      operation: Operation::CreateServer,
      start_ts,
      end_ts: Some(monitor_timestamp()),
      operator: user.id.clone(),
      success: true,
      logs: vec![
        Log::simple(
          "create server",
          format!(
            "created server\nid: {}\nname: {}",
            server.id, server.name
          ),
        ),
        Log::simple("config", format!("{:#?}", server.config)),
      ],
      ..Default::default()
    };

    add_update(update).await?;

    update_cache_for_server(&server).await;

    Ok(server)
  }
}

#[async_trait]
impl Resolve<DeleteServer, User> for State {
  async fn resolve(
    &self,
    DeleteServer { id }: DeleteServer,
    user: User,
  ) -> anyhow::Result<Server> {
    if action_states().server.busy(&id).await {
      return Err(anyhow!("server busy"));
    }

    let server: Server = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    let start_ts = monitor_timestamp();

    db_client()
      .await
      .builds
      .update_many(
        doc! { "config.builder.params.server_id": &id },
        doc! { "$set": { "config.builder.params.server_id": "" } },
        None,
      )
      .await
      .context("failed to detach server from builds")?;

    db_client()
      .await
      .deployments
      .update_many(
        doc! { "config.server_id": &id },
        doc! { "$set": { "config.server_id": "" } },
        None,
      )
      .await
      .context("failed to detach server from deployments")?;

    db_client()
      .await
      .repos
      .update_many(
        doc! { "config.server_id": &id },
        doc! { "$set": { "config.server_id": "" } },
        None,
      )
      .await
      .context("failed to detach server from repos")?;

    delete_one_by_id(&db_client().await.servers, &id, None)
      .await
      .context("failed to delete server from mongo")?;

    let mut update = Update {
      target: ResourceTarget::Server(id.clone()),
      operation: Operation::DeleteServer,
      start_ts,
      operator: user.id.clone(),
      logs: vec![Log::simple(
        "delete server",
        format!("deleted server {}", server.name),
      )],
      ..Default::default()
    };

    update.finalize();
    add_update(update).await?;

    server_status_cache().remove(&id).await;

    remove_from_recently_viewed(&server).await?;

    Ok(server)
  }
}

#[async_trait]
impl Resolve<UpdateServer, User> for State {
  async fn resolve(
    &self,
    UpdateServer { id, config }: UpdateServer,
    user: User,
  ) -> anyhow::Result<Server> {
    if action_states().server.busy(&id).await {
      return Err(anyhow!("server busy"));
    }
    let server: Server = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;
    let mut update =
      make_update(&server, Operation::UpdateServer, &user);

    update_one_by_id(
      &db_client().await.servers,
      &id,
      mungos::update::Update::FlattenSet(
        doc! { "config": to_bson(&config)? },
      ),
      None,
    )
    .await
    .context("failed to update server on mongo")?;

    update.push_simple_log(
      "server update",
      serde_json::to_string_pretty(&config)?,
    );

    let new_server: Server = self.get_resource(&id).await?;

    update_cache_for_server(&new_server).await;

    update.finalize();

    add_update(update).await?;

    Ok(new_server)
  }
}

#[async_trait]
impl Resolve<RenameServer, User> for State {
  async fn resolve(
    &self,
    RenameServer { id, name }: RenameServer,
    user: User,
  ) -> anyhow::Result<Update> {
    let server: Server = self
      .get_resource_check_permissions(
        &id,
        &user,
        PermissionLevel::Update,
      )
      .await?;
    let mut update =
      make_update(&server, Operation::RenameServer, &user);

    update_one_by_id(&db_client().await.servers, &id, mungos::update::Update::Set(doc! { "name": &name, "updated_at": monitor_timestamp() }), None)
      .await
      .context("failed to update server on db. this name may already be taken.")?;
    update.push_simple_log(
      "rename server",
      format!("renamed server {id} from {} to {name}", server.name),
    );
    update.finalize();
    update.id = add_update(update.clone()).await?;
    Ok(update)
  }
}

#[async_trait]
impl Resolve<CreateNetwork, User> for State {
  async fn resolve(
    &self,
    CreateNetwork { server_id, name }: CreateNetwork,
    user: User,
  ) -> anyhow::Result<Update> {
    let server: Server = self
      .get_resource_check_permissions(
        &server_id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    let periphery = periphery_client(&server)?;

    let mut update =
      make_update(&server, Operation::CreateNetwork, &user);
    update.status = UpdateStatus::InProgress;
    update.id = add_update(update.clone()).await?;

    match periphery
      .request(api::network::CreateNetwork { name, driver: None })
      .await
    {
      Ok(log) => update.logs.push(log),
      Err(e) => {
        update.push_error_log("create network", format!("{e:#?}"))
      }
    };

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}

#[async_trait]
impl Resolve<DeleteNetwork, User> for State {
  async fn resolve(
    &self,
    DeleteNetwork { server_id, name }: DeleteNetwork,
    user: User,
  ) -> anyhow::Result<Update> {
    let server: Server = self
      .get_resource_check_permissions(
        &server_id,
        &user,
        PermissionLevel::Update,
      )
      .await?;

    let periphery = periphery_client(&server)?;

    let mut update =
      make_update(&server, Operation::DeleteNetwork, &user);
    update.status = UpdateStatus::InProgress;
    update.id = add_update(update.clone()).await?;

    match periphery
      .request(api::network::DeleteNetwork { name })
      .await
    {
      Ok(log) => update.logs.push(log),
      Err(e) => {
        update.push_error_log("delete network", format!("{e:#?}"))
      }
    };

    update.finalize();
    update_update(update.clone()).await?;

    Ok(update)
  }
}
