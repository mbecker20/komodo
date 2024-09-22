use std::{path::Path, sync::OnceLock, time::Duration};

use anyhow::Context;
use reqwest::StatusCode;
use resolver_api::HasResponse;
use serde_json::json;
use serror::deserialize_error;

pub mod api;

static PERIPHERY_HTTP_CLIENT: OnceLock<reqwest::Client> =
  OnceLock::new();

/// Must call in Core Main to init the client with given ssl options.
pub fn init_periphery_http_client(
  accept_self_signed_certs: bool,
  ca_pem_path: &Path,
) {
  let mut client = reqwest::Client::builder()
    .danger_accept_invalid_certs(accept_self_signed_certs);

  let client = if ca_pem_path.is_file() {
    let cert = std::fs::read(ca_pem_path)
      .expect("failed to read ca pem contents");
    client.add_root_certificate(
      reqwest::Certificate::from_pem(&cert)
        .expect("invalid ca pem contents"),
    )
  } else if ca_pem_path.is_dir() {
    for entry in ca_pem_path.read_dir().unwrap() {
      let Ok(entry) = entry else {
        continue;
      };
      if entry.file_type().is_err()
        || !entry.file_type().unwrap().is_file()
      {
        continue;
      }
      let path = entry.path();
      let Some(extension) = path.extension() else {
        continue;
      };
      if extension != "pem" {
        continue;
      }
      let contents = std::fs::read(path).unwrap();
      client = client.add_root_certificate(
        reqwest::Certificate::from_pem(&contents).unwrap(),
      );
    }
    client
  } else {
    client
  };

  let client = client
    .build()
    .context("Invalid Periphery http client ssl configuration")
    .unwrap();

  PERIPHERY_HTTP_CLIENT
    .set(client)
    .expect("Client has already been set")
}

fn periphery_http_client() -> &'static reqwest::Client {
  PERIPHERY_HTTP_CLIENT
    .get()
    .expect("Called for periphery http client before `init_reqwest`")
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
    let mut req = periphery_http_client()
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
