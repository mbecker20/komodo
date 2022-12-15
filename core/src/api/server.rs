use anyhow::Context;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use helpers::handle_anyhow_error;
use mungos::{Deserialize, Document};
use types::{
    traits::Permissioned, BasicContainerInfo, ImageSummary, Log, Network, PermissionLevel, Server,
    SystemStats,
};

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
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Query(query): Query<Document>| async move {
                    let servers = state
                        .list_servers(&user, query)
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
            "/create_full",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(server): Json<Server>| async move {
                    let server = state
                        .create_full_server(server, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(server))
                },
            ),
        )
        .route(
            "/:id",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(server_id): Path<ServerId>| async move {
                    let server = state
                        .get_server_check_permissions(&server_id.id, &user, PermissionLevel::Read)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(server))
                },
            ),
        )
        .route(
            "/:id/delete",
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
            "/:id/stats",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(ServerId { id }): Path<ServerId>| async move {
                    let stats = state
                        .get_server_stats(&user, &id)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/networks",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(ServerId { id }): Path<ServerId>| async move {
                    let stats = state
                        .get_networks(&user, &id)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/networks/prune",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(ServerId { id }): Path<ServerId>| async move {
                    let stats = state
                        .prune_networks(&user, &id)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/images",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(ServerId { id }): Path<ServerId>| async move {
                    let stats = state
                        .get_images(&user, &id)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/images/prune",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(ServerId { id }): Path<ServerId>| async move {
                    let stats = state
                        .prune_images(&user, &id)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/containers",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(ServerId { id }): Path<ServerId>| async move {
                    let stats = state
                        .get_containers(&user, &id)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/containers/prune",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(ServerId { id }): Path<ServerId>| async move {
                    let stats = state
                        .prune_containers(&user, &id)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
}

impl State {
    async fn list_servers(
        &self,
        user: &RequestUser,
        query: impl Into<Option<Document>>,
    ) -> anyhow::Result<Vec<Server>> {
        let mut servers: Vec<Server> = self
            .db
            .servers
            .get_some(query, None)
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

    async fn get_networks(
        &self,
        user: &RequestUser,
        server_id: &str,
    ) -> anyhow::Result<Vec<Network>> {
        let server = self
            .get_server_check_permissions(server_id, user, PermissionLevel::Read)
            .await?;
        let stats = self.periphery.network_list(&server).await.context(format!(
            "failed to get networks from server {}",
            server.name
        ))?;
        Ok(stats)
    }

    async fn prune_networks(&self, user: &RequestUser, server_id: &str) -> anyhow::Result<Log> {
        let server = self
            .get_server_check_permissions(server_id, user, PermissionLevel::Write)
            .await?;
        let log = self
            .periphery
            .network_prune(&server)
            .await
            .context(format!(
                "failed to prune networks on server {}",
                server.name
            ))?;
        Ok(log)
    }

    async fn get_images(
        &self,
        user: &RequestUser,
        server_id: &str,
    ) -> anyhow::Result<Vec<ImageSummary>> {
        let server = self
            .get_server_check_permissions(server_id, user, PermissionLevel::Read)
            .await?;
        let images = self
            .periphery
            .image_list(&server)
            .await
            .context(format!("failed to get images from server {}", server.name))?;
        Ok(images)
    }

    async fn prune_images(&self, user: &RequestUser, server_id: &str) -> anyhow::Result<Log> {
        let server = self
            .get_server_check_permissions(server_id, user, PermissionLevel::Write)
            .await?;
        let stats = self
            .periphery
            .image_prune(&server)
            .await
            .context(format!("failed to prune images on server {}", server.name))?;
        Ok(stats)
    }

    async fn get_containers(
        &self,
        user: &RequestUser,
        server_id: &str,
    ) -> anyhow::Result<Vec<BasicContainerInfo>> {
        let server = self
            .get_server_check_permissions(server_id, user, PermissionLevel::Read)
            .await?;
        let images = self
            .periphery
            .container_list(&server)
            .await
            .context(format!(
                "failed to get containers from server {}",
                server.name
            ))?;
        Ok(images)
    }

    async fn prune_containers(&self, user: &RequestUser, server_id: &str) -> anyhow::Result<Log> {
        let server = self
            .get_server_check_permissions(server_id, user, PermissionLevel::Write)
            .await?;
        let log = self
            .periphery
            .container_prune(&server)
            .await
            .context(format!(
                "failed to prune containers on server {}",
                server.name
            ))?;
        Ok(log)
    }
}
