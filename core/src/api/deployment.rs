use std::collections::HashMap;

use anyhow::Context;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use futures_util::future::join_all;
use helpers::handle_anyhow_error;
use mungos::{Deserialize, Document, Serialize};
use types::{traits::Permissioned, Deployment, DeploymentWithContainer, PermissionLevel, Server};
use typeshare::typeshare;

use crate::{
    auth::{RequestUser, RequestUserExtension},
    response,
    state::{State, StateExtension},
};

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct DeploymentId {
    id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct CreateDeploymentBody {
    name: String,
    server_id: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/:id",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(deployment_id): Path<DeploymentId>| async move {
                    let res = state
                        .get_deployment_with_container(&user, &deployment_id.id)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(res))
                },
            ),
        )
        .route(
            "/list",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Query(query): Query<Document>| async move {
                    let deployments = state
                        .list_deployments_with_container_info(&user, query)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(deployments))
                },
            ),
        )
        .route(
            "/create",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(deployment): Json<CreateDeploymentBody>| async move {
                    let deployment = state
                        .create_deployment(&deployment.name, deployment.server_id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(deployment))
                },
            ),
        )
        .route(
            "/create_full",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(full_deployment): Json<Deployment>| async move {
                    let deployment = state
                        .create_full_deployment(full_deployment, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(deployment))
                },
            ),
        )
        .route(
            "/:id/delete",
            delete(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(deployment_id): Path<DeploymentId>| async move {
                    let deployment = state
                        .delete_deployment(&deployment_id.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(deployment))
                },
            ),
        )
        .route(
            "/update",
            patch(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(deployment): Json<Deployment>| async move {
                    let deployment = state
                        .update_deployment(deployment, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(deployment))
                },
            ),
        )
        .route(
            "/:id/reclone",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(deployment_id): Path<DeploymentId>| async move {
                    let update = state
                        .reclone_deployment(&deployment_id.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(update))
                },
            ),
        )
        .route(
            "/:id/deploy",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(deployment_id): Path<DeploymentId>| async move {
                    let update = state
                        .deploy_container(&deployment_id.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(update))
                },
            ),
        )
        .route(
            "/:id/start_container",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(deployment_id): Path<DeploymentId>| async move {
                    let update = state
                        .start_container(&deployment_id.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(update))
                },
            ),
        )
        .route(
            "/:id/stop_container",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(deployment_id): Path<DeploymentId>| async move {
                    let update = state
                        .stop_container(&deployment_id.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(update))
                },
            ),
        )
        .route(
            "/:id/remove_container",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(deployment_id): Path<DeploymentId>| async move {
                    let update = state
                        .remove_container(&deployment_id.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(update))
                },
            ),
        )
}

impl State {
    async fn get_deployment_with_container(
        &self,
        user: &RequestUser,
        id: &str,
    ) -> anyhow::Result<DeploymentWithContainer> {
        let deployment = self
            .get_deployment_check_permissions(id, user, PermissionLevel::Read)
            .await?;
        let server = self.db.get_server(&deployment.server_id).await?;
        let container = match self.periphery.container_list(&server).await {
            Ok(containers) => containers.into_iter().find(|c| c.name == deployment.name),
            Err(_) => None,
        };
        Ok(DeploymentWithContainer {
            deployment,
            container,
        })
    }

    async fn list_deployments_with_container_info(
        &self,
        user: &RequestUser,
        query: impl Into<Option<Document>>,
    ) -> anyhow::Result<Vec<DeploymentWithContainer>> {
        let mut deployments: Vec<Deployment> = self
            .db
            .deployments
            .get_some(query, None)
            .await
            .context("failed at get all deployments query")?
            .into_iter()
            .filter(|s| {
                if user.is_admin {
                    true
                } else {
                    let permissions = s.get_user_permissions(&user.id);
                    permissions != PermissionLevel::None
                }
            })
            .collect();
        let mut servers: Vec<Server> = Vec::new();
        for d in &deployments {
            if servers.iter().find(|s| s.id == d.server_id).is_none() {
                servers.push(self.db.get_server(&d.server_id).await?)
            }
        }
        let containers_futures = servers
            .into_iter()
            .map(|server| async { (self.periphery.container_list(&server).await, server.id) });

        let containers = join_all(containers_futures)
            .await
            .into_iter()
            .map(|(container, server_id)| (server_id, container.ok()))
            .collect::<HashMap<_, _>>();
        deployments.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        let res = deployments
            .into_iter()
            .map(|deployment| {
                let container = match containers.get(&deployment.server_id).unwrap() {
                    Some(container) => container
                        .iter()
                        .find(|c| c.name == deployment.name)
                        .map(|c| c.to_owned()),
                    None => None,
                };
                DeploymentWithContainer {
                    container,
                    deployment,
                }
            })
            .collect();
        Ok(res)
    }
}
