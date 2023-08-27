use anyhow::anyhow;
use async_trait::async_trait;
use monitor_types::{
    entities::{
        build::Build,
        deployment::{Deployment, DeploymentImage},
        server::ServerStatus,
        update::{Log, ResourceTarget, Update, UpdateStatus},
        Operation, PermissionLevel, Version,
    },
    get_image_name, monitor_timestamp,
    requests::execute::*,
};
use periphery_client::requests;
use resolver_api::Resolve;

use crate::{auth::RequestUser, helpers::resource::StateResource, state::State};

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

        let mut deployment: Deployment = self
            .get_resource_check_permissions(&deployment_id, &user, PermissionLevel::Execute)
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

        let periphery = self.periphery_client(&server)?;

        let inner = || async move {
            let start_ts = monitor_timestamp();

            let version = match deployment.config.image {
                DeploymentImage::Build { build_id, version } => {
                    let build: Build = self.get_resource(&build_id).await?;
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

        let deployment: Deployment = self
            .get_resource_check_permissions(&deployment_id, &user, PermissionLevel::Execute)
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

        let periphery = self.periphery_client(&server)?;

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

        let deployment: Deployment = self
            .get_resource_check_permissions(&deployment_id, &user, PermissionLevel::Execute)
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

        let periphery = self.periphery_client(&server)?;

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

        let deployment: Deployment = self
            .get_resource_check_permissions(&deployment_id, &user, PermissionLevel::Execute)
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

        let periphery = self.periphery_client(&server)?;

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
