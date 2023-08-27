use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_types::{
    all_logs_success,
    entities::{
        build::Build,
        deployment::{Deployment, DeploymentImage, DockerContainerState},
        server::Server,
        update::{Log, ResourceTarget, Update, UpdateStatus},
        Operation, PermissionLevel,
    },
    monitor_timestamp,
    requests::write::*,
    to_monitor_name,
};
use mungos::mongodb::bson::{doc, to_bson};
use periphery_client::requests;
use resolver_api::Resolve;

use crate::{
    auth::RequestUser,
    helpers::{empty_or_only_spaces, make_update, resource::StateResource},
    state::State,
};

#[async_trait]
impl Resolve<CreateDeployment, RequestUser> for State {
    async fn resolve(
        &self,
        CreateDeployment { name, config }: CreateDeployment,
        user: RequestUser,
    ) -> anyhow::Result<Deployment> {
        let name = to_monitor_name(&name);
        if let Some(server_id) = &config.server_id {
            if !server_id.is_empty() {
                let _: Server = self.get_resource_check_permissions(server_id, &user, PermissionLevel::Update)
                    .await
                    .context("cannot create deployment on this server. user must have update permissions on the server to perform this action.")?;
            }
        }
        if let Some(DeploymentImage::Build { build_id, .. }) = &config.image {
            if !build_id.is_empty() {
                let _: Build = self.get_resource_check_permissions(build_id, &user, PermissionLevel::Read)
                    .await
                    .context("cannot create deployment with this build attached. user must have at least read permissions on the build to perform this action.")?;
            }
        }
        let start_ts = monitor_timestamp();
        let deployment = Deployment {
            id: Default::default(),
            name,
            updated_at: start_ts,
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            description: Default::default(),
            tags: Default::default(),
            config: config.into(),
            info: (),
        };
        let deployment_id = self
            .db
            .deployments
            .create_one(&deployment)
            .await
            .context("failed to add deployment to db")?;
        let deployment: Deployment = self.get_resource(&deployment_id).await?;
        let update = Update {
            target: ResourceTarget::Deployment(deployment_id),
            operation: Operation::CreateDeployment,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            success: true,
            logs: vec![
                Log::simple(
                    "create deployment",
                    format!(
                        "created deployment\nid: {}\nname: {}",
                        deployment.id, deployment.name
                    ),
                ),
                Log::simple("config", format!("{:#?}", deployment.config)),
            ],
            ..Default::default()
        };

        self.add_update(update).await?;

        Ok(deployment)
    }
}

#[async_trait]
impl Resolve<CopyDeployment, RequestUser> for State {
    async fn resolve(
        &self,
        CopyDeployment { name, id }: CopyDeployment,
        user: RequestUser,
    ) -> anyhow::Result<Deployment> {
        let Deployment {
            config,
            description,
            tags,
            ..
        } = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;
        if !config.server_id.is_empty() {
            let _: Server = self.get_resource_check_permissions(&config.server_id, &user, PermissionLevel::Update)
                    .await
                    .context("cannot create deployment on this server. user must have update permissions on the server to perform this action.")?;
        }
        if let DeploymentImage::Build { build_id, .. } = &config.image {
            if !build_id.is_empty() {
                let _: Build = self.get_resource_check_permissions(build_id, &user, PermissionLevel::Read)
                    .await
                    .context("cannot create deployment with this build attached. user must have at least read permissions on the build to perform this action.")?;
            }
        }
        let start_ts = monitor_timestamp();
        let deployment = Deployment {
            id: Default::default(),
            name,
            updated_at: start_ts,
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            description,
            tags,
            config,
            info: (),
        };
        let deployment_id = self
            .db
            .deployments
            .create_one(&deployment)
            .await
            .context("failed to add deployment to db")?;
        let deployment: Deployment = self.get_resource(&deployment_id).await?;
        let update = Update {
            target: ResourceTarget::Deployment(deployment_id),
            operation: Operation::CreateDeployment,
            start_ts,
            end_ts: Some(monitor_timestamp()),
            operator: user.id.clone(),
            success: true,
            logs: vec![
                Log::simple(
                    "create deployment",
                    format!(
                        "created deployment\nid: {}\nname: {}",
                        deployment.id, deployment.name
                    ),
                ),
                Log::simple("config", format!("{:#?}", deployment.config)),
            ],
            ..Default::default()
        };

        self.add_update(update).await?;

        Ok(deployment)
    }
}

#[async_trait]
impl Resolve<DeleteDeployment, RequestUser> for State {
    async fn resolve(
        &self,
        DeleteDeployment { id }: DeleteDeployment,
        user: RequestUser,
    ) -> anyhow::Result<Deployment> {
        if self.action_states.deployment.busy(&id).await {
            return Err(anyhow!("deployment busy"));
        }

        let deployment: Deployment = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let inner = || async move {
            let start_ts = monitor_timestamp();

            let state = self
                .get_deployment_state(&deployment)
                .await
                .context("failed to get container state")?;

            let mut update = Update {
                target: ResourceTarget::Deployment(deployment.id.clone()),
                operation: Operation::DeleteDeployment,
                start_ts,
                operator: user.id.clone(),
                success: true,
                status: UpdateStatus::InProgress,
                ..Default::default()
            };

            update.id = self.add_update(update.clone()).await?;

            if !matches!(
                state,
                DockerContainerState::NotDeployed | DockerContainerState::Unknown
            ) {
                // container needs to be destroyed
                let server: anyhow::Result<Server> =
                    self.get_resource(&deployment.config.server_id).await;
                if let Err(e) = server {
                    update.logs.push(Log::error(
                        "remove container",
                        format!(
                            "failed to retrieve server at {} from db | {e:#?}",
                            deployment.config.server_id
                        ),
                    ));
                } else if let Ok(server) = server {
                    match self.periphery_client(&server) {
                        Ok(periphery) => match periphery
                            .request(requests::RemoveContainer {
                                name: deployment.name.clone(),
                                signal: deployment.config.termination_signal.into(),
                                time: deployment.config.termination_timeout.into(),
                            })
                            .await
                        {
                            Ok(log) => update.logs.push(log),
                            Err(e) => update.push_error_log(
                                "remove container",
                                format!("failed to remove container on periphery | {e:#?}"),
                            ),
                        },
                        Err(e) => update.push_error_log(
                            "remove container",
                            format!("failed to remove container on periphery | {e:#?}"),
                        ),
                    };
                }
            }

            let res = self
                .db
                .deployments
                .delete_one(&deployment.id)
                .await
                .context("failed to delete deployment from mongo");

            let log = match res {
                Ok(_) => Log::simple(
                    "delete deployment",
                    format!("deleted deployment {}", deployment.name),
                ),
                Err(e) => Log::error(
                    "delete deployment",
                    format!("failed to delete deployment\n{e:#?}"),
                ),
            };

            update.logs.push(log);
            update.end_ts = Some(monitor_timestamp());
            update.status = UpdateStatus::Complete;
            update.success = all_logs_success(&update.logs);

            self.update_update(update).await?;

            self.remove_from_recently_viewed(&deployment).await?;

            Ok(deployment)
        };

        self.action_states
            .deployment
            .update_entry(id.clone(), |entry| {
                entry.deleting = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .deployment
            .update_entry(id, |entry| {
                entry.deleting = false;
            })
            .await;

        res
    }
}

#[async_trait]
impl Resolve<UpdateDeployment, RequestUser> for State {
    async fn resolve(
        &self,
        UpdateDeployment { id, mut config }: UpdateDeployment,
        user: RequestUser,
    ) -> anyhow::Result<Deployment> {
        if self.action_states.deployment.busy(&id).await {
            return Err(anyhow!("deployment busy"));
        }

        let deployment: Deployment = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let inner = || async move {
            let start_ts = monitor_timestamp();

            if let Some(server_id) = &config.server_id {
                let _: Server = self.get_resource_check_permissions(server_id, &user, PermissionLevel::Update)
                .await
                .context("cannot create deployment on this server. user must have update permissions on the server to perform this action.")?;
            }
            if let Some(DeploymentImage::Build { build_id, .. }) = &config.image {
                let _: Build = self.get_resource_check_permissions(build_id, &user, PermissionLevel::Read)
                .await
                .context("cannot create deployment with this build attached. user must have at least read permissions on the build to perform this action.")?;
            }

            if let Some(volumes) = &mut config.volumes {
                volumes.retain(|v| {
                    !empty_or_only_spaces(&v.local) && !empty_or_only_spaces(&v.container)
                })
            }
            if let Some(ports) = &mut config.ports {
                ports.retain(|v| {
                    !empty_or_only_spaces(&v.local) && !empty_or_only_spaces(&v.container)
                })
            }
            if let Some(environment) = &mut config.environment {
                environment.retain(|v| {
                    !empty_or_only_spaces(&v.variable) && !empty_or_only_spaces(&v.value)
                })
            }
            if let Some(extra_args) = &mut config.extra_args {
                extra_args.retain(|v| !empty_or_only_spaces(v))
            }

            self.db
                .deployments
                .update_one(
                    &id,
                    mungos::Update::FlattenSet(doc! { "config": to_bson(&config)? }),
                )
                .await
                .context("failed to update server on mongo")?;

            let update = Update {
                operation: Operation::UpdateDeployment,
                target: ResourceTarget::Deployment(id.clone()),
                start_ts,
                end_ts: Some(monitor_timestamp()),
                status: UpdateStatus::Complete,
                logs: vec![Log::simple(
                    "deployment update",
                    serde_json::to_string_pretty(&config).unwrap(),
                )],
                operator: user.id.clone(),
                success: true,
                ..Default::default()
            };

            self.add_update(update).await?;

            let deployment: Deployment = self.get_resource(&id).await?;

            anyhow::Ok(deployment)
        };

        self.action_states
            .deployment
            .update_entry(deployment.id.clone(), |entry| {
                entry.updating = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .deployment
            .update_entry(deployment.id, |entry| {
                entry.updating = false;
            })
            .await;

        res
    }
}

#[async_trait]
impl Resolve<RenameDeployment, RequestUser> for State {
    async fn resolve(
        &self,
        RenameDeployment { id, name }: RenameDeployment,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        let name = to_monitor_name(&name);
        if self.action_states.deployment.busy(&id).await {
            return Err(anyhow!("deployment busy"));
        }

        let deployment: Deployment = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let inner = || async {
            let name = to_monitor_name(&name);

            let container_state = self.get_deployment_state(&deployment).await?;

            if container_state == DockerContainerState::Unknown {
                return Err(anyhow!(
                    "cannot rename deployment when container status is unknown"
                ));
            }

            let mut update = make_update(&deployment, Operation::RenameDeployment, &user);

            self.db
                .deployments
                .update_one(
                    &deployment.id,
                    mungos::Update::Set(doc! { "name": &name, "updated_at": monitor_timestamp() }),
                )
                .await
                .context("failed to update deployment name on db")?;

            if container_state != DockerContainerState::NotDeployed {
                let server: Server = self.get_resource(&deployment.config.server_id).await?;
                let log = self
                    .periphery_client(&server)?
                    .request(requests::RenameContainer {
                        curr_name: deployment.name.clone(),
                        new_name: name.clone(),
                    })
                    .await
                    .context("failed to rename container on server")?;
                update.logs.push(log);
            }

            update.push_simple_log(
                "rename deployment",
                format!("renamed deployment from {} to {}", deployment.name, name),
            );
            update.finalize();

            self.add_update(update.clone()).await?;

            Ok(update)
        };

        self.action_states
            .deployment
            .update_entry(id.clone(), |entry| {
                entry.renaming = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .deployment
            .update_entry(id, |entry| {
                entry.renaming = false;
            })
            .await;

        res
    }
}
