use anyhow::Context;
use axum::{
    extract::Path,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use helpers::handle_anyhow_error;
use mungos::Deserialize;
use types::{traits::Permissioned, Deployment, PermissionLevel};

use crate::{auth::RequestUserExtension, response, state::StateExtension};

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
            "/list",
            get(|state, user| async { list(state, user).await.map_err(handle_anyhow_error) }),
        )
        .route(
            "/create",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(deployment): Json<CreateDeploymentBody>| async move {
                    let deployment = state
                        .create_deployment(deployment.name, deployment.server_id, &user)
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

async fn list(
    Extension(state): StateExtension,
    Extension(user): RequestUserExtension,
) -> anyhow::Result<Json<Vec<Deployment>>> {
    let mut deployments: Vec<Deployment> = state
        .db
        .deployments
        .get_some(None, None)
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
    Ok(Json(deployments))
}
