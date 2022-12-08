use anyhow::Context;
use monitor_types::{Build, SystemStats, Update};
use serde_json::json;

use crate::MonitorClient;

impl MonitorClient {
    pub async fn list_builds(&self) -> anyhow::Result<Vec<Build>> {
        self.get("/api/build/list")
            .await
            .context("failed at list builds")
    }

    pub async fn create_build(&self, name: &str, server_id: &str) -> anyhow::Result<Build> {
        self.post(
            "/api/build/create",
            json!({ "name": name, "server_id": server_id }),
        )
        .await
        .context(format!(
            "failed at creating build with name {name} on server id {server_id}"
        ))
    }

    pub async fn delete_build(&self, id: &str) -> anyhow::Result<Build> {
        self.delete::<(), _>(&format!("/api/build/delete/{id}"), None)
            .await
            .context(format!("failed at deleting build {id}"))
    }

    pub async fn update_build(&self, build: Build) -> anyhow::Result<Build> {
        self.patch("/api/build/update", build)
            .await
            .context("failed at updating build")
    }

    pub async fn build(&self, build_id: &str) -> anyhow::Result<Update> {
        self.post::<(), _>(&format!("/api/build/build/{build_id}"), None)
            .await
            .context(format!("failed at building build {build_id}"))
    }

    pub async fn reclone_build(&self, id: &str) -> anyhow::Result<Update> {
        self.post::<(), _>(&format!("/api/build/reclone/{id}"), None)
            .await
            .context(format!("failed at recloning build {id}"))
    }
}
