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
    traits::Permissioned, Build, Log, Operation, PermissionLevel, Update, UpdateStatus,
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
            "/list",
            get(|db, user| async { list(db, user).await.map_err(handle_anyhow_error) }),
        )
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
        .route(
            "/reclone/:id",
            post(|db, user, update_ws, periphery, build_id| async {
                reclone(db, user, update_ws, periphery, build_id)
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

async fn list(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
) -> anyhow::Result<Json<Vec<Build>>> {
    let mut builds: Vec<Build> = db
        .builds
        .get_some(None, None)
        .await
        .context("failed at get all builds query")?
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
    builds.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(Json(builds))
}

async fn create(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Json(build): Json<CreateBuildBody>,
) -> anyhow::Result<Json<Build>> {
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
    let build = db.get_build(&build_id).await?;
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
    Ok(Json(build))
}

async fn delete_one(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Extension(periphery): PeripheryExtension,
    Path(BuildId { id }): Path<BuildId>,
) -> anyhow::Result<Json<Build>> {
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
    Ok(Json(build))
}

async fn update(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Extension(periphery): PeripheryExtension,
    Json(mut new_build): Json<Build>,
) -> anyhow::Result<Json<Build>> {
    let current_build = db.get_build(&new_build.id).await?;
    let permissions = current_build.get_user_permissions(&user.id);
    if !user.is_admin && permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to update build {} ({})",
            current_build.name,
            current_build.id
        ));
    }

    new_build.permissions = current_build.permissions.clone();

    db.builds
        .update_one(&new_build.id, mungos::Update::Regular(new_build.clone()))
        .await
        .context("failed at update one build")?;

    let diff = current_build.diff(&new_build);

    let mut update = Update {
        operation: Operation::UpdateBuild,
        target: UpdateTarget::Build(new_build.id.clone()),
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
        let server = db.get_server(&current_build.server_id).await?;
        match periphery.clone_repo(&server, &new_build).await {
            Ok(clone_logs) => {
                update.logs.extend(clone_logs);
            }
            Err(e) => update
                .logs
                .push(Log::error("cloning repo", format!("{e:#?}"))),
        }
    }

    update.end_ts = Some(unix_timestamp_ms() as i64);
    update.success = all_logs_success(&update.logs);
    update.status = UpdateStatus::Complete;

    update_update(update, &db, &update_ws).await?;
    Ok(Json(new_build))
}

async fn reclone(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Extension(periphery): PeripheryExtension,
    Path(BuildId { id }): Path<BuildId>,
) -> anyhow::Result<Json<Update>> {
    let update = reclone_build(&id, &user, &db, &periphery, update_ws).await?;
    Ok(Json(update))
}

pub async fn reclone_build(
    build_id: &str,
    user: &RequestUser,
    db: &DbClient,
    periphery: &PeripheryClient,
    update_ws: update::UpdateWsSender,
) -> anyhow::Result<Update> {
    let build = db.get_build(build_id).await?;
    let permissions = build.get_user_permissions(&user.id);
    if !user.is_admin && permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to reclone build {} ({})",
            build.name,
            build.id
        ));
    }
    let server = db.get_server(&build.server_id).await?;
    let mut update = Update {
        target: UpdateTarget::Build(build_id.to_string()),
        operation: Operation::RecloneBuild,
        start_ts: unix_timestamp_ms() as i64,
        status: UpdateStatus::InProgress,
        operator: user.id.clone(),
        success: true,
        ..Default::default()
    };
    update.id = add_update(update.clone(), &db, &update_ws).await?;

    update.success = match periphery.clone_repo(&server, &build).await {
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
    update.end_ts = Some(unix_timestamp_ms() as i64);

    update_update(update.clone(), &db, &update_ws).await?;

    Ok(update)
}
