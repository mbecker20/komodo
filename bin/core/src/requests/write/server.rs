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
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State, resource::Resource};

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

        self.update_cache_for_server(&server, 0).await;

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
        let start_ts = monitor_timestamp();
        let _: Server = self.get_resource_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;
        self.db
            .servers
            .update_one(
                &id,
                mungos::Update::Set(doc! { "config": to_bson(&config)? }),
            )
            .await
            .context("failed to update server on mongo")?;
        let update = Update {
            operation: Operation::UpdateServer,
            target: ResourceTarget::Server(id.clone()),
            start_ts,
            end_ts: Some(monitor_timestamp()),
            status: UpdateStatus::Complete,
            logs: vec![Log::simple(
                "server update",
                serde_json::to_string_pretty(&config).unwrap(),
            )],
            operator: user.id.clone(),
            success: true,
            ..Default::default()
        };

        let new_server: Server = self.get_resource(&id).await?;

        self.update_cache_for_server(&new_server, 0).await;

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
        let start_ts = monitor_timestamp();
        let server: Server = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;
        self.db
            .updates
            .update_one(
                &id,
                mungos::Update::Set(doc! { "name": &name, "updated_at": monitor_timestamp() }),
            )
            .await?;
        let mut update = Update {
            target: ResourceTarget::Deployment(id.clone()),
            operation: Operation::RenameServer,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            logs: vec![Log::simple(
                "rename server",
                format!("renamed server {id} from {} to {name}", server.name),
            )],
            status: UpdateStatus::Complete,
            success: true,
            operator: user.id.clone(),
            ..Default::default()
        };
        update.id = self.add_update(update.clone()).await?;
        Ok(update)
    }
}
