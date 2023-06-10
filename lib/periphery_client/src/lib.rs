#[macro_use]
extern crate log;

use anyhow::{anyhow, Context};
use reqwest::StatusCode;
use resolver_api::HasResponse;
use serde_json::json;

pub use periphery_api::requests;

pub struct PeripheryClient {
    reqwest: reqwest::Client,
    address: String,
    passkey: String,
}

impl PeripheryClient {
    pub fn new(address: impl Into<String>, passkey: impl Into<String>) -> PeripheryClient {
        PeripheryClient {
            reqwest: Default::default(),
            address: address.into(),
            passkey: passkey.into(),
        }
    }

    pub async fn request<T: HasResponse>(&self, request: T) -> anyhow::Result<T::Response> {
        let req_type = T::req_type();
        trace!("sending request | type: {req_type} | body: {request:?}");
        let res = self
            .reqwest
            .post(&self.address)
            .json(&json!({
                "type": req_type,
                "params": request
            }))
            .header("authorization", &self.passkey)
            .send()
            .await?;
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
            Err(anyhow!("request failed | {status} | {text}"))
        }
    }

    pub async fn health_check(&self) -> anyhow::Result<()> {
        self.request(requests::GetHealth {}).await?;
        Ok(())
    }
}
