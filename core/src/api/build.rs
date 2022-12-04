use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{
    extract::Path,
    routing::{delete, post},
    Extension, Json, Router,
};
use db::DbExtension;
use helpers::handle_anyhow_error;
use mungos::Deserialize;
use types::{traits::Permissioned, Build, Log, Operation, PermissionLevel, Update, UpdateTarget};

use crate::{auth::RequestUserExtension, ws::update};

use super::{add_update, PeripheryExtension};

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
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await
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
    if permissions != PermissionLevel::Write {
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
        log: vec![
            delete_repo_log,
            Log::simple(format!(
                "deleted build {} on server {}",
                build.name, server.name
            )),
        ],
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await
}
