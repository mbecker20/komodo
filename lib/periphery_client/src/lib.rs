use anyhow::{anyhow, Context};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use types::Server;

mod container;
mod git;
mod network;
mod stats;

pub struct PeripheryClient {
    http_client: reqwest::Client,
}

impl PeripheryClient {
    pub fn new() -> PeripheryClient {
        PeripheryClient {
            http_client: reqwest::Client::new(),
        }
    }

    pub async fn get_github_accounts(&self, server: &Server) -> anyhow::Result<Vec<String>> {
        self.get_json(server, "/accounts/github").await
    }

    pub async fn get_docker_accounts(&self, server: &Server) -> anyhow::Result<Vec<String>> {
        self.get_json(server, "/accounts/docker").await
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
