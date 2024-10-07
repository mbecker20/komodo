//! # Komodo
//! *A system to build and deploy software accross many servers*
//!
//! This is a client library for the Komodo Core API.
//! It contains:
//! - Definitions for the application [api] and [entities].
//! - A [client][KomodoClient] to interact with the Komodo Core API.
//! - Information on configuring Komodo [core][entities::config::core] and [periphery][entities::config::periphery].
//!
//! ## Client Configuration
//!
//! The client includes a convenenience method to parse the Komodo API url and credentials from the environment:
//! - KOMODO_ADDRESS
//! - KOMODO_API_KEY
//! - KOMODO_API_SECRET
//!
//! ## Client Example
//! ```
//! dotenvy::dotenv().ok();
//!
//! let client = KomodoClient::new_from_env()?;
//!
//! // Get all the deployments
//! let deployments = client.read(ListDeployments::default()).await?;
//!
//! println!("{deployments:#?}");
//!
//! let update = client.execute(RunBuild { build: "test-build".to_string() }).await?:
//! ```

use anyhow::Context;
use api::read::GetVersion;
use serde::Deserialize;

pub mod api;
pub mod busy;
pub mod entities;
pub mod ws;

mod parser;
mod request;

#[derive(Deserialize)]
struct KomodoEnv {
  komodo_address: String,
  komodo_api_key: String,
  komodo_api_secret: String,
}

#[derive(Clone)]
pub struct KomodoClient {
  reqwest: reqwest::Client,
  address: String,
  key: String,
  secret: String,
}

impl KomodoClient {
  #[tracing::instrument(skip_all)]
  pub async fn new(
    address: impl Into<String>,
    key: impl Into<String>,
    secret: impl Into<String>,
  ) -> anyhow::Result<KomodoClient> {
    let client = KomodoClient {
      reqwest: Default::default(),
      address: address.into(),
      key: key.into(),
      secret: secret.into(),
    };
    client.read(GetVersion {}).await?;
    Ok(client)
  }

  #[tracing::instrument]
  pub async fn new_from_env() -> anyhow::Result<KomodoClient> {
    let KomodoEnv {
      komodo_address,
      komodo_api_key,
      komodo_api_secret,
    } = envy::from_env()
      .context("failed to parse environment for komodo client")?;
    KomodoClient::new(
      komodo_address,
      komodo_api_key,
      komodo_api_secret,
    )
    .await
  }
}
