use std::str::FromStr;

use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{Extension, Json, Router};
use db::DbExtension;
use mungos::ObjectId;
use types::{Build, EntityType, Operation, PermissionLevel, Update};

use crate::{auth::RequestUserExtension, ws::update};

use super::PeripheryExtension;

pub fn router() -> Router {
    Router::new()
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
    let mut update = Update {
        entity_type: EntityType::Build,
        entity_id: Some(build_id),
        operation: Operation::CreateBuild,
        start_ts,
        end_ts: unix_timestamp_ms() as i64,
        ..Default::default()
    };
    let update_id = db
        .updates
        .create_one(update.clone())
        .await
        .context("failed to insert update into db. the create build process was completed.")?;
	update.id = Some(ObjectId::from_str(&update_id).context("failed at attaching update id")?);
    let update_msg = serde_json::to_string(&update).unwrap();
    let _ = update_ws.lock().await.send((update, update_msg));
    Ok(())
}
