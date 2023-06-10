use anyhow::anyhow;
use monitor_types::api::{
    periphery::{
        requests::{GetHealth, GetVersion, GetVersionResponse},
        PeripheryRequest,
    },
    Resolve, ResolveToString,
};

use crate::state::State;

mod system_stats;

impl State {
    pub async fn handle_request(&self, request: PeripheryRequest) -> anyhow::Result<String> {
        match request {
            PeripheryRequest::GetHealth(req) => self.resolve_to_string(req).await,
            PeripheryRequest::GetVersion(req) => self.resolve_to_json(req).await,
            // system stats
            PeripheryRequest::GetSystemInformation(req) => self.resolve_to_string(req).await,
            PeripheryRequest::GetAllSystemStats(req) => self.resolve_to_string(req).await,
            PeripheryRequest::GetBasicSystemStats(req) => self.resolve_to_string(req).await,
            PeripheryRequest::GetCpuUsage(req) => self.resolve_to_string(req).await,
            PeripheryRequest::GetDiskUsage(req) => self.resolve_to_string(req).await,
            PeripheryRequest::GetNetworkUsage(req) => self.resolve_to_string(req).await,
            PeripheryRequest::GetSystemProcesses(req) => self.resolve_to_string(req).await,
            PeripheryRequest::GetSystemComponents(req) => self.resolve_to_string(req).await,
            // 
            _ => Err(anyhow!("not implemented")),
        }
    }
}

#[async_trait::async_trait]
impl ResolveToString<GetHealth> for State {
    async fn resolve_to_string(&self, _: GetHealth) -> anyhow::Result<String> {
        Ok(String::from("{}"))
    }
}

#[async_trait::async_trait]
impl Resolve<GetVersion> for State {
    async fn resolve(&self, _: GetVersion) -> anyhow::Result<GetVersionResponse> {
        Ok(GetVersionResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }
}
