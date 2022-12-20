use anyhow::{anyhow, Context};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use types::{Server, SystemStats};

mod build;
mod container;
mod git;
mod image;
mod network;

#[derive(Default)]
pub struct PeripheryClient {
    http_client: reqwest::Client,
}

impl PeripheryClient {
    pub async fn health_check(&self, server: &Server) -> anyhow::Result<String> {
        self.get_text(server, "health")
            .await
            .context("failed at health check on periphery")
    }

    pub async fn get_github_accounts(&self, server: &Server) -> anyhow::Result<Vec<String>> {
        self.get_json(server, "/accounts/github")
            .await
            .context("failed to get github accounts from periphery")
    }

    pub async fn get_docker_accounts(&self, server: &Server) -> anyhow::Result<Vec<String>> {
        self.get_json(server, "/accounts/docker")
            .await
            .context("failed to get docker accounts from periphery")
    }

    pub async fn get_system_stats(&self, server: &Server) -> anyhow::Result<SystemStats> {
        self.get_json(server, "/stats/system")
            .await
            .context("failed to get system stats from periphery")
    }

    async fn get_text(&self, server: &Server, endpoint: &str) -> anyhow::Result<String> {
        let res = self
            .http_client
            .get(format!("{}{endpoint}", server.address))
            .send()
            .await
            .context(format!(
                "failed at get request to server {} | not reachable",
                server.name
            ))?;
        let status = res.status();
        if status == StatusCode::OK {
            let text = res.text().await.context("failed at parsing response")?;
            Ok(text)
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
                "failed at get request to server {} | not reachable",
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
                "failed at post request to server {} | not reachable",
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
