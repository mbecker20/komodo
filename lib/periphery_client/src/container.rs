use anyhow::Context;
use serde_json::json;
use types::{BasicContainerInfo, Deployment, DockerContainerStats, Log, Server};

use crate::PeripheryClient;

impl PeripheryClient {
    pub async fn container_list(&self, server: &Server) -> anyhow::Result<Vec<BasicContainerInfo>> {
        self.get_json(server, "/container/list")
            .await
            .context("failed to get container list on periphery")
    }

    pub async fn container_log(
        &self,
        server: &Server,
        container_name: &str,
        tail: Option<u32>,
    ) -> anyhow::Result<Log> {
        self.get_json(
            server,
            &format!(
                "/container/log/{container_name}?tail={}",
                tail.unwrap_or(50)
            ),
        )
        .await
        .context("failed to get container log from periphery")
    }

    pub async fn container_start(
        &self,
        server: &Server,
        container_name: &str,
    ) -> anyhow::Result<Log> {
        self.post_json(
            server,
            "/container/start",
            &json!({ "name": container_name }),
        )
        .await
        .context("failed to start container on periphery")
    }

    pub async fn container_stop(
        &self,
        server: &Server,
        container_name: &str,
    ) -> anyhow::Result<Log> {
        self.post_json(
            server,
            "/container/stop",
            &json!({ "name": container_name }),
        )
        .await
        .context("failed to stop container on periphery")
    }

    pub async fn container_remove(
        &self,
        server: &Server,
        container_name: &str,
    ) -> anyhow::Result<Log> {
        self.post_json(
            server,
            "/container/remove",
            &json!({ "name": container_name }),
        )
        .await
        .context("failed to remove container on periphery")
    }

    pub async fn deploy(&self, server: &Server, deployment: &Deployment) -> anyhow::Result<Log> {
        self.post_json(server, "/container/deploy", deployment)
            .await
            .context("failed to deploy container on periphery")
    }

    pub async fn container_prune(&self, server: &Server) -> anyhow::Result<Log> {
        self.post_json(server, "/container/prune", &json!({}))
            .await
            .context("failed to prune containers on periphery")
    }

    pub async fn container_stats(
        &self,
        server: &Server,
        container_name: &str,
    ) -> anyhow::Result<DockerContainerStats> {
        self.get_json(server, &format!("/container/stats/{container_name}"))
            .await
            .context("failed to get container stats from periphery")
    }

    pub async fn container_stats_list(
        &self,
        server: &Server,
    ) -> anyhow::Result<Vec<DockerContainerStats>> {
        self.get_json(server, "/container/stats/list")
            .await
            .context("failed to get stats list from periphery")
    }
}
