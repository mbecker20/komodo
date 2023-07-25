use std::cmp;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use futures::future::join_all;
use monitor_types::{
    entities::{
        deployment::{
            Deployment, DeploymentActionState, DeploymentConfig, DeploymentImage,
            DockerContainerStats,
        },
        update::{Log, UpdateStatus},
        Operation, PermissionLevel,
    },
    permissioned::Permissioned,
    requests::read::*,
};
use mungos::mongodb::{bson::doc, options::FindOneOptions};
use periphery_client::requests;
use resolver_api::Resolve;

use crate::{auth::RequestUser, state::State};

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
    ) -> anyhow::Result<Vec<DeploymentListItem>> {
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

        let deployments = deployments.into_iter().map(|deployment| async {
            let status = self.deployment_status_cache.get(&deployment.id).await;
            DeploymentListItem {
                id: deployment.id,
                name: deployment.name,
                tags: deployment.tags,
                state: status.as_ref().map(|s| s.state).unwrap_or_default(),
                status: status
                    .as_ref()
                    .and_then(|s| s.container.as_ref().and_then(|c| c.status.to_owned())),
                image: String::new(),
                version: String::new(),
            }
        });

        let deployments = join_all(deployments).await;

        Ok(deployments)
    }
}

#[async_trait]
impl Resolve<GetDeploymentStatus, RequestUser> for State {
    async fn resolve(
        &self,
        GetDeploymentStatus { id }: GetDeploymentStatus,
        user: RequestUser,
    ) -> anyhow::Result<GetDeploymentStatusResponse> {
        self.get_deployment_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        let status = self
            .deployment_status_cache
            .get(&id)
            .await
            .unwrap_or_default();
        let response = GetDeploymentStatusResponse {
            status: status.container.as_ref().and_then(|c| c.status.clone()),
            state: status.state,
        };
        Ok(response)
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
impl Resolve<GetDeploymentActionState, RequestUser> for State {
    async fn resolve(
        &self,
        GetDeploymentActionState { id }: GetDeploymentActionState,
        user: RequestUser,
    ) -> anyhow::Result<DeploymentActionState> {
        self.get_deployment_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        let action_state = self
            .action_states
            .deployment
            .get(&id)
            .await
            .unwrap_or_default();
        Ok(action_state)
    }
}

#[async_trait]
impl Resolve<GetDeploymentsSummary, RequestUser> for State {
    async fn resolve(
        &self,
        GetDeploymentsSummary {}: GetDeploymentsSummary,
        user: RequestUser,
    ) -> anyhow::Result<GetDeploymentsSummaryResponse> {
        todo!()
    }
}
