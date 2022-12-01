use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{
    extract::Path,
    routing::{get, post},
    Extension, Json, Router,
};
use db::DbExtension;
use helpers::handle_anyhow_error;
use mungos::Deserialize;
use types::{
    traits::Permissioned, Operation, PermissionLevel, Server, SystemStats, Update, UpdateTarget,
};

use crate::{auth::RequestUserExtension, ws::update};

use super::{add_update, PeripheryExtension};

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
        .route(
            "/stats/:server_id",
            get(|db, user, periphery, path| async {
                stats(db, user, periphery, path)
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
    Extension(update_ws): update::UpdateWsSenderExtension,
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
        end_ts: Some(unix_timestamp_ms() as i64),
        operator: user.id.clone(),
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await
}

#[derive(Deserialize)]
struct GetStatsPath {
    server_id: String,
}

async fn stats(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(periphery): PeripheryExtension,
    Path(GetStatsPath { server_id }): Path<GetStatsPath>,
) -> anyhow::Result<Json<SystemStats>> {
    let server = db
        .servers
        .find_one_by_id(&server_id)
        .await
        .context("failed at query to get server")?
        .ok_or(anyhow!("failed to find server with id {server_id}"))?;
    let permissions = server.get_user_permissions(&user.id);
    if permissions == PermissionLevel::None {
        return Err(anyhow!("user does not have permissions on this server"));
    }
    let stats = periphery
        .get_system_stats(&server)
        .await
        .context(format!("failed to get stats from server {}", server.name))?;
    Ok(Json(stats))
}
