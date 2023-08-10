use std::cmp;

use anyhow::{anyhow, Context};
use async_trait::async_trait;
use monitor_types::{
    entities::{
        deployment::{
            Deployment, DeploymentActionState, DeploymentConfig, DeploymentImage,
            DockerContainerState, DockerContainerStats, DeploymentListItem,
        },
        server::Server,
        update::{Log, UpdateStatus},
        Operation, PermissionLevel,
    },
    requests::read::*,
};
use mungos::mongodb::{bson::doc, options::FindOneOptions};
use periphery_client::requests;
use resolver_api::Resolve;

use crate::{auth::RequestUser, resource::StateResource, state::State};

#[async_trait]
impl Resolve<GetDeployment, RequestUser> for State {
    async fn resolve(
        &self,
        GetDeployment { id }: GetDeployment,
        user: RequestUser,
    ) -> anyhow::Result<Deployment> {
        self.get_resource_check_permissions(&id, &user, PermissionLevel::Read)
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
        <State as StateResource<Deployment>>::list_resources_for_user(self, query, &user).await
    }
}

#[async_trait]
impl Resolve<GetDeploymentStatus, RequestUser> for State {
    async fn resolve(
        &self,
        GetDeploymentStatus { id }: GetDeploymentStatus,
        user: RequestUser,
    ) -> anyhow::Result<GetDeploymentStatusResponse> {
        let _: Deployment = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Read)
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
            .get_resource_check_permissions(&deployment_id, &user, PermissionLevel::Read)
            .await?;
        if server_id.is_empty() {
            return Ok(Log::default());
        }
        let server: Server = self.get_resource(&server_id).await?;
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
            .get_resource_check_permissions(&deployment_id, &user, PermissionLevel::Read)
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
            .get_resource_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        if server_id.is_empty() {
            return Err(anyhow!("deployment has no server attached"));
        }
        let server: Server = self.get_resource(&server_id).await?;
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
        let _: Deployment = self
            .get_resource_check_permissions(&id, &user, PermissionLevel::Read)
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
        let query = if user.is_admin {
            None
        } else {
            let query = doc! {
                format!("permissions.{}", user.id): { "$in": ["read", "execute", "update"] }
            };
            Some(query)
        };
        let deployments = self
            .db
            .deployments
            .get_some(query, None)
            .await
            .context("failed to count all deployment documents")?;
        let mut res = GetDeploymentsSummaryResponse::default();
        for deployment in deployments {
            res.total += 1;
            let status = self
                .deployment_status_cache
                .get(&deployment.id)
                .await
                .unwrap_or_default();
            match status.state {
                DockerContainerState::Running => {
                    res.running += 1;
                }
                DockerContainerState::Unknown => {
                    res.unknown += 1;
                }
                DockerContainerState::NotDeployed => {
                    res.not_deployed += 1;
                }
                _ => {
                    res.stopped += 1;
                }
            }
        }
        Ok(res)
    }
}
