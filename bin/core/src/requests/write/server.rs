use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_types::{
    entities::{
        server::Server,
        update::{Log, ResourceTarget, Update, UpdateStatus},
        Operation, PermissionLevel,
    },
    monitor_timestamp,
    requests::write::*,
};
use mungos::mongodb::bson::{doc, to_bson};
use periphery_client::requests;
use resolver_api::Resolve;

use crate::{
    auth::RequestUser,
    helpers::{make_update, resource::StateResource},
    state::State,
};

#[async_trait]
impl Resolve<CreateServer, RequestUser> for State {
    async fn resolve(&self, req: CreateServer, user: RequestUser) -> anyhow::Result<Server> {
        if !user.is_admin && !user.create_server_permissions {
            return Err(anyhow!("user does not have create server permissions"));
        }
        let start_ts = monitor_timestamp();
        let server = Server {
            id: Default::default(),
            name: req.name,
            updated_at: start_ts,
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            description: Default::default(),
            tags: Default::default(),
            config: req.config.into(),
            info: (),
        };
        let server_id = self
            .db
            .servers
            .create_one(&server)
            .await
            .context("failed to add server to db")?;
        let server: Server = self.get_resource(&server_id).await?;
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
                    format!("created server\nid: {}\nname: {}", server.id, server.name),
                ),
                Log::simple("config", format!("{:#?}", server.config)),
            ],
            ..Default::default()
        };

        self.add_update(update).await?;

        self.update_cache_for_server(&server).await;

        Ok(server)
    }
}

#[async_trait]
impl Resolve<DeleteServer, RequestUser> for State {
    async fn resolve(
        &self,
        DeleteServer { id }: DeleteServer,
        user: RequestUser,
    ) -> anyhow::Result<Server> {
        if self.action_states.server.busy(&id).await {
            return Err(anyhow!("server busy"));
        }

        let server: Server = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let start_ts = monitor_timestamp();

        self.db
            .builds
            .update_many(
                doc! { "config.builder.params.server_id": &id },
                doc! { "$set": { "config.builder.params.server_id": "" } },
            )
            .await
            .context("failed to detach server from builds")?;

        self.db
            .deployments
            .update_many(
                doc! { "config.server_id": &id },
                doc! { "$set": { "config.server_id": "" } },
            )
            .await
            .context("failed to detach server from deployments")?;

        self.db
            .repos
            .update_many(
                doc! { "config.server_id": &id },
                doc! { "$set": { "config.server_id": "" } },
            )
            .await
            .context("failed to detach server from repos")?;

        self.db
            .servers
            .delete_one(&id)
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
        self.add_update(update).await?;

        self.server_status_cache.remove(&id).await;

        self.remove_from_recently_viewed(&server).await?;

        Ok(server)
    }
}

#[async_trait]
impl Resolve<UpdateServer, RequestUser> for State {
    async fn resolve(
        &self,
        UpdateServer { id, config }: UpdateServer,
        user: RequestUser,
    ) -> anyhow::Result<Server> {
        if self.action_states.server.busy(&id).await {
            return Err(anyhow!("server busy"));
        }
        let server: Server = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;
        let mut update = make_update(&server, Operation::UpdateServer, &user);
        self.db
            .servers
            .update_one(
                &id,
                mungos::Update::FlattenSet(doc! { "config": to_bson(&config)? }),
            )
            .await
            .context("failed to update server on mongo")?;

        update.push_simple_log("server update", serde_json::to_string_pretty(&config)?);
        
        let new_server: Server = self.get_resource(&id).await?;

        self.update_cache_for_server(&new_server).await;

        update.finalize();

        self.add_update(update).await?;

        Ok(new_server)
    }
}

#[async_trait]
impl Resolve<RenameServer, RequestUser> for State {
    async fn resolve(
        &self,
        RenameServer { id, name }: RenameServer,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        let server: Server = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;
        let mut update = make_update(&server, Operation::RenameServer, &user);
        self.db
            .updates
            .update_one(
                &id,
                mungos::Update::Set(doc! { "name": &name, "updated_at": monitor_timestamp() }),
            )
            .await
            .context("failed to update server on db. this name may already be taken.")?;
        update.push_simple_log(
            "rename server",
            format!("renamed server {id} from {} to {name}", server.name),
        );
        update.finalize();
        update.id = self.add_update(update.clone()).await?;
        Ok(update)
    }
}

#[async_trait]
impl Resolve<CreateNetwork, RequestUser> for State {
    async fn resolve(
        &self,
        CreateNetwork { server_id, name }: CreateNetwork,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        let server: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Update)
            .await?;

        let periphery = self.periphery_client(&server)?;

        let mut update = make_update(&server, Operation::CreateNetwork, &user);
        update.status = UpdateStatus::InProgress;
        update.id = self.add_update(update.clone()).await?;

        match periphery
            .request(requests::CreateNetwork { name, driver: None })
            .await
        {
            Ok(log) => update.logs.push(log),
            Err(e) => update.push_error_log("create network", format!("{e:#?}")),
        };

        update.finalize();
        self.update_update(update.clone()).await?;

        Ok(update)
    }
}

#[async_trait]
impl Resolve<DeleteNetwork, RequestUser> for State {
    async fn resolve(
        &self,
        DeleteNetwork { server_id, name }: DeleteNetwork,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        let server: Server = self
            .get_resource_check_permissions(&server_id, &user, PermissionLevel::Update)
            .await?;

        let periphery = self.periphery_client(&server)?;

        let mut update = make_update(&server, Operation::DeleteNetwork, &user);
        update.status = UpdateStatus::InProgress;
        update.id = self.add_update(update.clone()).await?;

        match periphery.request(requests::DeleteNetwork { name }).await {
            Ok(log) => update.logs.push(log),
            Err(e) => update.push_error_log("delete network", format!("{e:#?}")),
        };

        update.finalize();
        self.update_update(update.clone()).await?;

        Ok(update)
    }
}
