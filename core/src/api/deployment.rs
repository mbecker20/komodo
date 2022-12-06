use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{
    extract::Path,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use db::{DbClient, DbExtension};
use diff::Diff;
use helpers::handle_anyhow_error;
use mungos::Deserialize;
use periphery::PeripheryClient;
use types::{
    traits::Permissioned, Deployment, Log, Operation, PermissionLevel, Update, UpdateStatus,
    UpdateTarget,
};

use crate::{
    auth::{RequestUser, RequestUserExtension},
    helpers::{
        add_update, all_logs_success, any_option_diff_is_some, option_diff_is_some, update_update,
    },
    ws::update,
};

use super::PeripheryExtension;

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
            get(|db, user| async { list(db, user).await.map_err(handle_anyhow_error) }),
        )
        .route(
            "/create",
            post(|db, user, update_ws, deployment| async {
                create(db, user, update_ws, deployment)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .route(
            "/delete/:id",
            delete(|db, user, update_ws, periphery, deployment_id| async {
                delete_one(db, user, update_ws, periphery, deployment_id)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .route(
            "/update",
            patch(|db, user, periphery, update_ws, new_deployment| async {
                update(db, user, periphery, update_ws, new_deployment)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .route(
            "/reclone/:id",
            post(|db, user, update_ws, periphery, deployment_id| async {
                reclone(db, user, update_ws, periphery, deployment_id)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
}

impl Into<Deployment> for CreateDeploymentBody {
    fn into(self) -> Deployment {
        Deployment {
            name: self.name,
            server_id: self.server_id,
            ..Default::default()
        }
    }
}

async fn list(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
) -> anyhow::Result<Json<Vec<Deployment>>> {
    let mut deployments: Vec<Deployment> = db
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

async fn create(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Json(deployment): Json<CreateDeploymentBody>,
) -> anyhow::Result<Json<Deployment>> {
    let server = db.get_server(&deployment.server_id).await?;
    let permissions = server.get_user_permissions(&user.id);
    if permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to create deployment on this server"
        ));
    }
    let mut deployment: Deployment = deployment.into();
    deployment.permissions = [(user.id.clone(), PermissionLevel::Write)]
        .into_iter()
        .collect();
    let start_ts = unix_timestamp_ms() as i64;
    let deployment_id = db
        .deployments
        .create_one(deployment)
        .await
        .context("failed to add server to db")?;
    let deployment = db.get_deployment(&deployment_id).await?;
    let update = Update {
        target: UpdateTarget::Deployment(deployment_id),
        operation: Operation::CreateDeployment,
        start_ts,
        end_ts: Some(unix_timestamp_ms() as i64),
        operator: user.id.clone(),
        success: true,
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await?;
    Ok(Json(deployment))
}

async fn delete_one(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Extension(periphery): PeripheryExtension,
    Path(DeploymentId { id }): Path<DeploymentId>,
) -> anyhow::Result<Json<Deployment>> {
    let deployment = db.get_deployment(&id).await?;
    let permissions = deployment.get_user_permissions(&user.id);
    if permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to delete deployment {} ({id})",
            deployment.name
        ));
    }
    let start_ts = unix_timestamp_ms() as i64;
    let server = db.get_server(&deployment.server_id).await?;
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
        logs: vec![
            log,
            Log::simple(format!(
                "deleted deployment {} on server {}",
                deployment.name, server.name
            )),
        ],
        success: true,
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await?;
    Ok(Json(deployment))
}

async fn update(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(periphery): PeripheryExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Json(new_deployment): Json<Deployment>,
) -> anyhow::Result<Json<Deployment>> {
    let deployment = update_deployment(&user, new_deployment, &db, &periphery, update_ws).await?;
    Ok(Json(deployment))
}

pub async fn update_deployment(
    user: &RequestUser,
    mut new_deployment: Deployment,
    db: &DbClient,
    periphery: &PeripheryClient,
    update_ws: update::UpdateWsSender,
) -> anyhow::Result<Deployment> {
    let current_deployment = db.get_deployment(&new_deployment.id).await?;
    let permissions = current_deployment.get_user_permissions(&user.id);
    if !user.is_admin && permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to update deployment {} ({})",
            current_deployment.name,
            current_deployment.id
        ));
    }

    new_deployment.permissions = current_deployment.permissions.clone();
    let diff = current_deployment.diff(&new_deployment);

    let mut update = Update {
        operation: Operation::UpdateDeployment,
        target: UpdateTarget::Deployment(new_deployment.id.clone()),
        start_ts: unix_timestamp_ms() as i64,
        status: UpdateStatus::InProgress,
        logs: vec![Log::simple(serde_json::to_string_pretty(&diff).unwrap())],
        operator: user.id.clone(),
        success: true,
        ..Default::default()
    };

    update.id = add_update(update.clone(), &db, &update_ws).await?;

    if any_option_diff_is_some(&[&diff.repo, &diff.branch, &diff.github_account])
        || option_diff_is_some(&diff.on_clone)
    {
        let server = db.get_server(&current_deployment.server_id).await?;
        match periphery.clone_repo(&server, &new_deployment).await {
            Ok(clone_logs) => {
                update.logs.extend(clone_logs);
            }
            Err(e) => update
                .logs
                .push(Log::error("cloning repo", format!("{e:#?}"))),
        }
    }

    db.deployments
        .update_one(
            &new_deployment.id,
            mungos::Update::Regular(new_deployment.clone()),
        )
        .await
        .context("failed at update one deployment")?;

    update.end_ts = Some(unix_timestamp_ms() as i64);
    update.success = all_logs_success(&update.logs);
    update.status = UpdateStatus::Complete;

    update_update(update, &db, &update_ws).await?;
    Ok(new_deployment)
}

async fn reclone(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Extension(periphery): PeripheryExtension,
    Path(DeploymentId { id }): Path<DeploymentId>,
) -> anyhow::Result<Json<Update>> {
    let update = reclone_deployment(&id, &user, &db, &periphery, update_ws).await?;
    Ok(Json(update))
}

pub async fn reclone_deployment(
    deployment_id: &str,
    user: &RequestUser,
    db: &DbClient,
    periphery: &PeripheryClient,
    update_ws: update::UpdateWsSender,
) -> anyhow::Result<Update> {
    let deployment = db.get_deployment(deployment_id).await?;
    let permissions = deployment.get_user_permissions(&user.id);
    if !user.is_admin && permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to reclone deployment {} ({})",
            deployment.name,
            deployment.id
        ));
    }
    let server = db.get_server(&deployment.server_id).await?;
    let mut update = Update {
        target: UpdateTarget::Deployment(deployment_id.to_string()),
        operation: Operation::RecloneDeployment,
        start_ts: unix_timestamp_ms() as i64,
        status: UpdateStatus::InProgress,
        operator: user.id.clone(),
        success: true,
        ..Default::default()
    };
    update.id = add_update(update.clone(), &db, &update_ws).await?;

    update.success = match periphery.clone_repo(&server, &deployment).await {
        Ok(clone_logs) => {
            update.logs.extend(clone_logs);
            true
        }
        Err(e) => {
            update
                .logs
                .push(Log::error("clone repo", format!("{e:#?}")));
            false
        }
    };

    update.status = UpdateStatus::Complete;

    update_update(update.clone(), &db, &update_ws).await?;

    Ok(update)
}
