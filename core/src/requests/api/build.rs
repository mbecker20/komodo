use std::pin::Pin;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use futures::Future;
use monitor_helpers::{all_logs_success, monitor_timestamp};
use monitor_types::{
    entities::{
        build::{Build, BuildBuilderConfig},
        builder::{AwsBuilder, BuilderConfig},
        update::{Log, Update, UpdateStatus, UpdateTarget},
        Operation, PermissionLevel,
    },
    permissioned::Permissioned,
    requests::api::{CreateBuild, DeleteBuild, GetBuild, ListBuilds, RunBuild, UpdateBuild},
};
use mungos::mongodb::bson::{doc, to_bson};
use periphery_client::PeripheryClient;
use resolver_api::Resolve;

use crate::{
    auth::RequestUser,
    cloud::{aws::Ec2Instance, InstanceCleanupData},
    helpers::empty_or_only_spaces,
    state::State,
};

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
        if let Some(BuildBuilderConfig::Server { server_id }) = &config.builder {
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

#[async_trait]
impl Resolve<RunBuild, RequestUser> for State {
    async fn resolve(
        &self,
        RunBuild { build_id }: RunBuild,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        if self.action_states.build.busy(&build_id).await {
            return Err(anyhow!("build busy"));
        }

        let mut build = self
            .get_build_check_permissions(&build_id, &user, PermissionLevel::Execute)
            .await?;

        let inner = || async move {
            build.config.version.increment();
            let mut update = Update {
                target: UpdateTarget::Build(build.id.clone()),
                operation: Operation::RunBuild,
                start_ts: monitor_timestamp(),
                status: UpdateStatus::InProgress,
                success: true,
                operator: user.id.clone(),
                version: build.config.version.clone(),
                ..Default::default()
            };
            update.id = self.add_update(update.clone()).await?;

            let builder = self.get_build_builder(&build, &mut update).await;

            if let Err(e) = &builder {
                update
                    .logs
                    .push(Log::error("get builder", format!("{e:#?}")));
                update.finalize();
                self.update_update(update.clone()).await?;
                return Ok(update);
            }

            let (periphery, cleanup_data) = builder.unwrap();

            // ...

            self.cleanup_builder_instance(cleanup_data, &mut update)
                .await;

            update.finalize();

            self.update_update(update.clone()).await?;

            Ok(update)
        };

        self.action_states
            .build
            .update_entry(build_id.clone(), |entry| {
                entry.building = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .build
            .update_entry(build_id, |entry| {
                entry.building = false;
            })
            .await;

        res
    }
}

impl State {
    async fn get_build_builder(
        &self,
        build: &Build,
        update: &mut Update,
    ) -> anyhow::Result<(PeripheryClient, Option<InstanceCleanupData>)> {
        match &build.config.builder {
            BuildBuilderConfig::Server { server_id } => {
                let server = self.get_server(server_id).await?;
                let periphery = self.periphery_client(&server);
                Ok((periphery, None))
            }
            BuildBuilderConfig::Builder { builder_id } => {
                let builder = self.get_builder(builder_id).await?;
                match builder.config {
                    BuilderConfig::AwsBuilder(config) => {
                        self.get_aws_builder(build, config, update).await
                    }
                }
            }
        }
    }

    async fn get_aws_builder(
        &self,
        build: &Build,
        builder: AwsBuilder,
        update: &mut Update,
    ) -> anyhow::Result<(PeripheryClient, Option<InstanceCleanupData>)> {
        let instance_name = format!(
            "BUILDER-{}-v{}",
            build.name,
            build.config.version.to_string()
        );
        let Ec2Instance { instance_id, ip } =
            self.create_ec2_instance(&instance_name, &builder).await?;

        update
            .logs
            .push(Log::simple("started builder instance", format!("")));

        self.update_update(update.clone()).await?;

        let periphery = PeripheryClient::new(format!("http://{ip}:8000"), &self.config.passkey);

        Ok((
            periphery,
            InstanceCleanupData::Aws {
                instance_id,
                region: builder.region,
            }
            .into(),
        ))
    }

    async fn cleanup_builder_instance(
        &self,
        cleanup_data: Option<InstanceCleanupData>,
        update: &mut Update,
    ) {
        if cleanup_data.is_none() {
            return;
        }
        match cleanup_data.unwrap() {
            InstanceCleanupData::Aws {
                instance_id,
                region,
            } => {
                let res = self
                    .terminate_ec2_instance(region, &instance_id)
                    .await
                    .context("failed to terminate ec2 instance");
                let log = match res {
                    Ok(_) => Log::simple(
                        "terminate instance",
                        format!("terminate instance id {}", instance_id),
                    ),
                    Err(e) => Log::error("terminate instance", format!("{e:#?}")),
                };
                update.logs.push(log);
            }
        }
    }
}
