use anyhow::Context;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use helpers::handle_anyhow_error;
use mungos::{Deserialize, Document};
use types::{traits::Permissioned, Deployment, PermissionLevel};

use crate::{
    auth::{RequestUser, RequestUserExtension},
    response,
    state::{State, StateExtension},
};

#[derive(Deserialize)]
pub struct DeploymentId {
    id: String,
}

#[derive(Deserialize)]
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
                    let deployment = state
                        .get_deployment_check_permissions(
                            &deployment_id.id,
                            &user,
                            PermissionLevel::Read,
                        )
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(deployment))
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
                        .list_deployments(&user, query)
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
            "/delete/:id",
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
            "/deploy/:id",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(deployment_id): Path<DeploymentId>| async move {
                    let update = state
                        .deploy(&deployment_id.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(update))
                },
            ),
        )
        .route(
            "/reclone/:id",
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
}

impl State {
    async fn list_deployments(
        &self,
        user: &RequestUser,
        query: impl Into<Option<Document>>,
    ) -> anyhow::Result<Vec<Deployment>> {
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
        deployments.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        Ok(deployments)
    }
}
