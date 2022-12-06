use monitor_types::{Build, SystemStats, Update};
use serde_json::json;

use crate::MonitorClient;

impl MonitorClient {
    pub async fn list_builds(&self) -> anyhow::Result<Vec<Build>> {
        self.get("/api/build/list").await
    }

    pub async fn create_build(&self, name: &str, server_id: &str) -> anyhow::Result<Build> {
        self.post(
            "/api/build/create",
            json!({ "name": name, "server_id": server_id }),
        )
        .await
    }

    pub async fn delete_build(&self, id: &str) -> anyhow::Result<Build> {
        self.delete::<(), _>(&format!("/api/build/delete/{id}"), None)
            .await
    }

    pub async fn update_build(&self, build: Build) -> anyhow::Result<Build> {
        self.patch("/api/build/update", build).await
    }

    pub async fn reclone_build(&self, id: &str) -> anyhow::Result<Update> {
        self.post::<(), _>(&format!("/api/build/reclone/{id}"), None).await
    }
}
