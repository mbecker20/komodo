use anyhow::{anyhow, Context};
use futures_util::{SinkExt, StreamExt};
use monitor_types::{
    BasicContainerInfo, ImageSummary, Log, Network, Server, ServerActionState, ServerWithStatus,
    SystemStats, SystemStatsQuery,
};
use serde_json::{json, Value};
use tokio::{
    sync::broadcast::{self, Receiver},
    task::JoinHandle,
};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use tokio_util::sync::CancellationToken;

use crate::MonitorClient;

impl MonitorClient {
    pub async fn list_servers(
        &self,
        query: impl Into<Option<Value>>,
    ) -> anyhow::Result<Vec<ServerWithStatus>> {
        self.get("/api/server/list", query.into())
            .await
            .context("failed at list servers")
    }

    pub async fn get_server(&self, server_id: &str) -> anyhow::Result<ServerWithStatus> {
        self.get(&format!("/api/server/{server_id}"), Option::<()>::None)
            .await
    }

    pub async fn get_server_action_state(
        &self,
        server_id: &str,
    ) -> anyhow::Result<ServerActionState> {
        self.get(
            &format!("/api/server/{server_id}/action_state"),
            Option::<()>::None,
        )
        .await
    }

    pub async fn get_server_github_accounts(&self, server_id: &str) -> anyhow::Result<Vec<String>> {
        self.get(
            &format!("/api/server/{server_id}/github_accounts"),
            Option::<()>::None,
        )
        .await
    }

    pub async fn get_server_docker_accounts(&self, server_id: &str) -> anyhow::Result<Vec<String>> {
        self.get(
            &format!("/api/server/{server_id}/docker_accounts"),
            Option::<()>::None,
        )
        .await
    }

    pub async fn create_server(&self, name: &str, address: &str) -> anyhow::Result<Server> {
        self.post(
            "/api/server/create",
            json!({ "name": name, "address": address }),
        )
        .await
        .context(format!(
            "failed at creating server with name {name} at address {address}"
        ))
    }

    pub async fn create_full_server(&self, server: &Server) -> anyhow::Result<Server> {
        self.post::<&Server, _>("/api/server/create_full", server)
            .await
            .context(format!("failed at creating full server"))
    }

    pub async fn delete_server(&self, id: &str) -> anyhow::Result<Server> {
        self.delete::<(), _>(&format!("/api/server/{id}/delete"), None)
            .await
            .context(format!("failed at delete server {id}"))
    }

    pub async fn update_server(&self, server: Server) -> anyhow::Result<Server> {
        self.patch("/api/server/update", server)
            .await
            .context("failed at update server")
    }

    pub async fn get_server_stats(
        &self,
        server_id: &str,
        query: impl Into<Option<&SystemStatsQuery>>,
    ) -> anyhow::Result<SystemStats> {
        self.get(&format!("/api/server/{server_id}/stats"), query.into())
            .await
            .context(format!("failed to get server stats at id {server_id}"))
    }

    pub async fn subscribe_to_stats_ws(
        &self,
        server_id: &str,
        query: impl Into<Option<SystemStatsQuery>>,
    ) -> anyhow::Result<(
        Receiver<SystemStats>,
        JoinHandle<anyhow::Result<()>>,
        CancellationToken,
    )> {
        let query = query.into().unwrap_or_default();
        let endpoint = format!(
            "{}/ws/stats/{server_id}?networks={}&components={}&processes={}",
            self.url.replace("http", "ws"),
            query.networks,
            query.components,
            query.processes
        );
        let (mut socket, _) = connect_async(endpoint).await?;
        socket.send(Message::Text(self.token.clone())).await?;
        let msg = socket.next().await;
        if let Some(Ok(Message::Text(msg))) = &msg {
            if msg.as_str() == "LOGGED_IN" {
                let cancel = CancellationToken::new();
                let cancel_clone = cancel.clone();
                let (sender, receiver) = broadcast::channel(100);
                let handle = tokio::spawn(async move {
                    loop {
                        let stats = tokio::select! {
                            _ = cancel_clone.cancelled() => {
                                let _ = socket.close(None).await;
                                break;
                            },
                            stats = socket.next() => stats,
                        };
                        if let Some(Ok(Message::Text(stats))) = stats {
                            let stats: SystemStats = serde_json::from_str(&stats)
                                .context("failed to parse msg as SystemStats")?;
                            sender
                                .send(stats)
                                .context("failed to send stats through broadcast channel")?;
                        }
                    }
                    Ok(())
                });
                Ok((receiver, handle, cancel))
            } else {
                Err(anyhow!("failed to log in"))
            }
        } else if let Some(Err(e)) = &msg {
            Err(anyhow!("error on connection: {e:?}"))
        } else {
            Err(anyhow!("some other failure"))
        }
    }

    pub async fn get_docker_networks(&self, server_id: &str) -> anyhow::Result<Vec<Network>> {
        self.get(
            &format!("/api/server/{server_id}/networks"),
            Option::<()>::None,
        )
        .await
        .context(format!("failed to get networks on server id {server_id}"))
    }

    pub async fn prune_docker_networks(&self, server_id: &str) -> anyhow::Result<Log> {
        self.post::<(), _>(&format!("/api/server/{server_id}/networks/prune"), None)
            .await
            .context(format!("failed to prune networks on server id {server_id}"))
    }

    pub async fn get_docker_images(&self, server_id: &str) -> anyhow::Result<Vec<ImageSummary>> {
        self.get(
            &format!("/api/server/{server_id}/images"),
            Option::<()>::None,
        )
        .await
        .context(format!("failed to get images on server id {server_id}"))
    }

    pub async fn prune_docker_images(&self, server_id: &str) -> anyhow::Result<Log> {
        self.post::<(), _>(&format!("/api/server/{server_id}/images/prune"), None)
            .await
            .context(format!("failed to prune images on server id {server_id}"))
    }

    pub async fn get_docker_containers(
        &self,
        server_id: &str,
    ) -> anyhow::Result<Vec<BasicContainerInfo>> {
        self.get(
            &format!("/api/server/{server_id}/containers"),
            Option::<()>::None,
        )
        .await
        .context(format!("failed to get containers on server id {server_id}"))
    }

    pub async fn prune_docker_containers(&self, server_id: &str) -> anyhow::Result<Log> {
        self.post::<(), _>(&format!("/api/server/{server_id}/containers/prune"), None)
            .await
            .context(format!(
                "failed to prune containers on server id {server_id}"
            ))
    }
}
