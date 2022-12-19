#![allow(unused)]

use anyhow::{anyhow, Context};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};

pub use monitor_types as types;
use serde_json::json;

mod build;
mod deployment;
mod permissions;
mod procedure;
mod secret;
mod server;

#[derive(Clone)]
pub struct MonitorClient {
    http_client: reqwest::Client,
    url: String,
    token: String,
}

impl MonitorClient {
    pub fn new_with_token(url: &str, token: impl Into<String>) -> MonitorClient {
        MonitorClient {
            http_client: reqwest::Client::new(),
            url: parse_url(url),
            token: token.into(),
        }
    }

    pub async fn new_with_password(
        url: &str,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> anyhow::Result<MonitorClient> {
        let mut client = MonitorClient::new_with_token(url, "");
        let token = client
            .post_string(
                "/auth/local/login",
                json!({ "username": username.into(), "password": password.into() }),
            )
            .await
            .context("failed to log in with password")?;
        client.token = token;
        Ok(client)
    }

    pub async fn new_with_secret(
        url: &str,
        username: impl Into<String>,
        secret: impl Into<String>,
    ) -> anyhow::Result<MonitorClient> {
        let mut client = MonitorClient::new_with_token(url, "");
        let token = client
            .post_string(
                "/auth/secret/login",
                json!({ "username": username.into(), "secret": secret.into() }),
            )
            .await
            .context("failed to log in with secret")?;
        client.token = token;
        Ok(client)
    }

    pub async fn new_with_new_account(
        url: &str,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> anyhow::Result<MonitorClient> {
        let mut client = MonitorClient::new_with_token(url, "");
        client.token = client.create_user(username, password).await?;
        Ok(client)
    }

    pub async fn create_user(
        &self,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> anyhow::Result<String> {
        self.post_string(
            "/auth/local/create_user",
            json!({ "username": username.into(), "password": password.into() }),
        )
        .await
    }

    pub async fn get_user(&self) -> anyhow::Result<String> {
        self.get_string("/api/user", Option::<String>::None).await
    }

    async fn get<R: DeserializeOwned>(
        &self,
        endpoint: &str,
        query: impl Serialize,
    ) -> anyhow::Result<R> {
        let res = self
            .http_client
            .get(format!("{}{endpoint}", self.url))
            .header("Authorization", format!("Bearer {}", self.token))
            .query(&query)
            .send()
            .await
            .context("failed to reach monitor api")?;
        let status = res.status();
        if status == StatusCode::OK {
            match res.json().await {
                Ok(res) => Ok(res),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        } else {
            match res.text().await {
                Ok(res) => Err(anyhow!("{status}: {res}")),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        }
    }

    async fn get_string(&self, endpoint: &str, query: impl Serialize) -> anyhow::Result<String> {
        let res = self
            .http_client
            .get(format!("{}{endpoint}", self.url))
            .header("Authorization", format!("Bearer {}", self.token))
            .query(&query)
            .send()
            .await
            .context("failed to reach monitor api")?;
        let status = res.status();
        if status == StatusCode::OK {
            match res.text().await {
                Ok(res) => Ok(res),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        } else {
            match res.text().await {
                Ok(res) => Err(anyhow!("{status}: {res}")),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        }
    }

    async fn post<B: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: impl Into<Option<B>>,
    ) -> anyhow::Result<R> {
        let req = self
            .http_client
            .post(format!("{}{endpoint}", self.url))
            .header("Authorization", format!("Bearer {}", self.token));
        let req = if let Some(body) = body.into() {
            req.header("Content-Type", "application/json").json(&body)
        } else {
            req
        };
        let res = req.send().await.context("failed to reach monitor api")?;
        let status = res.status();
        if status == StatusCode::OK {
            match res.json().await {
                Ok(res) => Ok(res),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        } else {
            match res.text().await {
                Ok(res) => Err(anyhow!("{status}: {res}")),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        }
    }

    async fn post_string<B: Serialize>(
        &self,
        endpoint: &str,
        body: impl Into<Option<B>>,
    ) -> anyhow::Result<String> {
        let req = self
            .http_client
            .post(format!("{}{endpoint}", self.url))
            .header("Authorization", format!("Bearer {}", self.token));
        let req = if let Some(body) = body.into() {
            req.header("Content-Type", "application/json").json(&body)
        } else {
            req
        };
        let res = req.send().await.context("failed to reach monitor api")?;
        let status = res.status();
        if status == StatusCode::OK {
            match res.text().await {
                Ok(res) => Ok(res),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        } else {
            match res.text().await {
                Ok(res) => Err(anyhow!("{status}: {res}")),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        }
    }

    async fn patch<B: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: impl Into<Option<B>>,
    ) -> anyhow::Result<R> {
        let req = self
            .http_client
            .patch(format!("{}{endpoint}", self.url))
            .header("Authorization", format!("Bearer {}", self.token));
        let req = if let Some(body) = body.into() {
            req.header("Content-Type", "application/json").json(&body)
        } else {
            req
        };
        let res = req.send().await.context("failed to reach monitor api")?;
        let status = res.status();
        if status == StatusCode::OK {
            match res.json().await {
                Ok(res) => Ok(res),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        } else {
            match res.text().await {
                Ok(res) => Err(anyhow!("{status}: {res}")),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        }
    }

    async fn patch_string<B: Serialize>(
        &self,
        endpoint: &str,
        body: impl Into<Option<B>>,
    ) -> anyhow::Result<String> {
        let req = self
            .http_client
            .patch(format!("{}{endpoint}", self.url))
            .header("Authorization", format!("Bearer {}", self.token));
        let req = if let Some(body) = body.into() {
            req.header("Content-Type", "application/json").json(&body)
        } else {
            req
        };
        let res = req.send().await.context("failed to reach monitor api")?;
        let status = res.status();
        if status == StatusCode::OK {
            match res.text().await {
                Ok(res) => Ok(res),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        } else {
            match res.text().await {
                Ok(res) => Err(anyhow!("{status}: {res}")),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        }
    }

    async fn delete<B: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: impl Into<Option<B>>,
    ) -> anyhow::Result<R> {
        let req = self
            .http_client
            .delete(format!("{}{endpoint}", self.url))
            .header("Authorization", format!("Bearer {}", self.token));
        let req = if let Some(body) = body.into() {
            req.header("Content-Type", "application/json").json(&body)
        } else {
            req
        };
        let res = req.send().await.context("failed to reach monitor api")?;
        let status = res.status();
        if status == StatusCode::OK {
            match res.json().await {
                Ok(res) => Ok(res),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        } else {
            match res.text().await {
                Ok(res) => Err(anyhow!("{status}: {res}")),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        }
    }

    async fn delete_string<B: Serialize>(
        &self,
        endpoint: &str,
        body: impl Into<Option<B>>,
    ) -> anyhow::Result<String> {
        let req = self
            .http_client
            .delete(format!("{}{endpoint}", self.url))
            .header("Authorization", format!("Bearer {}", self.token));
        let req = if let Some(body) = body.into() {
            req.header("Content-Type", "application/json").json(&body)
        } else {
            req
        };
        let res = req.send().await.context("failed to reach monitor api")?;
        let status = res.status();
        if status == StatusCode::OK {
            match res.text().await {
                Ok(res) => Ok(res),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        } else {
            match res.text().await {
                Ok(res) => Err(anyhow!("{status}: {res}")),
                Err(e) => Err(anyhow!("{status}: {e:#?}")),
            }
        }
    }
}

fn parse_url(url: &str) -> String {
    if url.chars().nth(url.len() - 1).unwrap() == '/' {
        url[..url.len() - 1].to_string()
    } else {
        url.to_string()
    }
}
