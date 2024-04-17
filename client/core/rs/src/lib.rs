//! # Monitor
//! *A system to build and deploy software accross many servers*
//! 
//! This is a client library for the monitor core API.
//! It contains the definitions for all application [entities]
//! (eg. [Build][entities::build::Build], [Deployment][entities::deployment::Deployment], [Server][entities::server::Server], etc.),
//! api requests / responses (eg. [Deploy][api::execute::Deploy], [GetBuild][api::read::GetBuild]),
//! and usage guides.

use anyhow::Context;
use api::read::GetVersion;
use serde::Deserialize;

pub mod api;
pub mod busy;
pub mod entities;
pub mod ws;

mod request;

#[derive(Deserialize)]
struct MonitorEnv {
  monitor_address: String,
  monitor_api_key: String,
  monitor_api_secret: String,
}

#[derive(Clone)]
pub struct MonitorClient {
  reqwest: reqwest::Client,
  address: String,
  key: String,
  secret: String,
}

impl MonitorClient {
  #[tracing::instrument(skip_all)]
  pub async fn new(
    address: impl Into<String>,
    key: impl Into<String>,
    secret: impl Into<String>,
  ) -> anyhow::Result<MonitorClient> {
    let client = MonitorClient {
      reqwest: Default::default(),
      address: address.into(),
      key: key.into(),
      secret: secret.into(),
    };
    client.read(GetVersion {}).await?;
    Ok(client)
  }

  #[tracing::instrument]
  pub async fn new_from_env() -> anyhow::Result<MonitorClient> {
    let MonitorEnv {
      monitor_address,
      monitor_api_key,
      monitor_api_secret,
    } = envy::from_env()
      .context("failed to parse environment for monitor client")?;
    MonitorClient::new(
      monitor_address,
      monitor_api_key,
      monitor_api_secret,
    )
    .await
  }
}
