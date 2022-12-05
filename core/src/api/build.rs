use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{
    extract::Path,
    routing::{delete, patch, post},
    Extension, Json, Router,
};
use db::DbExtension;
use diff::Diff;
use helpers::handle_anyhow_error;
use mungos::Deserialize;
use types::{traits::Permissioned, Build, Log, Operation, PermissionLevel, Update, UpdateTarget};

use crate::{
    auth::RequestUserExtension,
    helpers::{add_update, all_logs_success, any_option_diff_is_some, option_diff_is_some},
    ws::update,
};

use super::PeripheryExtension;

#[derive(Deserialize)]
struct BuildId {
    id: String,
}

#[derive(Deserialize)]
struct CreateBuildBody {
    name: String,
    server_id: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/create",
            post(|db, user, update_ws, build| async {
                create(db, user, update_ws, build)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .route(
            "/delete/:id",
            delete(|db, user, update_ws, periphery, build_id| async {
                delete_one(db, user, update_ws, periphery, build_id)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .route(
            "/update",
            patch(|db, user, update_ws, periphery, build| async {
                update(db, user, update_ws, periphery, build)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
}

impl Into<Build> for CreateBuildBody {
    fn into(self) -> Build {
        Build {
            name: self.name,
            server_id: self.server_id,
            ..Default::default()
        }
    }
}

async fn create(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Json(build): Json<CreateBuildBody>,
) -> anyhow::Result<()> {
    let server = db.get_server(&build.server_id).await?;
    let permissions = server.get_user_permissions(&user.id);
    if !user.is_admin && permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to create build on this server"
        ));
    }
    let mut build: Build = build.into();
    build.permissions = [(user.id.clone(), PermissionLevel::Write)]
        .into_iter()
        .collect();
    let start_ts = unix_timestamp_ms() as i64;
    let build_id = db.builds.create_one(build).await?;
    let update = Update {
        target: UpdateTarget::Build(build_id),
        operation: Operation::CreateBuild,
        start_ts,
        end_ts: Some(unix_timestamp_ms() as i64),
        operator: user.id.clone(),
        success: true,
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await?;
    Ok(())
}

async fn delete_one(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Extension(periphery): PeripheryExtension,
    Path(BuildId { id }): Path<BuildId>,
) -> anyhow::Result<()> {
    let build = db.get_build(&id).await?;
    let permissions = build.get_user_permissions(&user.id);
    if !user.is_admin && permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to delete build {} ({id})",
            build.name
        ));
    }
    let start_ts = unix_timestamp_ms() as i64;
    let server = db.get_server(&build.server_id).await?;
    let delete_repo_log = periphery
        .delete_repo(&server, &build.name)
        .await
        .context("failed at deleting repo")?;
    db.builds.delete_one(&id).await?;
    let update = Update {
        target: UpdateTarget::System,
        operation: Operation::DeleteDeployment,
        start_ts,
        end_ts: Some(unix_timestamp_ms() as i64),
        operator: user.id.clone(),
        logs: vec![
            delete_repo_log,
            Log::simple(format!(
                "deleted build {} on server {}",
                build.name, server.name
            )),
        ],
        success: true,
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await?;
    Ok(())
}

async fn update(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Extension(periphery): PeripheryExtension,
    Json(mut new_build): Json<Build>,
) -> anyhow::Result<()> {
    let current_build = db.get_build(&new_build.id).await?;
    let permissions = current_build.get_user_permissions(&user.id);
    if !user.is_admin && permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to update build {} ({})",
            current_build.name,
            current_build.id
        ));
    }
    let start_ts = unix_timestamp_ms() as i64;
    let server = db.get_server(&current_build.server_id).await?;

    new_build.permissions = current_build.permissions.clone();
    let diff = current_build.diff(&new_build);
    let mut logs = vec![Log::simple(format!("{diff:#?}"))];

    if any_option_diff_is_some(&[&diff.repo, &diff.branch, &diff.github_account])
        || option_diff_is_some(&diff.on_clone)
    {
        match periphery.clone_repo(&server, &new_build).await {
            Ok(clone_logs) => {
                logs.extend(clone_logs);
            }
            Err(e) => logs.push(Log::error("cloning repo", format!("{e:#?}"))),
        }
    }

    let update = Update {
        operation: Operation::UpdateBuild,
        target: UpdateTarget::Build(new_build.id),
        start_ts,
        end_ts: Some(unix_timestamp_ms() as i64),
        success: all_logs_success(&logs),
        logs,
        operator: user.id.clone(),
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await?;
    Ok(())
}
