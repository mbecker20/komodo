use anyhow::{anyhow, Context};
use monitor_types::periphery_api::{requests::GetVersionResponse, PeripheryRequest};

use crate::state::State;

impl State {
    pub async fn handle_request(&self, request: PeripheryRequest) -> anyhow::Result<String> {
        match request {
            PeripheryRequest::GetHealth(_) => Ok(String::from("{}")),
            PeripheryRequest::GetVersion(_) => State::get_version(),
            PeripheryRequest::GetSystemInformation {} => todo!(),
            _ => Err(anyhow!("not implemented")),
        }
    }

    pub fn get_version() -> anyhow::Result<String> {
        serde_json::to_string(&GetVersionResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
        .context("failed to convert version to string")
    }
}
