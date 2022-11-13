use serde_json::json;
use types::{Log, Server};

use crate::PeripheryClient;

impl PeripheryClient {
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
    }

    pub async fn network_delete(&self, server: &Server, name: &str) -> anyhow::Result<Log> {
        self.post_json(server, "/network/delete", &json!({ "name": name }))
            .await
    }

    pub async fn network_prune(&self, server: &Server) -> anyhow::Result<Log> {
        self.post_json(server, "/network/prune", &json!({})).await
    }
}
