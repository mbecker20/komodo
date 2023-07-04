use std::cmp;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_types::{
    all_logs_success,
    entities::{
        deployment::{
            Deployment, DeploymentConfig, DeploymentImage, DockerContainerState,
            DockerContainerStats,
        },
        server::ServerStatus,
        update::{Log, ResourceTarget, Update, UpdateStatus},
        Operation, PermissionLevel, Version,
    },
    get_image_name, monitor_timestamp,
    permissioned::Permissioned,
    requests::api::*,
    to_monitor_name,
};
use mungos::mongodb::{
    bson::{doc, to_bson},
    options::FindOneOptions,
};
use periphery_client::requests;
use resolver_api::Resolve;

use crate::{auth::RequestUser, helpers::empty_or_only_spaces, state::State};

#[async_trait]
impl Resolve<GetDeployment, RequestUser> for State {
    async fn resolve(
        &self,
        GetDeployment { id }: GetDeployment,
        user: RequestUser,
    ) -> anyhow::Result<Deployment> {
        self.get_deployment_check_permissions(&id, &user, PermissionLevel::Read)
            .await
    }
}

#[async_trait]
impl Resolve<ListDeployments, RequestUser> for State {
    async fn resolve(
        &self,
        ListDeployments { query }: ListDeployments,
        user: RequestUser,
    ) -> anyhow::Result<Vec<Deployment>> {
        let deployments = self
            .db
            .deployments
            .get_some(query, None)
            .await
            .context("failed to pull deployments from mongo")?;

        let deployments = if user.is_admin {
            deployments
        } else {
            deployments
                .into_iter()
                .filter(|deployment| {
                    deployment.get_user_permissions(&user.id) > PermissionLevel::None
                })
                .collect()
        };

        Ok(deployments)
    }
}

const MAX_LOG_LENGTH: u64 = 5000;

#[async_trait]
impl Resolve<GetLog, RequestUser> for State {
    async fn resolve(
        &self,
        GetLog {
            deployment_id,
            tail,
        }: GetLog,
        user: RequestUser,
    ) -> anyhow::Result<Log> {
        let Deployment {
            name,
            config: DeploymentConfig { server_id, .. },
            ..
        } = self
            .get_deployment_check_permissions(&deployment_id, &user, PermissionLevel::Read)
            .await?;
        if server_id.is_empty() {
            return Ok(Log::default());
        }
        let server = self.get_server(&server_id).await?;
        self.periphery_client(&server)
            .request(requests::GetContainerLog {
                name,
                tail: cmp::min(tail, MAX_LOG_LENGTH),
            })
            .await
            .context("failed at call to periphery")
    }
}

#[async_trait]
impl Resolve<GetDeployedVersion, RequestUser> for State {
    async fn resolve(
        &self,
        GetDeployedVersion { deployment_id }: GetDeployedVersion,
        user: RequestUser,
    ) -> anyhow::Result<GetDeployedVersionResponse> {
        let Deployment {
            config: DeploymentConfig { image, .. },
            ..
        } = self
            .get_deployment_check_permissions(&deployment_id, &user, PermissionLevel::Read)
            .await?;
        let version = match image {
            DeploymentImage::Build { .. } => {
                let latest_deploy_update = self
                    .db
                    .updates
                    .find_one(
                        doc! {
                            "target": {
                                "type": "Deployment",
                                "id": deployment_id
                            },
                            "operation": Operation::DeployContainer.to_string(),
                            "status": UpdateStatus::Complete.to_string(),
                            "success": true,
                        },
                        FindOneOptions::builder().sort(doc! { "_id": -1 }).build(),
                    )
                    .await
                    .context("failed at query to get latest deploy update from mongo")?;
                if let Some(update) = latest_deploy_update {
                    if !update.version.is_none() {
                        update.version.to_string()
                    } else {
                        "unknown".to_string()
                    }
                } else {
                    "unknown".to_string()
                }
            }
            DeploymentImage::Image { image } => {
                let split = image.split(':').collect::<Vec<&str>>();
                if let Some(version) = split.get(1) {
                    version.to_string()
                } else {
                    "unknown".to_string()
                }
            }
        };
        Ok(GetDeployedVersionResponse { version })
    }
}

#[async_trait]
impl Resolve<GetDeploymentStats, RequestUser> for State {
    async fn resolve(
        &self,
        GetDeploymentStats { id }: GetDeploymentStats,
        user: RequestUser,
    ) -> anyhow::Result<DockerContainerStats> {
        let Deployment {
            name,
            config: DeploymentConfig { server_id, .. },
            ..
        } = self
            .get_deployment_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        if server_id.is_empty() {
            return Err(anyhow!("deployment has no server attached"));
        }
        let server = self.get_server(&server_id).await?;
        self.periphery_client(&server)
            .request(requests::GetContainerStats { name })
            .await
            .context("failed to get stats from periphery")
    }
}

#[async_trait]
impl Resolve<CreateDeployment, RequestUser> for State {
    async fn resolve(
        &self,
        CreateDeployment { name, config }: CreateDeployment,
        user: RequestUser,
    ) -> anyhow::Result<Deployment> {
        if let Some(server_id) = &config.server_id {
            if !server_id.is_empty() {
                self.get_server_check_permissions(server_id, &user, PermissionLevel::Update)
                    .await
                    .context("cannot create deployment on this server. user must have update permissions on the server to perform this action.")?;
            }
        }
        if let Some(DeploymentImage::Build { build_id, .. }) = &config.image {
            if !build_id.is_empty() {
                self.get_build_check_permissions(build_id, &user, PermissionLevel::Read)
                    .await
                    .context("cannot create deployment with this build attached. user must have at least read permissions on the build to perform this action.")?;
            }
        }
        let start_ts = monitor_timestamp();
        let deployment = Deployment {
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
        let deployment_id = self
            .db
            .deployments
            .create_one(&deployment)
            .await
            .context("failed to add deployment to db")?;
        let deployment = self.get_deployment(&deployment_id).await?;
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
            ..
        } = self
            .get_deployment_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;
        if !config.server_id.is_empty() {
            self.get_server_check_permissions(&config.server_id, &user, PermissionLevel::Update)
                    .await
                    .context("cannot create deployment on this server. user must have update permissions on the server to perform this action.")?;
        }
        if let DeploymentImage::Build { build_id, .. } = &config.image {
            if !build_id.is_empty() {
                self.get_build_check_permissions(build_id, &user, PermissionLevel::Read)
                    .await
                    .context("cannot create deployment with this build attached. user must have at least read permissions on the build to perform this action.")?;
            }
        }
        let start_ts = monitor_timestamp();
        let deployment = Deployment {
            id: Default::default(),
            name,
            created_at: start_ts,
            updated_at: start_ts,
            permissions: [(user.id.clone(), PermissionLevel::Update)]
                .into_iter()
                .collect(),
            description,
            config,
        };
        let deployment_id = self
            .db
            .deployments
            .create_one(&deployment)
            .await
            .context("failed to add deployment to db")?;
        let deployment = self.get_deployment(&deployment_id).await?;
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

        let deployment = self
            .get_deployment_check_permissions(&id, &user, PermissionLevel::Update)
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
                let server = self.get_server(&deployment.config.server_id).await;
                if let Err(e) = server {
                    update.logs.push(Log::error(
                        "remove container",
                        format!(
                            "failed to retrieve server at {} from mongo | {e:#?}",
                            deployment.config.server_id
                        ),
                    ));
                } else {
                    let server = server.unwrap();
                    match self
                        .periphery_client(&server)
                        .request(requests::RemoveContainer {
                            name: deployment.name.clone(),
                            signal: deployment.config.termination_signal.into(),
                            time: deployment.config.termination_timeout.into(),
                        })
                        .await
                    {
                        Ok(log) => update.logs.push(log),
                        Err(e) => update.logs.push(Log::error(
                            "remove container",
                            format!("failed to remove container on periphery | {e:#?}"),
                        )),
                    }
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

        let deployment = self
            .get_deployment_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let inner = || async move {
            let start_ts = monitor_timestamp();

            if let Some(server_id) = &config.server_id {
                self.get_server_check_permissions(server_id, &user, PermissionLevel::Update)
                .await
                .context("cannot create deployment on this server. user must have update permissions on the server to perform this action.")?;
            }
            if let Some(DeploymentImage::Build { build_id, .. }) = &config.image {
                self.get_build_check_permissions(build_id, &user, PermissionLevel::Read)
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
                    mungos::Update::Set(doc! { "config": to_bson(&config)? }),
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

            let deployment = self.get_deployment(&id).await?;

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
        if self.action_states.deployment.busy(&id).await {
            return Err(anyhow!("deployment busy"));
        }

        let deployment = self
            .get_deployment_check_permissions(&id, &user, PermissionLevel::Update)
            .await?;

        let inner = || async {
            let start_ts = monitor_timestamp();

            let mut logs = Vec::new();
            let name = to_monitor_name(&name);

            let container_state = self.get_deployment_state(&deployment).await?;

            if container_state == DockerContainerState::Unknown {
                return Err(anyhow!(
                    "cannot rename deployment when container status is unknown"
                ));
            }

            if container_state != DockerContainerState::NotDeployed {
                let server = self.get_server(&deployment.config.server_id).await?;
                match self
                    .periphery_client(&server)
                    .request(requests::RenameContainer {
                        curr_name: deployment.name.clone(),
                        new_name: name.clone(),
                    })
                    .await
                    .context("failed to rename container on server")
                {
                    Ok(log) => logs.push(log),
                    Err(e) => return Err(e),
                };
            }

            self.db
                .deployments
                .update_one(
                    &deployment.id,
                    mungos::Update::Set(
                        doc! { "name": &name, "updated_at": monitor_timestamp() },
                    ),
                )
                .await
                .context("failed to update deployment name on mongo")?;

            logs.push(Log::simple(
                "rename deployment",
                format!("renamed deployment from {} to {}", deployment.name, name),
            ));

            let update = Update {
                target: ResourceTarget::Deployment(deployment.id),
                operation: Operation::RenameDeployment,
                start_ts,
                end_ts: monitor_timestamp().into(),
                status: UpdateStatus::InProgress,
                success: all_logs_success(&logs),
                operator: user.id.clone(),
                logs,
                ..Default::default()
            };

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

#[async_trait]
impl Resolve<Deploy, RequestUser> for State {
    async fn resolve(
        &self,
        Deploy {
            deployment_id,
            stop_signal,
            stop_time,
        }: Deploy,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        if self.action_states.deployment.busy(&deployment_id).await {
            return Err(anyhow!("deployment busy"));
        }

        let mut deployment = self
            .get_deployment_check_permissions(&deployment_id, &user, PermissionLevel::Execute)
            .await?;

        if deployment.config.server_id.is_empty() {
            return Err(anyhow!("deployment has no server configured"));
        }

        let (server, status) = self
            .get_server_with_status(&deployment.config.server_id)
            .await?;
        if status != ServerStatus::Ok {
            return Err(anyhow!(
                "cannot send action when server is unreachable or disabled"
            ));
        }

        let periphery = self.periphery_client(&server);

        let inner = || async move {
            let start_ts = monitor_timestamp();

            let version = match deployment.config.image {
                DeploymentImage::Build { build_id, version } => {
                    let build = self.get_build(&build_id).await?;
                    let image_name = get_image_name(&build);
                    let version = if version.is_none() {
                        build.config.version
                    } else {
                        version
                    };
                    deployment.config.image = DeploymentImage::Image {
                        image: format!("{image_name}:{}", version.to_string()),
                    };
                    if deployment.config.docker_account.is_empty() {
                        deployment.config.docker_account = build.config.docker_account;
                    }
                    version
                }
                DeploymentImage::Image { .. } => Version::default(),
            };

            let mut update = Update {
                target: ResourceTarget::Deployment(deployment.id.clone()),
                operation: Operation::DeployContainer,
                start_ts,
                status: UpdateStatus::InProgress,
                success: true,
                operator: user.id.clone(),
                version,
                ..Default::default()
            };

            update.id = self.add_update(update.clone()).await?;

            let log = match periphery
                .request(requests::Deploy {
                    deployment,
                    stop_signal,
                    stop_time,
                })
                .await
            {
                Ok(log) => log,
                Err(e) => Log::error("deploy container", format!("{e:#?}")),
            };

            update.logs.push(log);
            update.finalize();
            self.update_cache_for_server(&server).await;
            self.update_update(update.clone()).await?;

            Ok(update)
        };

        self.action_states
            .deployment
            .update_entry(deployment_id.to_string(), |entry| {
                entry.deploying = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .deployment
            .update_entry(deployment_id, |entry| {
                entry.deploying = false;
            })
            .await;

        res
    }
}

#[async_trait]
impl Resolve<StartContainer, RequestUser> for State {
    async fn resolve(
        &self,
        StartContainer { deployment_id }: StartContainer,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        if self.action_states.deployment.busy(&deployment_id).await {
            return Err(anyhow!("deployment busy"));
        }

        let deployment = self
            .get_deployment_check_permissions(&deployment_id, &user, PermissionLevel::Execute)
            .await?;

        if deployment.config.server_id.is_empty() {
            return Err(anyhow!("deployment has no server configured"));
        }

        let (server, status) = self
            .get_server_with_status(&deployment.config.server_id)
            .await?;
        if status != ServerStatus::Ok {
            return Err(anyhow!(
                "cannot send action when server is unreachable or disabled"
            ));
        }

        let periphery = self.periphery_client(&server);

        let inner = || async move {
            let start_ts = monitor_timestamp();

            let mut update = Update {
                target: ResourceTarget::Deployment(deployment.id.clone()),
                operation: Operation::StartContainer,
                start_ts,
                status: UpdateStatus::InProgress,
                success: true,
                operator: user.id.clone(),
                ..Default::default()
            };

            update.id = self.add_update(update.clone()).await?;

            let log = match periphery
                .request(requests::StartContainer {
                    name: deployment.name.clone(),
                })
                .await
            {
                Ok(log) => log,
                Err(e) => Log::error("start container", format!("{e:#?}")),
            };

            update.logs.push(log);
            update.finalize();
            self.update_cache_for_server(&server).await;
            self.update_update(update.clone()).await?;

            Ok(update)
        };

        self.action_states
            .deployment
            .update_entry(deployment_id.to_string(), |entry| {
                entry.starting = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .deployment
            .update_entry(deployment_id, |entry| {
                entry.starting = false;
            })
            .await;

        res
    }
}

#[async_trait]
impl Resolve<StopContainer, RequestUser> for State {
    async fn resolve(
        &self,
        StopContainer {
            deployment_id,
            signal,
            time,
        }: StopContainer,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        if self.action_states.deployment.busy(&deployment_id).await {
            return Err(anyhow!("deployment busy"));
        }

        let deployment = self
            .get_deployment_check_permissions(&deployment_id, &user, PermissionLevel::Execute)
            .await?;

        if deployment.config.server_id.is_empty() {
            return Err(anyhow!("deployment has no server configured"));
        }

        let (server, status) = self
            .get_server_with_status(&deployment.config.server_id)
            .await?;
        if status != ServerStatus::Ok {
            return Err(anyhow!(
                "cannot send action when server is unreachable or disabled"
            ));
        }

        let periphery = self.periphery_client(&server);

        let inner = || async move {
            let start_ts = monitor_timestamp();

            let mut update = Update {
                target: ResourceTarget::Deployment(deployment.id.clone()),
                operation: Operation::StopContainer,
                start_ts,
                status: UpdateStatus::InProgress,
                success: true,
                operator: user.id.clone(),
                ..Default::default()
            };

            update.id = self.add_update(update.clone()).await?;

            let log = match periphery
                .request(requests::StopContainer {
                    name: deployment.name.clone(),
                    signal: signal
                        .unwrap_or(deployment.config.termination_signal)
                        .into(),
                    time: time.unwrap_or(deployment.config.termination_timeout).into(),
                })
                .await
            {
                Ok(log) => log,
                Err(e) => Log::error("stop container", format!("{e:#?}")),
            };

            update.logs.push(log);
            update.finalize();
            self.update_cache_for_server(&server).await;
            self.update_update(update.clone()).await?;

            Ok(update)
        };

        self.action_states
            .deployment
            .update_entry(deployment_id.to_string(), |entry| {
                entry.stopping = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .deployment
            .update_entry(deployment_id, |entry| {
                entry.stopping = false;
            })
            .await;

        res
    }
}

#[async_trait]
impl Resolve<RemoveContainer, RequestUser> for State {
    async fn resolve(
        &self,
        RemoveContainer {
            deployment_id,
            signal,
            time,
        }: RemoveContainer,
        user: RequestUser,
    ) -> anyhow::Result<Update> {
        if self.action_states.deployment.busy(&deployment_id).await {
            return Err(anyhow!("deployment busy"));
        }

        let deployment = self
            .get_deployment_check_permissions(&deployment_id, &user, PermissionLevel::Execute)
            .await?;

        if deployment.config.server_id.is_empty() {
            return Err(anyhow!("deployment has no server configured"));
        }

        let (server, status) = self
            .get_server_with_status(&deployment.config.server_id)
            .await?;
        if status != ServerStatus::Ok {
            return Err(anyhow!(
                "cannot send action when server is unreachable or disabled"
            ));
        }

        let periphery = self.periphery_client(&server);

        let inner = || async move {
            let start_ts = monitor_timestamp();

            let mut update = Update {
                target: ResourceTarget::Deployment(deployment.id.clone()),
                operation: Operation::RemoveContainer,
                start_ts,
                status: UpdateStatus::InProgress,
                success: true,
                operator: user.id.clone(),
                ..Default::default()
            };

            update.id = self.add_update(update.clone()).await?;

            let log = match periphery
                .request(requests::RemoveContainer {
                    name: deployment.name.clone(),
                    signal: signal
                        .unwrap_or(deployment.config.termination_signal)
                        .into(),
                    time: time.unwrap_or(deployment.config.termination_timeout).into(),
                })
                .await
            {
                Ok(log) => log,
                Err(e) => Log::error("stop container", format!("{e:#?}")),
            };

            update.logs.push(log);
            update.finalize();
            self.update_cache_for_server(&server).await;
            self.update_update(update.clone()).await?;

            Ok(update)
        };

        self.action_states
            .deployment
            .update_entry(deployment_id.to_string(), |entry| {
                entry.removing = true;
            })
            .await;

        let res = inner().await;

        self.action_states
            .deployment
            .update_entry(deployment_id, |entry| {
                entry.removing = false;
            })
            .await;

        res
    }
}
