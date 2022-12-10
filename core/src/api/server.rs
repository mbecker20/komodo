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
            "/:id",
            get(
                |Extension(state): StateExtension, Extension(user): RequestUserExtension, Path(server_id): Path<ServerId>| async move {
                    let server = state
                        .get_server_check_permissions(
                            &server_id.id,
                            &user,
                            PermissionLevel::Read
                        ).await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(server))
                }
            )
        )
        .route(
            "/list",
            get(
                |Extension(state): StateExtension, Extension(user): RequestUserExtension| async move {
                    let servers = state
                        .list_servers(&user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(servers))
                },
            ),
        )
        .route(
            "/create",
            post(
                |Extension(state): StateExtension,
                Extension(user): RequestUserExtension,
                Json(server): Json<CreateServerBody>| async move {
                    let server = state
                        .create_server(&server.name, server.address, &user)
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
            get(
            |Extension(state): StateExtension,
            Extension(user): RequestUserExtension,
            Path(ServerId { id }): Path<ServerId>| async move {
                let stats = state.get_server_stats(&user, &id)
                    .await
                    .map_err(handle_anyhow_error)?;
                response!(Json(stats))
            })
        )
}

impl State {
    async fn list_servers(&self, user: &RequestUser) -> anyhow::Result<Vec<Server>> {
        let mut servers: Vec<Server> = self
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
        Ok(servers)
    }

    async fn get_server_stats(
        &self,
        user: &RequestUser,
        server_id: &str,
    ) -> anyhow::Result<SystemStats> {
        let server = self
            .get_server_check_permissions(server_id, user, PermissionLevel::Read)
            .await?;
        let stats = self
            .periphery
            .get_system_stats(&server)
            .await
            .context(format!("failed to get stats from server {}", server.name))?;
        Ok(stats)
    }
}
