use serde_json::json;
use types::{BasicContainerInfo, Deployment, Log, Server};

use crate::PeripheryClient;

impl PeripheryClient {
    pub async fn container_list(&self, server: &Server) -> anyhow::Result<Vec<BasicContainerInfo>> {
        self.get_json(server, "/container/list").await
    }

    pub async fn container_start(
        &self,
        server: &Server,
        container_name: &str,
    ) -> anyhow::Result<Log> {
        self.post_json(
            server,
            &format!("/container/start"),
            &json!({ "name": container_name }),
        )
        .await
    }

    pub async fn container_stop(
        &self,
        server: &Server,
        container_name: &str,
    ) -> anyhow::Result<Log> {
        self.post_json(
            server,
            &format!("/container/stop"),
            &json!({ "name": container_name }),
        )
        .await
    }

    pub async fn container_remove(
        &self,
        server: &Server,
        container_name: &str,
    ) -> anyhow::Result<Log> {
        self.post_json(
            server,
            &format!("/container/remove"),
            &json!({ "name": container_name }),
        )
        .await
    }

    pub async fn deploy(&self, server: &Server, deployment: &Deployment) -> anyhow::Result<Log> {
        self.post_json(server, &format!("/container/deploy"), deployment)
            .await
    }

    pub async fn container_prune(&self, server: &Server) -> anyhow::Result<Log> {
        self.post_json(server, "container/prune", &json!({})).await
    }
}
