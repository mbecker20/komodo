#[macro_use]
extern crate log;

use std::time::Duration;

use anyhow::Context;
use reqwest::StatusCode;
use resolver_api::HasResponse;
use serde_json::json;

pub use monitor_periphery::requests;
use serror::deserialize_error;

pub struct PeripheryClient {
    reqwest: reqwest::Client,
    address: String,
    passkey: String,
}

impl PeripheryClient {
    pub fn new(
        address: impl Into<String>,
        passkey: impl Into<String>,
    ) -> PeripheryClient {
        PeripheryClient {
            reqwest: Default::default(),
            address: address.into(),
            passkey: passkey.into(),
        }
    }

    pub async fn request<T: HasResponse>(
        &self,
        request: T,
    ) -> anyhow::Result<T::Response> {
        self.health_check().await?;
        self.request_inner(request, None).await
    }

    pub async fn health_check(&self) -> anyhow::Result<()> {
        self.request_inner(
            requests::GetHealth {},
            Some(Duration::from_secs(1)),
        )
        .await?;
        Ok(())
    }

    async fn request_inner<T: HasResponse>(
        &self,
        request: T,
        timeout: Option<Duration>,
    ) -> anyhow::Result<T::Response> {
        let req_type = T::req_type();
        trace!(
            "sending request | type: {req_type} | body: {request:?}"
        );
        let mut req = self
            .reqwest
            .post(&self.address)
            .json(&json!({
                "type": req_type,
                "params": request
            }))
            .header("authorization", &self.passkey);
        if let Some(timeout) = timeout {
            req = req.timeout(timeout);
        }
        let res = req
            .send()
            .await
            .context("failed at request to periphery")?;
        let status = res.status();
        debug!("got response | type: {req_type} | {status} | body: {res:?}",);
        if status == StatusCode::OK {
            res.json().await.context(format!(
                "failed to parse response to json | type: {req_type} | body: {request:?}"
            ))
        } else {
            let text = res
                .text()
                .await
                .context("failed to convert response to text")?;

            let error = deserialize_error(text).context(format!(
                "request to periphery failed | {status}"
            ));

            Err(error)
        }
    }
}
