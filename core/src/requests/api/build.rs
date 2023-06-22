use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_helpers::{all_logs_success, monitor_timestamp};
use monitor_types::{
    entities::{
        build::Build,
        update::{Log, Update, UpdateStatus, UpdateTarget},
        Operation, PermissionLevel,
    },
    permissioned::Permissioned,
    requests::api::{CreateBuild, DeleteBuild, GetBuild, ListBuilds, UpdateBuild},
};
use mungos::mongodb::bson::{doc, to_bson};
use resolver_api::Resolve;

use crate::{auth::RequestUser, helpers::empty_or_only_spaces, state::State};

#[async_trait]
impl Resolve<GetBuild, RequestUser> for State {
    async fn resolve(&self, GetBuild { id }: GetBuild, user: RequestUser) -> anyhow::Result<Build> {
        self.get_build_check_permissions(&id, &user, PermissionLevel::Read)
            .await
    }
}

#[async_trait]
impl Resolve<ListBuilds, RequestUser> for State {
    async fn resolve(
        &self,
        ListBuilds { query }: ListBuilds,
        user: RequestUser,
    ) -> anyhow::Result<Vec<Build>> {
        let builds = self
            .db
            .builds
            .get_some(query, None)
            .await
            .context("failed to pull builds from mongo")?;

        let builds = if user.is_admin {
            builds
        } else {
            builds
                .into_iter()
                .filter(|build| build.get_user_permissions(&user.id) > PermissionLevel::None)
                .collect()
        };

        Ok(builds)
    }
}

#[async_trait]
impl Resolve<CreateBuild, RequestUser> for State {
    async fn resolve(
        &self,
        CreateBuild { name, config }: CreateBuild,
        user: RequestUser,
    ) -> anyhow::Result<Build> {
        if let Some(server_id) = &config.server_id {
            self.get_server_check_permissions(server_id, &user, PermissionLevel::Update)
                .await
                .context("cannot create build on this server")?;
        }
        let start_ts = monitor_timestamp();
        let build = Build {
            id: Default::default(),
            name,
            created_at: start_ts,
            updated_at: start_ts,
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            description: Default::default(),
            config: config.into(),
        };
        let build_id = self
            .db
            .builds
            .create_one(&build)
            .await
            .context("failed to add build to db")?;
        let build = self.get_build(&build_id).await?;
        let update = Update {
            target: UpdateTarget::Build(build_id),
            operation: Operation::CreateBuild,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            success: true,
            logs: vec![
                Log::simple(
                    "create build",
                    format!("created build\nid: {}\nname: {}", build.id, build.name),
                ),
                Log::simple("config", format!("{:#?}", build.config)),
            ],
            ..Default::default()
        };

        self.add_update(update).await?;

        Ok(build)
    }
}

#[async_trait]
impl Resolve<DeleteBuild, RequestUser> for State {
    async fn resolve(
        &self,
        DeleteBuild { id }: DeleteBuild,
        user: RequestUser,
    ) -> anyhow::Result<Build> {
        if self.action_states.build.busy(&id).await {
            return Err(anyhow!("build busy"));
        }

        let build = self
            .get_build_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let start_ts = monitor_timestamp();

        let mut update = Update {
            target: UpdateTarget::Build(id.clone()),
            operation: Operation::DeleteBuild,
            start_ts,
            operator: user.id.clone(),
            success: true,
            status: UpdateStatus::InProgress,
            ..Default::default()
        };

        update.id = self.add_update(update.clone()).await?;

        let res = self
            .db
            .builds
            .delete_one(&id)
            .await
            .context("failed to delete build from mongo");

        let log = match res {
            Ok(_) => Log::simple("delete build", format!("deleted build {}", build.name)),
            Err(e) => Log::error("delete build", format!("failed to delete build\n{e:#?}")),
        };

        update.logs.push(log);
        update.end_ts = Some(monitor_timestamp());
        update.status = UpdateStatus::Complete;
        update.success = all_logs_success(&update.logs);

        self.update_update(update).await?;

        Ok(build)
    }
}

#[async_trait]
impl Resolve<UpdateBuild, RequestUser> for State {
    async fn resolve(
        &self,
        UpdateBuild { id, mut config }: UpdateBuild,
        user: RequestUser,
    ) -> anyhow::Result<Build> {
        if self.action_states.build.busy(&id).await {
            return Err(anyhow!("build busy"));
        }

        let build = self
            .get_build_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let inner = || async move {
            let start_ts = monitor_timestamp();

            if let Some(build_args) = &mut config.build_args {
                build_args.retain(|v| {
                    !empty_or_only_spaces(&v.variable) && !empty_or_only_spaces(&v.value)
                })
            }
            if let Some(extra_args) = &mut config.extra_args {
                extra_args.retain(|v| !empty_or_only_spaces(v))
            }

            self.db
                .builds
                .update_one(
                    &id,
                    mungos::Update::<()>::Set(doc! { "config": to_bson(&config)? }),
                )
                .await
                .context("failed to update server on mongo")?;

            let update = Update {
                operation: Operation::UpdateBuild,
                target: UpdateTarget::Build(id.clone()),
                start_ts,
                end_ts: Some(monitor_timestamp()),
                status: UpdateStatus::Complete,
                logs: vec![Log::simple(
                    "build update",
                    serde_json::to_string_pretty(&config).unwrap(),
                )],
                operator: user.id.clone(),
                success: true,
                ..Default::default()
            };

            self.add_update(update).await?;

            let build = self.get_build(&id).await?;

            anyhow::Ok(build)
        };

        self.action_states
            .build
            .update_entry(build.id.clone(), |entry| {
                entry.updating = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .build
            .update_entry(build.id, |entry| {
                entry.updating = false;
            })
            .await;

        res
    }
}
