use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{routing::post, Extension, Json, Router};
use db::DbExtension;
use helpers::handle_anyhow_error;
use types::{EntityType, Operation, PermissionLevel, Server, Update};

use crate::{auth::RequestUserExtension, ws::update};

use super::add_update;

pub fn router() -> Router {
    Router::new().route(
        "/create",
        post(|db, user, update_ws, server| async {
            create(db, user, update_ws, server)
                .await
                .map_err(handle_anyhow_error)
        }),
    )
}

async fn create(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::WsSenderExtension,
    Json(mut server): Json<Server>,
) -> anyhow::Result<()> {
    server.permissions = [(user.id.clone(), PermissionLevel::Write)]
        .into_iter()
        .collect();
    let start_ts = unix_timestamp_ms() as i64;
    let server_id = db
        .servers
        .create_one(server)
        .await
        .context("failed to add server to db")?;
    let update = Update {
        entity_type: EntityType::Server,
        entity_id: Some(server_id),
        operation: Operation::CreateServer,
        start_ts,
        end_ts: unix_timestamp_ms() as i64,
        operator: user.id.clone(),
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await
}
