use std::str::FromStr;

use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{routing::post, Extension, Json, Router};
use db::DbExtension;
use helpers::handle_anyhow_error;
use mungos::ObjectId;
use types::{Build, EntityType, Operation, PermissionLevel, Update};

use crate::{auth::RequestUserExtension, ws::update};

use super::{add_update, PeripheryExtension};

pub fn router() -> Router {
    Router::new().route(
        "/create",
        post(|db, user, update_ws, build| async {
            create(db, user, update_ws, build)
                .await
                .map_err(handle_anyhow_error);
        }),
    )
}

async fn create(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::WsSenderExtension,
    Json(mut build): Json<Build>,
) -> anyhow::Result<()> {
    let build_server = db
        .servers
        .find_one_by_id(&build.server_id)
        .await
        .context("failed at query to find server")?
        .ok_or(anyhow!("did not find server with server_id given on build"))?;
    let permissions = *build_server.permissions.get(&user.id).ok_or(anyhow!(
        "user does not have permissions to create build on this server"
    ))?;
    if permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to create build on this server"
        ));
    }
    build.permissions = [(user.id.clone(), PermissionLevel::Write)]
        .into_iter()
        .collect();
    let start_ts = unix_timestamp_ms() as i64;
    let build_id = db.builds.create_one(build).await?;
    let update = Update {
        entity_type: EntityType::Build,
        entity_id: Some(build_id),
        operation: Operation::CreateBuild,
        start_ts,
        end_ts: unix_timestamp_ms() as i64,
        operator: user.id.clone(),
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await
}
