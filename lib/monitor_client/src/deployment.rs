use monitor_types::{Deployment, SystemStats, Update};
use serde_json::json;

use crate::MonitorClient;

impl MonitorClient {
    pub async fn list_deployments(&self) -> anyhow::Result<Vec<Deployment>> {
        self.get("/api/deployment/list").await
    }

    pub async fn create_deployment(
        &self,
        name: &str,
        server_id: &str,
    ) -> anyhow::Result<Deployment> {
        self.post(
            "/api/deployment/create",
            json!({ "name": name, "server_id": server_id }),
        )
        .await
    }

    pub async fn delete_deployment(&self, id: &str) -> anyhow::Result<Deployment> {
        self.delete::<(), _>(&format!("/api/deployment/delete/{id}"), None)
            .await
    }

    pub async fn update_deployment(&self, deployment: Deployment) -> anyhow::Result<Deployment> {
        self.patch("/api/deployment/update", deployment).await
    }
    
    pub async fn reclone_deployment(&self, id: &str) -> anyhow::Result<Update> {
        self.post::<(), _>(&format!("/api/deployment/reclone/{id}"), None).await
    }
}
