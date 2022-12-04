use monitor_types::{Build, SystemStats};
use serde_json::json;

use crate::MonitorClient;

impl MonitorClient {
    pub async fn list_builds(&self) -> anyhow::Result<Vec<Build>> {
        self.get("/api/build/list").await
    }

    pub async fn create_build(&self, name: &str, address: &str) -> anyhow::Result<()> {
        self.post(
            "/api/build/create",
            json!({ "name": name, "address": address }),
        )
        .await
    }

    pub async fn delete_build(&self, id: &str) -> anyhow::Result<()> {
        self.delete::<(), _>(&format!("/api/build/delete/{id}"), None)
            .await
    }
}
