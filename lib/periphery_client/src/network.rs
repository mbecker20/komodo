use anyhow::Context;
use serde_json::json;
use types::{Log, Network, Server};

use crate::PeripheryClient;

impl PeripheryClient {
    pub async fn network_list(&self, server: &Server) -> anyhow::Result<Vec<Network>> {
        self.get_json(server, "/network/list")
            .await
            .context("failed to get network list from periphery")
    }

    pub async fn network_create(
        &self,
        server: &Server,
        name: &str,
        driver: Option<String>,
    ) -> anyhow::Result<Log> {
        self.post_json(
            server,
            "/network/create",
            &json!({
                "name": name,
                "driver": driver
            }),
        )
        .await
        .context("failed to create network on periphery")
    }

    pub async fn network_delete(&self, server: &Server, name: &str) -> anyhow::Result<Log> {
        self.post_json(server, "/network/delete", &json!({ "name": name }))
            .await
            .context("failed to delete network on periphery")
    }

    pub async fn network_prune(&self, server: &Server) -> anyhow::Result<Log> {
        self.post_json(server, "/network/prune", &json!({}))
            .await
            .context("failed to prune networks on periphery")
    }
}
