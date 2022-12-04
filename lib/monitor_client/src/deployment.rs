use monitor_types::{Deployment, SystemStats};
use serde_json::json;

use crate::MonitorClient;

impl MonitorClient {
    pub async fn list_deployments(&self) -> anyhow::Result<Vec<Deployment>> {
        self.get("/api/deployment/list").await
    }

    pub async fn create_deployment(&self, name: &str, address: &str) -> anyhow::Result<()> {
        self.post(
            "/api/deployment/create",
            json!({ "name": name, "address": address }),
        )
        .await
    }

    pub async fn delete_deployment(&self, id: &str) -> anyhow::Result<()> {
        self.delete::<(), _>(&format!("/api/deployment/delete/{id}"), None)
            .await
    }
}
