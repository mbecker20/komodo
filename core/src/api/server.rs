use anyhow::Context;
use axum::{
    extract::Path,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use helpers::handle_anyhow_error;
use mungos::Deserialize;
use types::{traits::Permissioned, PermissionLevel, Server, SystemStats};

use crate::{
    auth::{RequestUser, RequestUserExtension},
    response,
    state::{State, StateExtension},
};

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
            get(|state, user| async { list(state, user).await.map_err(handle_anyhow_error) }),
        )
        .route(
            "/create",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(server): Json<CreateServerBody>| async move {
                    let server = state
                        .create_server(server.name, server.address, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(server))
                },
            ),
        )
        .route(
            "/delete/:id",
            delete(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(server): Path<ServerId>| async move {
                    let server = state
                        .delete_server(&server.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(server))
                },
            ),
        )
        .route(
            "/update",
            patch(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(server): Json<Server>| async move {
                    let server = state
                        .update_server(server, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(server))
                },
            ),
        )
        .route(
            "/stats/:id",
            get(|state, user, server_id| async {
                stats(state, user, server_id)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
}

async fn list(
    Extension(state): StateExtension,
    Extension(user): RequestUserExtension,
) -> anyhow::Result<Json<Vec<Server>>> {
    let mut servers: Vec<Server> = state
        .db
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

async fn stats(
    Extension(state): StateExtension,
    Extension(user): RequestUserExtension,
    Path(ServerId { id }): Path<ServerId>,
) -> anyhow::Result<Json<SystemStats>> {
    let stats = get_server_stats(&user, &id, &state).await?;
    Ok(Json(stats))
}

pub async fn get_server_stats(
    user: &RequestUser,
    server_id: &str,
    state: &State,
) -> anyhow::Result<SystemStats> {
    let server = state
        .get_server_check_permissions(server_id, user, PermissionLevel::Read)
        .await?;
    let stats = state
        .periphery
        .get_system_stats(&server)
        .await
        .context(format!("failed to get stats from server {}", server.name))?;
    Ok(stats)
}
