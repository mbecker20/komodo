use anyhow::{anyhow, Context};
use monitor_types::requests::{
  auth::MonitorAuthRequest, execute::MonitorExecuteRequest,
  read::MonitorReadRequest, write::MonitorWriteRequest,
};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use serror::deserialize_error;

use crate::MonitorClient;

impl MonitorClient {
  pub async fn auth<T: MonitorAuthRequest>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response> {
    let req_type = T::req_type();
    self
      .post(
        "/auth",
        json!({
            "type": req_type,
            "params": request
        }),
      )
      .await
  }

  pub async fn read<T: MonitorReadRequest>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response> {
    let req_type = T::req_type();
    self
      .post(
        "/read",
        json!({
            "type": req_type,
            "params": request
        }),
      )
      .await
  }

  pub async fn write<T: MonitorWriteRequest>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response> {
    let req_type = T::req_type();
    self
      .post(
        "/write",
        json!({
            "type": req_type,
            "params": request
        }),
      )
      .await
  }

  pub async fn execute<T: MonitorExecuteRequest>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response> {
    let req_type = T::req_type();
    self
      .post(
        "/execute",
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
    let res =
      req.send().await.context("failed to reach monitor api")?;
    let status = res.status();
    if status == StatusCode::OK {
      match res.json().await {
        Ok(res) => Ok(res),
        Err(e) => Err(anyhow!("{status} | {e:#?}")),
      }
    } else {
      match res.text().await {
        Ok(res) => Err(deserialize_error(res).context(status)),
        Err(e) => Err(anyhow!("{status} | {e:#?}")),
      }
    }
  }
}
