use std::sync::OnceLock;

use anyhow::{anyhow, Context};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};

use self::create_server::{CreateServerBody, CreateServerResponse};

pub mod common;
pub mod create_server;

const BASE_URL: &str = "https://api.hetzner.cloud/v1";

pub struct HetznerClient {
  client: reqwest::Client,
  token: String,
}

impl HetznerClient {
  fn new(token: &str) -> HetznerClient {
    HetznerClient {
      client: Default::default(),
      token: format!("Bearer {token}"),
    }
  }

  pub async fn create_server(
    &self,
    body: &CreateServerBody,
  ) -> anyhow::Result<CreateServerResponse> {
    self.post("/servers", body).await
  }

  async fn post<Body: Serialize, Res: DeserializeOwned>(
    &self,
    path: &str,
    body: &Body,
  ) -> anyhow::Result<Res> {
    let res = self
      .client
      .post(format!("{BASE_URL}{path}"))
      .json(&body)
      .header("authorization", &self.token)
      .send()
      .await
      .context("failed to make request to Hetzner")?;

    let status = res.status();

    if status == StatusCode::OK {
      res.json().await.context("failed to parse response to json")
    } else {
      let text = res
        .text()
        .await
        .context("failed to get response body as text")?;
      Err(anyhow!("FAILED | {path} | {status} | {text}"))
    }
  }
}
