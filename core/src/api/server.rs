use anyhow::{anyhow, Context};
use async_timing_util::{get_timelength_in_ms, unix_timestamp_ms};
use axum::{
    extract::{ws::Message as AxumMessage, Path, Query, WebSocketUpgrade},
    response::IntoResponse,
    routing::{delete, get, patch, post},
    Json, Router,
};
use futures_util::{future::join_all, SinkExt, StreamExt};
use helpers::handle_anyhow_error;
use mungos::{doc, Deserialize, Document, FindOptions};
use tokio::select;
use tokio_tungstenite::tungstenite::Message;
use tokio_util::sync::CancellationToken;
use types::{
    traits::Permissioned, BasicContainerInfo, HistoricalStatsQuery, ImageSummary, Network,
    PermissionLevel, Server, ServerActionState, ServerStatus, ServerWithStatus, SystemInformation,
    SystemStats, SystemStatsQuery, SystemStatsRecord,
};
use typeshare::typeshare;

const MAX_HISTORICAL_STATS_LIMIT: i64 = 500;

use crate::{
    auth::{RequestUser, RequestUserExtension},
    response,
    state::{State, StateExtension},
};

use super::spawn_request_action;

#[derive(Deserialize)]
struct ServerId {
    id: String,
}

#[derive(Deserialize)]
struct Ts {
    ts: i64,
}

#[typeshare]
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
                |state: StateExtension,
                 user: RequestUserExtension,
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
                |state: StateExtension,
                 user: RequestUserExtension,
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
                |state: StateExtension,
                 user: RequestUserExtension,
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
                |state: StateExtension,
                 user: RequestUserExtension,
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
                |state: StateExtension,
                 user: RequestUserExtension,
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
                |state: StateExtension,
                 user: RequestUserExtension,
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
            "/:id/version",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id })| async move {
                    let stats = state
                        .get_server_version(&id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/system_information",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id })| async move {
                    let stats = state
                        .get_server_system_info(&id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/stats",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id }),
                 query: Query<SystemStatsQuery>| async move {
                    let stats = state
                        .get_server_stats(&id, &user, &query)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/stats/history",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id }),
                 query: Query<HistoricalStatsQuery>| async move {
                    let stats = state
                        .get_historical_stats(&id, &user, &query)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/stats/at_ts",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id }),
                 Query(Ts { ts })| async move {
                    let stats = state
                        .get_stats_at_ts(&id, &user, ts)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/stats/ws",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id }),
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
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id })| async move {
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
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id })| async move {
                    let stats = spawn_request_action(async move {
                        state
                        .prune_networks(&id, &user)
                        .await
                        .map_err(handle_anyhow_error)
                    }).await??;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/images",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id })| async move {
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
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id })| async move {
                    let stats = spawn_request_action(async move {
                        state
                        .prune_images(&id, &user)
                        .await
                        .map_err(handle_anyhow_error)
                    }).await??;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/containers",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id })| async move {
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
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id })| async move {
                    let stats = spawn_request_action(async move {
                        state
                        .prune_containers(&id, &user)
                        .await
                        .map_err(handle_anyhow_error)
                    }).await??;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/github_accounts",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id })| async move {
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
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id })| async move {
                    let docker_accounts = state
                        .get_docker_accounts(&id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(docker_accounts))
                },
            ),
        )
        .route(
            "/:id/secrets",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id })| async move {
                    let vars = state
                        .get_available_secrets(&id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(vars))
                },
            ),
        )
        .route(
            "/:id/action_state",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(ServerId { id })| async move {
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
    pub async fn get_server(
        &self,
        id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<ServerWithStatus> {
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
        Ok(join_all(futures).await)
    }

    async fn get_server_version(
        &self,
        server_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<String> {
        let server = self
            .get_server_check_permissions(server_id, user, PermissionLevel::Read)
            .await?;
        let version = self.periphery.get_version(&server).await.context(format!(
            "failed to get system information from server {}",
            server.name
        ))?;
        Ok(version)
    }

    async fn get_server_system_info(
        &self,
        server_id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<SystemInformation> {
        let server = self
            .get_server_check_permissions(server_id, user, PermissionLevel::Read)
            .await?;
        let stats = self
            .periphery
            .get_system_information(&server)
            .await
            .context(format!(
                "failed to get system information from server {}",
                server.name
            ))?;
        Ok(stats)
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

    async fn get_historical_stats(
        &self,
        server_id: &str,
        user: &RequestUser,
        query: &HistoricalStatsQuery,
    ) -> anyhow::Result<Vec<SystemStatsRecord>> {
        self.get_server_check_permissions(server_id, user, PermissionLevel::Read)
            .await?;

        let mut projection = doc! { "processes": 0, "disk.disks": 0 };
        if !query.networks {
            projection.insert("networks", 0);
        }
        if !query.components {
            projection.insert("components", 0);
        }
        let limit = if query.limit as i64 > MAX_HISTORICAL_STATS_LIMIT {
            MAX_HISTORICAL_STATS_LIMIT
        } else {
            query.limit as i64
        };
        let interval = get_timelength_in_ms(query.interval.to_string().parse().unwrap()) as i64;
        let mut ts_vec = Vec::<i64>::new();
        let curr_ts = unix_timestamp_ms() as i64;
        let mut curr_ts = curr_ts - curr_ts % interval - interval * limit * query.page as i64;
        for _ in 0..limit {
            ts_vec.push(curr_ts);
            curr_ts -= interval;
        }
        self.db
            .stats
            .get_some(
                doc! {
                    "server_id": server_id, 
                    "ts": { "$in": ts_vec }
                },
                FindOptions::builder()
                    .sort(doc! { "ts": -1 })
                    .projection(projection)
                    .build(),
            )
            .await
            .context("failed at mongo query to get stats")
    }

    async fn get_stats_at_ts(
        &self,
        server_id: &str,
        user: &RequestUser,
        ts: i64,
    ) -> anyhow::Result<SystemStatsRecord> {
        self.get_server_check_permissions(server_id, user, PermissionLevel::Read)
            .await?;
        self.db
            .stats
            .find_one(doc! { "server_id": server_id, "ts": ts }, None)
            .await
            .context("failed at mongo query to get full stat entry")?
            .ok_or(anyhow!("did not find entry for server at time"))
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

    async fn get_available_secrets(
        &self,
        id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<Vec<String>> {
        let server = self
            .get_server_check_permissions(id, user, PermissionLevel::Read)
            .await?;
        let vars = self.periphery.get_available_secrets(&server).await?;
        Ok(vars)
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
            .await
            .entry(id)
            .or_default()
            .clone();
        Ok(action_state)
    }
}
