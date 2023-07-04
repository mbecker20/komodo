use anyhow::{anyhow, Context};
use monitor_types::requests::auth::{
    self, CreateLocalUserResponse, LoginLocalUserResponse, LoginWithSecretResponse,
};
use reqwest::StatusCode;
use resolver_api::HasResponse;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct MonitorEnv {
    monitor_address: String,
    monitor_token: Option<String>,
    monitor_username: Option<String>,
    monitor_password: Option<String>,
    monitor_secret: Option<String>,
}

pub struct MonitorClient {
    reqwest: reqwest::Client,
    address: String,
    jwt: String,
}

impl MonitorClient {
    pub fn new_with_token(address: impl Into<String>, token: impl Into<String>) -> MonitorClient {
        MonitorClient {
            reqwest: Default::default(),
            address: address.into(),
            jwt: token.into(),
        }
    }

    pub async fn new_with_password(
        address: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> anyhow::Result<MonitorClient> {
        let mut client = MonitorClient {
            reqwest: Default::default(),
            address: address.into(),
            jwt: Default::default(),
        };

        let LoginLocalUserResponse { jwt } = client
            .auth(auth::LoginLocalUser {
                username: username.into(),
                password: password.into(),
            })
            .await?;

        client.jwt = jwt;

        Ok(client)
    }

    pub async fn new_with_new_account(
        address: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> anyhow::Result<MonitorClient> {
        let mut client = MonitorClient {
            reqwest: Default::default(),
            address: address.into(),
            jwt: Default::default(),
        };

        let CreateLocalUserResponse { jwt } = client
            .auth(auth::CreateLocalUser {
                username: username.into(),
                password: password.into(),
            })
            .await?;

        client.jwt = jwt;

        Ok(client)
    }

    pub async fn new_with_secret(
        address: impl Into<String>,
        username: impl Into<String>,
        secret: impl Into<String>,
    ) -> anyhow::Result<MonitorClient> {
        let mut client = MonitorClient {
            reqwest: Default::default(),
            address: address.into(),
            jwt: Default::default(),
        };

        let LoginWithSecretResponse { jwt } = client
            .auth(auth::LoginWithSecret {
                username: username.into(),
                secret: secret.into(),
            })
            .await?;

        client.jwt = jwt;

        Ok(client)
    }

    pub async fn new_from_env() -> anyhow::Result<MonitorClient> {
        let env = envy::from_env::<MonitorEnv>()
            .context("failed to parse environment for monitor client")?;
        if let Some(token) = env.monitor_token {
            Ok(MonitorClient::new_with_token(&env.monitor_address, token))
        } else if let Some(password) = env.monitor_password {
            let username = env.monitor_username.ok_or(anyhow!(
                "must provide MONITOR_USERNAME to authenticate with MONITOR_PASSWORD"
            ))?;
            MonitorClient::new_with_password(&env.monitor_address, username, password).await
        } else if let Some(secret) = env.monitor_secret {
            let username = env.monitor_username.ok_or(anyhow!(
                "must provide MONITOR_USERNAME to authenticate with MONITOR_SECRET"
            ))?;
            MonitorClient::new_with_secret(&env.monitor_address, username, secret).await
        } else {
            Err(anyhow!("failed to initialize monitor client from env | must provide one of: (MONITOR_TOKEN), (MONITOR_USERNAME and MONITOR_PASSWORD), (MONITOR_USERNAME and MONITOR_SECRET)"))
        }
    }

    pub async fn api<T: HasResponse>(&self, request: T) -> anyhow::Result<T::Response> {
        let req_type = T::req_type();
        self.post(
            "/api",
            json!({
                "type": req_type,
                "params": request
            }),
        )
        .await
    }

    pub async fn auth<T: HasResponse>(&self, request: T) -> anyhow::Result<T::Response> {
        let req_type = T::req_type();
        self.post(
            "/auth",
            json!({
                "type": req_type,
                "params": request
            }),
        )
        .await
    }

    async fn post<B: Serialize, R: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: impl Into<Option<B>>,
    ) -> anyhow::Result<R> {
        let req = self
            .reqwest
            .post(format!("{}{endpoint}", self.address))
            .header("Authorization", format!("Bearer {}", self.jwt));
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
}
