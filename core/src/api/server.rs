use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use db::DbExtension;
use helpers::handle_anyhow_error;
use types::{Operation, PermissionLevel, Server, Update, UpdateTarget};

use crate::{auth::RequestUserExtension, ws::update};

use super::add_update;

pub fn router() -> Router {
    Router::new()
        .route(
            "/list",
            get(|db, user| async { list(db, user).await.map_err(handle_anyhow_error) }),
        )
        .route(
            "/create",
            post(|db, user, update_ws, server| async {
                create(db, user, update_ws, server)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
}

async fn list(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
) -> anyhow::Result<Json<Vec<Server>>> {
    let mut servers: Vec<Server> = db
        .servers
        .get_some(None, None)
        .await
        .context("failed at get all servers query")?
        .into_iter()
        .filter(|s| {
            if user.is_admin {
                true
            } else {
                let permissions = *s
                    .permissions
                    .get(&user.id)
                    .unwrap_or(&PermissionLevel::None);
                permissions != PermissionLevel::None
            }
        })
        .collect();
    servers.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(Json(servers))
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
        target: UpdateTarget::Server(server_id),
        operation: Operation::CreateServer,
        start_ts,
        end_ts: unix_timestamp_ms() as i64,
        operator: user.id.clone(),
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await
}
