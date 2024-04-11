use std::{sync::OnceLock, time::Duration};

use anyhow::Context;
use reqwest::StatusCode;
use resolver_api::HasResponse;
use serde_json::json;
use serror::deserialize_error;

pub mod api;

fn http_client() -> &'static reqwest::Client {
  static PERIPHERY_HTTP_CLIENT: OnceLock<reqwest::Client> =
    OnceLock::new();
  PERIPHERY_HTTP_CLIENT.get_or_init(Default::default)
}

pub struct PeripheryClient {
  address: String,
  passkey: String,
}

impl PeripheryClient {
  pub fn new(
    address: impl Into<String>,
    passkey: impl Into<String>,
  ) -> PeripheryClient {
    PeripheryClient {
      address: address.into(),
      passkey: passkey.into(),
    }
  }

  // tracing will skip self, to avoid including passkey in traces
  #[tracing::instrument(
    name = "PeripheryRequest",
    level = "debug",
    skip(self)
  )]
  pub async fn request<T: HasResponse>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response> {
    tracing::debug!("running health check");
    self.health_check().await?;
    tracing::debug!("health check passed. running inner request");
    self.request_inner(request, None).await
  }

  #[tracing::instrument(level = "debug", skip(self))]
  pub async fn health_check(&self) -> anyhow::Result<()> {
    self
      .request_inner(api::GetHealth {}, Some(Duration::from_secs(1)))
      .await?;
    Ok(())
  }

  #[tracing::instrument(level = "debug", skip(self))]
  async fn request_inner<T: HasResponse>(
    &self,
    request: T,
    timeout: Option<Duration>,
  ) -> anyhow::Result<T::Response> {
    let req_type = T::req_type();
    tracing::trace!(
      "sending request | type: {req_type} | body: {request:?}"
    );
    let mut req = http_client()
      .post(&self.address)
      .json(&json!({
        "type": req_type,
        "params": request
      }))
      .header("authorization", &self.passkey);
    if let Some(timeout) = timeout {
      req = req.timeout(timeout);
    }
    let res =
      req.send().await.context("failed at request to periphery")?;
    let status = res.status();
    tracing::debug!(
      "got response | type: {req_type} | {status} | body: {res:?}",
    );
    if status == StatusCode::OK {
      tracing::debug!("response ok, deserializing");
      res.json().await.with_context(|| format!(
        "failed to parse response to json | type: {req_type} | body: {request:?}"
      ))
    } else {
      tracing::debug!("response is non-200");

      let text = res
        .text()
        .await
        .context("failed to convert response to text")?;

      tracing::debug!("got response text, deserializing error");

      let error = deserialize_error(text)
        .context(format!("request to periphery failed | {status}"));

      Err(error)
    }
  }
}
