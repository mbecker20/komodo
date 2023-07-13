use anyhow::Context;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        builder::Builder,
        update::{Log, ResourceTarget, Update},
        Operation, PermissionLevel,
    },
    monitor_timestamp,
    requests::write::*,
};
use mungos::mongodb::bson::{doc, to_bson};
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

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
        };
        let builder_id = self
            .db
            .builders
            .create_one(builder)
            .await
            .context("failed to add builder to db")?;
        let builder = self.get_builder(&builder_id).await?;
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

        self.add_update(update).await?;

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
            .get_builder_check_permissions(&id, &user, PermissionLevel::Update)
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
        };
        let builder_id = self
            .db
            .builders
            .create_one(builder)
            .await
            .context("failed to add builder to db")?;
        let builder = self.get_builder(&builder_id).await?;
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

        self.add_update(update).await?;

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
        let builder = self
            .get_builder_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let start_ts = monitor_timestamp();

        self.db
            .builds
            .update_many(
                doc! { "config.builder.params.builder_id": &id },
                doc! { "$set": { "config.builder.params.builder_id": "" } },
            )
            .await?;

        self.db
            .builders
            .delete_one(&id)
            .await
            .context("failed to delete builder from database")?;

        let mut update = Update {
            target: ResourceTarget::Builder(id.clone()),
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
        self.add_update(update).await?;

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
        let builder = self
            .get_builder_check_permissions(&id, &user, PermissionLevel::Update)
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

        self.db
            .builders
            .update_one(
                &id,
                mungos::Update::Set(doc! { "config": to_bson(&config)? }),
            )
            .await?;

        let builder = self.get_builder(&id).await?;

        update.finalize();
        self.add_update(update).await?;

        Ok(builder)
    }
}
