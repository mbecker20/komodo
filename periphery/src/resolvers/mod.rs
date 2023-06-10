use periphery_api::requests::{
    GetAccounts, GetAllSystemStats, GetBasicSystemStats, GetCpuUsage, GetDiskUsage, GetHealth,
    GetNetworkUsage, GetSecrets, GetSystemComponents, GetSystemInformation, GetSystemProcesses,
    GetVersion, GetVersionResponse,
};
use resolver_api::{derive::Resolver, Resolve, ResolveToString};
use serde::{Deserialize, Serialize};

use crate::state::State;

mod system_stats;

#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[serde(tag = "type", content = "params")]
#[resolver_target(State)]
#[allow(clippy::enum_variant_names)]
pub enum PeripheryRequest {
    // GET
    #[to_string_resolver]
    GetHealth(GetHealth),
    GetVersion(GetVersion),
    #[to_string_resolver]
    GetSystemInformation(GetSystemInformation),
    #[to_string_resolver]
    GetAllSystemStats(GetAllSystemStats),
    #[to_string_resolver]
    GetBasicSystemStats(GetBasicSystemStats),
    #[to_string_resolver]
    GetCpuUsage(GetCpuUsage),
    #[to_string_resolver]
    GetDiskUsage(GetDiskUsage),
    #[to_string_resolver]
    GetNetworkUsage(GetNetworkUsage),
    #[to_string_resolver]
    GetSystemProcesses(GetSystemProcesses),
    #[to_string_resolver]
    GetSystemComponents(GetSystemComponents),
    #[to_string_resolver]
    GetAccounts(GetAccounts),
    #[to_string_resolver]
    GetSecrets(GetSecrets),
    // GetContainerList {},
    // GetContainerLog {},
    // GetContainerStats {},
    // GetContainerStatsList {},
    // GetNetworkList {},

    // ACTIONS
    // RunCommand(SystemCommand),
    // CloneRepo {},
    // PullRepo {},
    // DeleteRepo {},
    // Build {},
    // Deploy {},
    // StartContainer {},
    // StopContainer {},
    // RemoveContainer {},
    // RenameContainer {},
    // PruneContainers {},
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

#[async_trait::async_trait]
impl ResolveToString<GetAccounts> for State {
    async fn resolve_to_string(&self, _: GetAccounts) -> anyhow::Result<String> {
        Ok(self.accounts_response.clone())
    }
}

#[async_trait::async_trait]
impl ResolveToString<GetSecrets> for State {
    async fn resolve_to_string(&self, _: GetSecrets) -> anyhow::Result<String> {
        Ok(self.secrets_response.clone())
    }
}
