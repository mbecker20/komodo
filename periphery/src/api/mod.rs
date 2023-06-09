use anyhow::{anyhow, Context};
use monitor_types::{periphery_api::{requests::{GetVersionResponse, GetHealth, GetHealthResponse, GetVersion}, PeripheryRequest}, Resolve};

use crate::state::State;

mod system_stats;

impl State {
    pub async fn handle_request(&self, request: PeripheryRequest) -> anyhow::Result<String> {
        match request {
            PeripheryRequest::GetHealth(_) => Ok(String::from("{}")),
            PeripheryRequest::GetVersion(_) => get_version(),
            PeripheryRequest::GetSystemInformation(_) => Ok(self.stats.read().await.info.clone()),
            _ => Err(anyhow!("not implemented")),
        }
    }
}

fn get_version() -> anyhow::Result<String> {
    serde_json::to_string(&GetVersionResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
    .context("failed to convert version to string")
}

#[async_trait::async_trait]
impl Resolve<GetHealth> for State {
    async fn resolve(&self, _: GetHealth) -> GetHealthResponse {
        GetHealthResponse {}
    }
}

#[async_trait::async_trait]
impl Resolve<GetVersion> for State {
    async fn resolve(&self, _: GetVersion) -> GetVersionResponse {
        todo!()
    }
}
