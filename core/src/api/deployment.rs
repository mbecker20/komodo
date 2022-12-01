use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{routing::post, Extension, Json, Router};
use db::DbExtension;
use helpers::handle_anyhow_error;
use mungos::Deserialize;
use types::{
    traits::Permissioned, Deployment, Log, Operation, PermissionLevel, Update, UpdateTarget,
};

use crate::{auth::RequestUserExtension, ws::update};

use super::{add_update, PeripheryExtension};

#[derive(Deserialize)]
pub struct DeploymentIdBody {
    id: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/create",
            post(|db, user, update_ws, deployment| async {
                create(db, user, update_ws, deployment)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .route(
            "/delete",
            post(|db, user, update_ws, periphery, deployment_id| async {
                delete(db, user, update_ws, periphery, deployment_id)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
}

async fn create(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Json(mut deployment): Json<Deployment>,
) -> anyhow::Result<()> {
    deployment.permissions = [(user.id.clone(), PermissionLevel::Write)]
        .into_iter()
        .collect();
    let start_ts = unix_timestamp_ms() as i64;
    let deployment_id = db
        .deployments
        .create_one(deployment)
        .await
        .context("failed to add server to db")?;
    let update = Update {
        target: UpdateTarget::Deployment(deployment_id),
        operation: Operation::CreateDeployment,
        start_ts,
        end_ts: Some(unix_timestamp_ms() as i64),
        operator: user.id.clone(),
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await
}

async fn delete(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Extension(periphery): PeripheryExtension,
    Json(DeploymentIdBody { id }): Json<DeploymentIdBody>,
) -> anyhow::Result<()> {
    let deployment = db.get_deployment(&id).await?;
    let permissions = deployment.get_user_permissions(&user.id);
    if permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to delete deployment {id}"
        ));
    }
    let server = db.get_server(&deployment.server_id).await?;
    let start_ts = unix_timestamp_ms() as i64;
    let log = periphery
        .container_remove(&server, &deployment.name)
        .await?;
    db.deployments.delete_one(&id).await?;
    let update = Update {
        target: UpdateTarget::System,
        operation: Operation::DeleteDeployment,
        start_ts,
        end_ts: Some(unix_timestamp_ms() as i64),
        operator: user.id.clone(),
        log: vec![
            log,
            Log::simple(format!(
                "deleted deployment {} on server {}",
                deployment.name, server.name
            )),
        ],
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await
}
