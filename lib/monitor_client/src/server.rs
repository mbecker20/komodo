use monitor_types::{Server, SystemStats};
use serde_json::json;

use crate::MonitorClient;

impl MonitorClient {
    pub async fn list_servers(&self) -> anyhow::Result<Vec<Server>> {
        self.get("/api/server/list").await
    }

    pub async fn create_server(&self, name: &str, address: &str) -> anyhow::Result<Server> {
        self.post(
            "/api/server/create",
            json!({ "name": name, "address": address }),
        )
        .await
    }

    pub async fn delete_server(&self, id: &str) -> anyhow::Result<Server> {
        self.delete::<(), _>(&format!("/api/server/delete/{id}"), None)
            .await
    }

    pub async fn update_server(&self, server: Server) -> anyhow::Result<Server> {
        self.patch("/api/server/update", server).await
    }

    pub async fn get_server_stats(&self, id: &str) -> anyhow::Result<SystemStats> {
        self.get(&format!("/api/server/stats/{id}")).await
    }
}
