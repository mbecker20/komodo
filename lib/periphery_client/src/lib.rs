use anyhow::{anyhow, Context};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use types::{BasicContainerInfo, Server, Log, Deployment};

pub struct PeripheryClient {
    http_client: reqwest::Client,
}

impl PeripheryClient {
    pub fn new() -> PeripheryClient {
        PeripheryClient {
            http_client: reqwest::Client::new(),
        }
    }

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
        self.post_json(
            server,
            &format!("/container/deploy"),
            deployment,
        )
        .await
    }

    async fn get_json<R: DeserializeOwned>(
        &self,
        server: &Server,
        endpoint: &str,
    ) -> anyhow::Result<R> {
        let res = self
            .http_client
            .get(format!("{}{endpoint}", server.address))
            .send()
            .await
            .context(format!(
                "failed at request to server {} | no response",
                server.name
            ))?;
        let status = res.status();
        if status == StatusCode::OK {
            let parsed = res
                .json::<R>()
                .await
                .context("failed at parsing response")?;
            Ok(parsed)
        } else {
            let error = res
                .text()
                .await
                .context(format!("failed at getting error text | status: {status}"))?;
            Err(anyhow!(
                "failed at request to server {} | status: {status} | error: {error:#?}",
                server.name
            ))
        }
    }

    async fn post_json<B: Serialize, R: DeserializeOwned>(
        &self,
        server: &Server,
        endpoint: &str,
        body: &B,
    ) -> anyhow::Result<R> {
        let res = self
            .http_client
            .post(format!("{}{endpoint}", server.address))
            .json(body)
            .send()
            .await
            .context(format!(
                "failed at request to server {} | no response",
                server.name
            ))?;
        let status = res.status();
        if status == StatusCode::OK {
            let parsed = res
                .json::<R>()
                .await
                .context("failed at parsing response")?;
            Ok(parsed)
        } else {
            let error = res
                .text()
                .await
                .context(format!("failed at getting error text | status: {status}"))?;
            Err(anyhow!(
                "failed at request to server {} | status: {status} | error: {error:#?}",
                server.name
            ))
        }
    }
}
