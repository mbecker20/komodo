use std::time::Duration;

use anyhow::{anyhow, Context};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use types::{Server, SystemInformation, SystemStats, SystemStatsQuery};

mod build;
mod command;
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
        self.get_text(server, "/health", 1000)
            .await
            .context("failed at health check on periphery")
    }

    pub async fn get_version(&self, server: &Server) -> anyhow::Result<String> {
        self.get_text(server, "/version", 1000)
            .await
            .context("failed to get version from periphery")
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

    pub async fn get_system_information(
        &self,
        server: &Server,
    ) -> anyhow::Result<SystemInformation> {
        self.get_json(server, "/system_information")
            .await
            .context("failed to get system information from periphery")
    }

    pub async fn get_system_stats(
        &self,
        server: &Server,
        query: &SystemStatsQuery,
    ) -> anyhow::Result<SystemStats> {
        self.get_json(
            server,
            &format!(
                "/stats?networks={}&components={}&processes={}&disks={}",
                query.networks, query.components, query.processes, query.disks
            ),
        )
        .await
        .context("failed to get system stats from periphery")
    }

    pub async fn subscribe_to_stats_ws(
        &self,
        server: &Server,
        query: &SystemStatsQuery,
    ) -> anyhow::Result<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        let ws_url = format!(
            "{}/stats/ws?networks={}&components={}&processes={}",
            server.address.replace("http", "ws"),
            query.networks,
            query.components,
            query.processes
        );
        let (socket, _) = connect_async(ws_url)
            .await
            .context("failed to connect to periphery stats ws")?;
        Ok(socket)
    }

    async fn get_text(
        &self,
        server: &Server,
        endpoint: &str,
        timeout_ms: impl Into<Option<u64>>,
    ) -> anyhow::Result<String> {
        let mut req = self
            .http_client
            .get(format!("{}{endpoint}", server.address));

        if let Some(timeout) = timeout_ms.into() {
            req = req.timeout(Duration::from_millis(timeout))
        }

        let res = req.send().await.context(format!(
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
        self.health_check(server).await?;
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
        self.health_check(server).await?;
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
