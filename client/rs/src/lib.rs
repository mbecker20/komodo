use anyhow::{anyhow, Context};
use serde::Deserialize;

pub mod api;
pub mod busy;
pub mod entities;
pub mod permissioned;

mod request;
mod subscribe;

#[derive(Deserialize)]
struct MonitorEnv {
  monitor_address: String,
  monitor_token: Option<String>,
  monitor_username: Option<String>,
  monitor_password: Option<String>,
  monitor_secret: Option<String>,
}

#[derive(Clone)]
pub struct MonitorClient {
  reqwest: reqwest::Client,
  address: String,
  jwt: String,
  creds: Option<RefreshTokenCreds>,
}

#[derive(Clone)]
struct RefreshTokenCreds {
  username: String,
  secret: String,
}

impl MonitorClient {
  pub fn new_with_token(
    address: impl Into<String>,
    token: impl Into<String>,
  ) -> MonitorClient {
    MonitorClient {
      reqwest: Default::default(),
      address: address.into(),
      jwt: token.into(),
      creds: None,
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
      creds: None,
    };

    let api::auth::LoginLocalUserResponse { jwt } = client
      .auth(api::auth::LoginLocalUser {
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
      creds: None,
    };

    let api::auth::CreateLocalUserResponse { jwt } = client
      .auth(api::auth::CreateLocalUser {
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
      creds: RefreshTokenCreds {
        username: username.into(),
        secret: secret.into(),
      }
      .into(),
    };

    client.refresh_jwt().await?;

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
      MonitorClient::new_with_password(
        &env.monitor_address,
        username,
        password,
      )
      .await
    } else if let Some(secret) = env.monitor_secret {
      let username = env.monitor_username.ok_or(anyhow!(
                "must provide MONITOR_USERNAME to authenticate with MONITOR_SECRET"
            ))?;
      MonitorClient::new_with_secret(
        &env.monitor_address,
        username,
        secret,
      )
      .await
    } else {
      Err(anyhow!("failed to initialize monitor client from env | must provide one of: (MONITOR_TOKEN), (MONITOR_USERNAME and MONITOR_PASSWORD), (MONITOR_USERNAME and MONITOR_SECRET)"))
    }
  }

  pub async fn refresh_jwt(&mut self) -> anyhow::Result<()> {
    if self.creds.is_none() {
      return Err(anyhow!(
                "only clients initialized using the secret login method can refresh their jwt"
            ));
    }

    let creds = self.creds.clone().unwrap();

    let api::auth::LoginWithSecretResponse { jwt } = self
      .auth(api::auth::LoginWithSecret {
        username: creds.username,
        secret: creds.secret,
      })
      .await?;

    self.jwt = jwt;

    Ok(())
  }
}
