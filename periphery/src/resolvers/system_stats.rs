use anyhow::Context;
use monitor_types::api::ResolveToString;
use periphery_api::requests::{
    GetAllSystemStats, GetBasicSystemStats, GetCpuUsage, GetDiskUsage, GetNetworkUsage,
    GetSystemComponents, GetSystemInformation, GetSystemProcesses,
};

use crate::state::State;

#[async_trait::async_trait]
impl ResolveToString<GetSystemInformation> for State {
    async fn resolve_to_string(&self, _: GetSystemInformation) -> anyhow::Result<String> {
        let info = &self.stats.read().await.info;
        serde_json::to_string(info).context("failed to serialize response to string")
    }
}

#[async_trait::async_trait]
impl ResolveToString<GetAllSystemStats> for State {
    async fn resolve_to_string(&self, _: GetAllSystemStats) -> anyhow::Result<String> {
        let stats = &self.stats.read().await.stats;
        serde_json::to_string(stats).context("failed to serialize response to string")
    }
}

#[async_trait::async_trait]
impl ResolveToString<GetBasicSystemStats> for State {
    async fn resolve_to_string(&self, _: GetBasicSystemStats) -> anyhow::Result<String> {
        let stats = &self.stats.read().await.stats.basic;
        serde_json::to_string(stats).context("failed to serialize response to string")
    }
}

#[async_trait::async_trait]
impl ResolveToString<GetCpuUsage> for State {
    async fn resolve_to_string(&self, _: GetCpuUsage) -> anyhow::Result<String> {
        let stats = &self.stats.read().await.stats.cpu;
        serde_json::to_string(stats).context("failed to serialize response to string")
    }
}

#[async_trait::async_trait]
impl ResolveToString<GetDiskUsage> for State {
    async fn resolve_to_string(&self, _: GetDiskUsage) -> anyhow::Result<String> {
        let stats = &self.stats.read().await.stats.disk;
        serde_json::to_string(stats).context("failed to serialize response to string")
    }
}

#[async_trait::async_trait]
impl ResolveToString<GetNetworkUsage> for State {
    async fn resolve_to_string(&self, _: GetNetworkUsage) -> anyhow::Result<String> {
        let stats = &self.stats.read().await.stats.network;
        serde_json::to_string(&stats).context("failed to serialize response to string")
    }
}

#[async_trait::async_trait]
impl ResolveToString<GetSystemProcesses> for State {
    async fn resolve_to_string(&self, _: GetSystemProcesses) -> anyhow::Result<String> {
        let stats = &self.stats.read().await.stats.processes;
        serde_json::to_string(&stats).context("failed to serialize response to string")
    }
}

#[async_trait::async_trait]
impl ResolveToString<GetSystemComponents> for State {
    async fn resolve_to_string(&self, _: GetSystemComponents) -> anyhow::Result<String> {
        let stats = &self.stats.read().await.stats.componenets;
        serde_json::to_string(&stats).context("failed to serialize response to string")
    }
}
