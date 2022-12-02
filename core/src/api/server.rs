use anyhow::{anyhow, Context};
use async_timing_util::unix_timestamp_ms;
use axum::{
    extract::Path,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use db::DbExtension;
use helpers::handle_anyhow_error;
use mungos::Deserialize;
use types::{
    traits::Permissioned, Log, Operation, PermissionLevel, Server, SystemStats, Update,
    UpdateTarget,
};

use crate::{auth::RequestUserExtension, ws::update};

use super::{add_update, PeripheryExtension};

#[derive(Deserialize)]
struct ServerId {
    id: String,
}

#[derive(Deserialize)]
struct CreateServerBody {
    name: String,
    address: String,
}

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
            "/delete/:id",
            delete(|db, user, update_ws, server_id| async {
                delete_one(db, user, update_ws, server_id)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .route(
            "/stats/:id",
            get(|db, user, periphery, server_id| async {
                stats(db, user, periphery, server_id)
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
                let permissions = s.get_user_permissions(&user.id);
                permissions != PermissionLevel::None
            }
        })
        .collect();
    servers.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(Json(servers))
}

impl Into<Server> for CreateServerBody {
    fn into(self) -> Server {
        Server {
            name: self.name,
            address: self.address,
            ..Default::default()
        }
    }
}

async fn create(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Json(server): Json<CreateServerBody>,
) -> anyhow::Result<()> {
    if !user.is_admin {
        return Err(anyhow!(
            "user does not have permissions to add server (not admin)"
        ));
    }
    let mut server: Server = server.into();
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

async fn delete_one(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Path(ServerId { id }): Path<ServerId>,
) -> anyhow::Result<()> {
    let server = db.get_server(&id).await?;
    let permissions = server.get_user_permissions(&user.id);
    if !user.is_admin && permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to delete server {} ({id})",
            server.name
        ));
    }
    let start_ts = unix_timestamp_ms() as i64;
    db.deployments.delete_one(&id).await?;
    let update = Update {
        target: UpdateTarget::System,
        operation: Operation::DeleteServer,
        start_ts,
        end_ts: Some(unix_timestamp_ms() as i64),
        operator: user.id.clone(),
        log: vec![Log::simple(format!("deleted server {}", server.name))],
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await
}

async fn stats(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(periphery): PeripheryExtension,
    Path(ServerId { id }): Path<ServerId>,
) -> anyhow::Result<Json<SystemStats>> {
    let server = db.get_server(&id).await?;
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
