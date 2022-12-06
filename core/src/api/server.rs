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
    traits::Permissioned, Log, Operation, PermissionLevel, Server, SystemStats, Update,
    UpdateStatus, UpdateTarget,
};

use crate::{
    auth::{RequestUser, RequestUserExtension},
    helpers::add_update,
    ws::update,
};

use super::PeripheryExtension;

#[derive(Deserialize)]
struct ServerId {
    id: String,
}

#[derive(Deserialize)]
pub struct CreateServerBody {
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
            "/update",
            patch(|db, user, update_ws, new_server| async {
                update(db, user, update_ws, new_server)
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
) -> anyhow::Result<Json<Server>> {
    let server = create_server(&user, server, &db, update_ws).await?;
    Ok(Json(server))
}

pub async fn create_server(
    user: &RequestUser,
    server: CreateServerBody,
    db: &DbClient,
    update_ws: update::UpdateWsSender,
) -> anyhow::Result<Server> {
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
    let server = db.get_server(&server_id).await?;
    let update = Update {
        target: UpdateTarget::Server(server_id),
        operation: Operation::CreateServer,
        start_ts,
        end_ts: Some(unix_timestamp_ms() as i64),
        operator: user.id.clone(),
        success: true,
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await?;
    Ok(server)
}

async fn delete_one(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Path(ServerId { id }): Path<ServerId>,
) -> anyhow::Result<Json<Server>> {
    let server = delete_server(&user, &id, &db, update_ws).await?;
    Ok(Json(server))
}

pub async fn delete_server(
    user: &RequestUser,
    server_id: &str,
    db: &DbClient,
    update_ws: update::UpdateWsSender,
) -> anyhow::Result<Server> {
    let server = db.get_server(server_id).await?;
    let permissions = server.get_user_permissions(&user.id);
    if !user.is_admin && permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to delete server {} ({server_id})",
            server.name
        ));
    }
    let start_ts = unix_timestamp_ms() as i64;
    db.deployments.delete_one(&server_id).await?;
    let update = Update {
        target: UpdateTarget::System,
        operation: Operation::DeleteServer,
        start_ts,
        end_ts: Some(unix_timestamp_ms() as i64),
        operator: user.id.clone(),
        logs: vec![Log::simple(format!("deleted server {}", server.name))],
        success: true,
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await?;
    Ok(server)
}

async fn update(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Json(new_server): Json<Server>,
) -> anyhow::Result<Json<Server>> {
    let server = update_server(&user, new_server, &db, update_ws).await?;
    Ok(Json(server))
}

pub async fn update_server(
    user: &RequestUser,
    mut new_server: Server,
    db: &DbClient,
    update_ws: update::UpdateWsSender,
) -> anyhow::Result<Server> {
    let current_server = db.get_server(&new_server.id).await?;
    let permissions = current_server.get_user_permissions(&user.id);
    if !user.is_admin && permissions != PermissionLevel::Write {
        return Err(anyhow!(
            "user does not have permissions to update server {} ({})",
            current_server.name,
            current_server.id
        ));
    }
    let start_ts = unix_timestamp_ms() as i64;

    new_server.permissions = current_server.permissions.clone();
    let diff = current_server.diff(&new_server);

    db.servers
        .update_one(&new_server.id, mungos::Update::Regular(new_server.clone()))
        .await
        .context("failed at update one server")?;

    let update = Update {
        operation: Operation::UpdateServer,
        target: UpdateTarget::Server(new_server.id.clone()),
        start_ts,
        end_ts: Some(unix_timestamp_ms() as i64),
        status: UpdateStatus::Complete,
        logs: vec![Log::simple(serde_json::to_string_pretty(&diff).unwrap())],
        operator: user.id.clone(),
        success: true,
        ..Default::default()
    };

    add_update(update, &db, &update_ws).await?;
    Ok(new_server)
}

async fn stats(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(periphery): PeripheryExtension,
    Path(ServerId { id }): Path<ServerId>,
) -> anyhow::Result<Json<SystemStats>> {
    let stats = get_server_stats(&user, &id, &db, &periphery).await?;
    Ok(Json(stats))
}

pub async fn get_server_stats(
    user: &RequestUser,
    server_id: &str,
    db: &DbClient,
    periphery: &PeripheryClient,
) -> anyhow::Result<SystemStats> {
    let server = db.get_server(server_id).await?;
    let permissions = server.get_user_permissions(&user.id);
    if !user.is_admin && permissions == PermissionLevel::None {
        return Err(anyhow!("user does not have permissions on this server"));
    }
    let stats = periphery
        .get_system_stats(&server)
        .await
        .context(format!("failed to get stats from server {}", server.name))?;
    Ok(stats)
}
