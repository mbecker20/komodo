use anyhow::Context;
use monitor_types::{
    BasicContainerInfo, ImageSummary, Log, Network, Server, ServerActionState, ServerWithStatus,
    SystemStats,
};
use serde_json::{json, Value};

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

    pub async fn get_server(&self, server_id: &str) -> anyhow::Result<Server> {
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

    pub async fn get_server_stats(&self, server_id: &str) -> anyhow::Result<SystemStats> {
        self.get(
            &format!("/api/server/{server_id}/stats"),
            Option::<()>::None,
        )
        .await
        .context(format!("failed to get server stats at id {server_id}"))
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
