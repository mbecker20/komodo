// #![allow(unused)]

use anyhow::{anyhow, Context};
use monitor_types::User;
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;

pub use futures_util;
pub use tokio_tungstenite;

pub use monitor_types as types;

mod build;
mod deployment;
mod group;
mod permissions;
mod procedure;
mod secret;
mod server;
mod update;

#[derive(Deserialize)]
struct MonitorEnv {
    monitor_url: String,
    monitor_token: Option<String>,
    monitor_username: Option<String>,
    monitor_password: Option<String>,
    monitor_secret: Option<String>,
}

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

    pub async fn new_from_env() -> anyhow::Result<MonitorClient> {
        let env = envy::from_env::<MonitorEnv>()
            .context("failed to parse environment for monitor client")?;
        if let Some(token) = env.monitor_token {
            Ok(MonitorClient::new_with_token(&env.monitor_url, token))
        } else if let Some(password) = env.monitor_password {
            let username = env.monitor_username.ok_or(anyhow!(
                "must provide MONITOR_USERNAME to authenticate with MONITOR_PASSWORD"
            ))?;
            MonitorClient::new_with_password(&env.monitor_url, username, password).await
        } else if let Some(secret) = env.monitor_secret {
            let username = env.monitor_username.ok_or(anyhow!(
                "must provide MONITOR_USERNAME to authenticate with MONITOR_SECRET"
            ))?;
            MonitorClient::new_with_secret(&env.monitor_url, username, secret).await
        } else {
            Err(anyhow!("failed to initialize monitor client from env | must provide one of: (MONITOR_TOKEN), (MONITOR_USERNAME and MONITOR_PASSWORD), (MONITOR_USERNAME and MONITOR_SECRET)"))
        }
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

    pub async fn get_user(&self) -> anyhow::Result<User> {
        self.get("/api/user", Option::<()>::None).await
    }

    pub async fn get_username(&self, user_id: &str) -> anyhow::Result<String> {
        self.get_string(&format!("/api/username/{user_id}"), Option::<()>::None)
            .await
    }

    pub async fn list_users(&self) -> anyhow::Result<Vec<User>> {
        self.get("/api/users", Option::<()>::None).await
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

    async fn _patch_string<B: Serialize>(
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

    async fn _delete_string<B: Serialize>(
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
