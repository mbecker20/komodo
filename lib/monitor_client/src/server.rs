use anyhow::Context;
use monitor_types::{Server, SystemStats};
use serde::Serialize;
use serde_json::{json, Value};

use crate::MonitorClient;

impl MonitorClient {
    pub async fn list_servers(
        &self,
        query: impl Into<Option<Value>>,
    ) -> anyhow::Result<Vec<Server>> {
        self.get("/api/server/list", query.into())
            .await
            .context("failed at list servers")
    }

    pub async fn get_server(&self, server_id: &str) -> anyhow::Result<Server> {
        self.get(&format!("/api/server/{server_id}"), Option::<()>::None)
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

    pub async fn delete_server(&self, id: &str) -> anyhow::Result<Server> {
        self.delete::<(), _>(&format!("/api/server/delete/{id}"), None)
            .await
            .context(format!("failed at delete server {id}"))
    }

    pub async fn update_server(&self, server: Server) -> anyhow::Result<Server> {
        self.patch("/api/server/update", server)
            .await
            .context("failed at update server")
    }

    pub async fn get_server_stats(&self, id: &str) -> anyhow::Result<SystemStats> {
        self.get(&format!("/api/server/stats/{id}"), Option::<()>::None)
            .await
            .context(format!("failed to get server stats at id {id}"))
    }
}
