use anyhow::{anyhow, Context};
use reqwest::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;
use serror::deserialize_error;

use crate::{
  api::{
    auth::MonitorAuthRequest, execute::MonitorExecuteRequest, read::MonitorReadRequest, user::MonitorUserRequest, write::MonitorWriteRequest
  },
  MonitorClient,
};

impl MonitorClient {
  #[tracing::instrument(skip(self))]
  pub async fn auth<T: MonitorAuthRequest>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response> {
    self
      .post(
        "/auth",
        json!({
          "type": T::req_type(),
          "params": request
        }),
      )
      .await
  }

  #[tracing::instrument(skip(self))]
  pub async fn user<T: MonitorUserRequest>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response> {
    self
      .post(
        "/auth",
        json!({
          "type": T::req_type(),
          "params": request
        }),
      )
      .await
  }

  #[tracing::instrument(skip(self))]
  pub async fn read<T: MonitorReadRequest>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response> {
    self
      .post(
        "/read",
        json!({
          "type": T::req_type(),
          "params": request
        }),
      )
      .await
  }

  #[tracing::instrument(skip(self))]
  pub async fn write<T: MonitorWriteRequest>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response> {
    self
      .post(
        "/write",
        json!({
          "type": T::req_type(),
          "params": request
        }),
      )
      .await
  }

  #[tracing::instrument(skip(self))]
  pub async fn execute<T: MonitorExecuteRequest>(
    &self,
    request: T,
  ) -> anyhow::Result<T::Response> {
    self
      .post(
        "/execute",
        json!({
          "type": T::req_type(),
          "params": request
        }),
      )
      .await
  }

  #[tracing::instrument(skip(self))]
  async fn post<
    B: Serialize + std::fmt::Debug,
    R: DeserializeOwned,
  >(
    &self,
    endpoint: &str,
    body: B,
  ) -> anyhow::Result<R> {
    let req = self
      .reqwest
      .post(format!("{}{endpoint}", self.address))
      .header("x-api-key", &self.key)
      .header("x-api-secret", &self.secret)
      .header("Content-Type", "application/json")
      .json(&body);
    let res =
      req.send().await.context("failed to reach monitor api")?;
    tracing::debug!("got response");
    let status = res.status();
    if status == StatusCode::OK {
      tracing::debug!("response is OK");
      match res.json().await {
        Ok(res) => Ok(res),
        Err(e) => Err(anyhow!("{status} | {e:#?}")),
      }
    } else {
      tracing::debug!("response is non-OK");
      match res.text().await {
        Ok(res) => Err(
          deserialize_error(res)
            .context(format!("request failed with status {status}")),
        ),
        Err(e) => Err(
          anyhow!("{e:?}")
            .context(format!("request failed with status {status}")),
        ),
      }
    }
  }
}
