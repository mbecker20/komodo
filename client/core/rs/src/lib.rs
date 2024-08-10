//! # Monitor
//! *A system to build and deploy software accross many servers*
//!
//! This is a client library for the monitor core API.
//! It contains:
//! - Definitions for the application [api] and [entities].
//! - A [client][MonitorClient] to interact with the monitor core API.
//! - Information on configuring monitor [core][entities::config::core] and [periphery][entities::config::periphery].
//!
//! ## Client Configuration
//!
//! The client includes a convenenience method to parse the monitor url and credentials from the environment:
//! - MONITOR_ADDRESS
//! - MONITOR_API_KEY
//! - MONITOR_API_SECRET
//!
//! ## Client Example
//! ```
//! dotenvy::dotenv().ok();
//!
//! let client = MonitorClient::new_from_env()?;
//!
//! // Get all the deployments
//! let deployments = client.read(ListDeployments::default()).await?;
//!
//! println!("{deployments:#?}");
//!
//! let update = client.execute
//! ```

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
