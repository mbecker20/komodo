use anyhow::Context;
use axum::{
    extract::{ws::Message as AxumMessage, Path, Query, WebSocketUpgrade},
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use futures_util::{future::join_all, SinkExt, StreamExt};
use helpers::handle_anyhow_error;
use mungos::{Deserialize, Document, Serialize};
use tokio::select;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;
use types::{
    traits::Permissioned, BasicContainerInfo, ImageSummary, Network, PermissionLevel, Server,
    ServerActionState, ServerStatus, ServerWithStatus, SystemStats, SystemStatsQuery,
};
use typeshare::typeshare;

use crate::{
    auth::{RequestUser, RequestUserExtension},
    response,
    state::{State, StateExtension},
};

#[derive(Serialize, Deserialize)]
struct ServerId {
    id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct CreateServerBody {
    name: String,
    address: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/:id",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(server_id): Path<ServerId>| async move {
                    let server = state
                        .get_server(&server_id.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(server))
                },
            ),
        )
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
                 Path(ServerId { id }): Path<ServerId>,
                 Query(query): Query<SystemStatsQuery>| async move {
                    let stats = state
                        .get_server_stats(&id, &user, &query)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/stats/ws",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(ServerId { id }): Path<ServerId>,
                 Query(query): Query<SystemStatsQuery>,
                 ws: WebSocketUpgrade| async move {
                    let connection = state
                        .subscribe_to_stats_ws(&id, &user, &query, ws)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(connection)
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
                        .get_networks(&id, &user)
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
                        .prune_networks(&id, &user)
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
                        .get_images(&id, &user)
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
                        .prune_images(&id, &user)
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
                        .get_containers(&id, &user)
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
                        .prune_containers(&id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/github_accounts",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(ServerId { id }): Path<ServerId>| async move {
                    let github_accounts = state
                        .get_github_accounts(&id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(github_accounts))
                },
            ),
        )
        .route(
            "/:id/docker_accounts",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(ServerId { id }): Path<ServerId>| async move {
                    let docker_accounts = state
                        .get_docker_accounts(&id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(docker_accounts))
                },
            ),
        )
        .route(
            "/:id/action_state",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(ServerId { id }): Path<ServerId>| async move {
                    let action_state = state
                        .get_server_action_states(id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(action_state))
                },
            ),
        )
}

impl State {
    async fn get_server(&self, id: &str, user: &RequestUser) -> anyhow::Result<ServerWithStatus> {
        let server = self
            .get_server_check_permissions(id, user, PermissionLevel::Read)
            .await?;
        let status = if server.enabled {
            let res = self.periphery.health_check(&server).await;
            match res {
                Ok(_) => ServerStatus::Ok,
                Err(_) => ServerStatus::NotOk,
            }
        } else {
            ServerStatus::Disabled
        };
        Ok(ServerWithStatus { server, status })
    }

    async fn list_servers(
        &self,
        user: &RequestUser,
        query: impl Into<Option<Document>>,
    ) -> anyhow::Result<Vec<ServerWithStatus>> {
        let futures = self
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
            .map(|server| async {
                let status = if server.enabled {
                    let res = self.periphery.health_check(&server).await;
                    match res {
                        Ok(_) => ServerStatus::Ok,
                        Err(_) => ServerStatus::NotOk,
                    }
                } else {
                    ServerStatus::Disabled
                };

                ServerWithStatus { server, status }
            });
        let mut servers: Vec<ServerWithStatus> = join_all(futures).await;
        servers.sort_by(|a, b| {
            a.server
                .name
                .to_lowercase()
                .cmp(&b.server.name.to_lowercase())
        });
        Ok(servers)
    }

    async fn get_server_stats(
        &self,
        server_id: &str,
        user: &RequestUser,
        query: &SystemStatsQuery,
    ) -> anyhow::Result<SystemStats> {
        let server = self
            .get_server_check_permissions(server_id, user, PermissionLevel::Read)
            .await?;
        let stats = self
            .periphery
            .get_system_stats(&server, query)
            .await
            .context(format!("failed to get stats from server {}", server.name))?;
        Ok(stats)
    }

    async fn subscribe_to_stats_ws(
        &self,
        server_id: &str,
        user: &RequestUser,
        query: &SystemStatsQuery,
        ws: WebSocketUpgrade,
    ) -> anyhow::Result<impl IntoResponse> {
        let server = self
            .get_server_check_permissions(server_id, user, PermissionLevel::Read)
            .await?;
        let mut stats_reciever = self.periphery.subscribe_to_stats_ws(&server, query).await?;
        let upgrade = ws.on_upgrade(|socket| async move {
            let (mut ws_sender, mut ws_recv) = socket.split();
            let cancel = CancellationToken::new();
            let cancel_clone = cancel.clone();
            tokio::spawn(async move {
                loop {
                    let stats = select! {
                        _ = cancel_clone.cancelled() => break,
                        stats = stats_reciever.next() => stats
                    };
                    if let Some(Ok(Message::Text(msg))) = stats {
                        let _ = ws_sender.send(AxumMessage::Text(msg)).await;
                    }
                }
            });
            while let Some(msg) = ws_recv.next().await {
                match msg {
                    Ok(msg) => match msg {
                        AxumMessage::Close(_) => {
                            cancel.cancel();
                            return;
                        }
                        _ => {}
                    },
                    Err(_) => {
                        cancel.cancel();
                        return;
                    }
                }
            }
        });
        Ok(upgrade)
    }

    async fn get_networks(
        &self,
        server_id: &str,
        user: &RequestUser,
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

    async fn get_images(
        &self,
        server_id: &str,
        user: &RequestUser,
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

    async fn get_containers(
        &self,
        server_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Vec<BasicContainerInfo>> {
        let server = self
            .get_server_check_permissions(server_id, user, PermissionLevel::Read)
            .await?;
        let containers = self
            .periphery
            .container_list(&server)
            .await
            .context(format!(
                "failed to get containers from server {}",
                server.name
            ))?;
        Ok(containers)
    }

    async fn get_github_accounts(
        &self,
        id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Vec<String>> {
        let server = self
            .get_server_check_permissions(id, user, PermissionLevel::Read)
            .await?;
        let github_accounts = self.periphery.get_github_accounts(&server).await?;
        Ok(github_accounts)
    }

    async fn get_docker_accounts(
        &self,
        id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Vec<String>> {
        let server = self
            .get_server_check_permissions(id, user, PermissionLevel::Read)
            .await?;
        let docker_accounts = self.periphery.get_docker_accounts(&server).await?;
        Ok(docker_accounts)
    }

    async fn get_server_action_states(
        &self,
        id: String,
        user: &RequestUser,
    ) -> anyhow::Result<ServerActionState> {
        self.get_server_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        let action_state = self
            .server_action_states
            .lock()
            .unwrap()
            .entry(id)
            .or_default()
            .clone();
        Ok(action_state)
    }
}
