use anyhow::Context;
use periphery_client::api::stats::{
  GetAllSystemStats, GetBasicSystemStats, GetCpuUsage, GetDiskUsage,
  GetNetworkUsage, GetSystemComponents, GetSystemInformation,
  GetSystemProcesses,
};
use resolver_api::ResolveToString;

use crate::{system_stats::stats_client, State};

#[async_trait::async_trait]
impl ResolveToString<GetSystemInformation> for State {
  async fn resolve_to_string(
    &self,
    _: GetSystemInformation,
    _: (),
  ) -> anyhow::Result<String> {
    let info = &stats_client().read().await.info;
    serde_json::to_string(info)
      .context("failed to serialize response to string")
  }
}

//

#[async_trait::async_trait]
impl ResolveToString<GetAllSystemStats> for State {
  async fn resolve_to_string(
    &self,
    _: GetAllSystemStats,
    _: (),
  ) -> anyhow::Result<String> {
    let stats = &stats_client().read().await.stats;
    serde_json::to_string(stats)
      .context("failed to serialize response to string")
  }
}

//

#[async_trait::async_trait]
impl ResolveToString<GetBasicSystemStats> for State {
  async fn resolve_to_string(
    &self,
    _: GetBasicSystemStats,
    _: (),
  ) -> anyhow::Result<String> {
    let stats = &stats_client().read().await.stats.basic;
    serde_json::to_string(stats)
      .context("failed to serialize response to string")
  }
}

//

#[async_trait::async_trait]
impl ResolveToString<GetCpuUsage> for State {
  async fn resolve_to_string(
    &self,
    _: GetCpuUsage,
    _: (),
  ) -> anyhow::Result<String> {
    let stats = &stats_client().read().await.stats.cpu;
    serde_json::to_string(stats)
      .context("failed to serialize response to string")
  }
}

//

#[async_trait::async_trait]
impl ResolveToString<GetDiskUsage> for State {
  async fn resolve_to_string(
    &self,
    _: GetDiskUsage,
    _: (),
  ) -> anyhow::Result<String> {
    let stats = &stats_client().read().await.stats.disk;
    serde_json::to_string(stats)
      .context("failed to serialize response to string")
  }
}

//

#[async_trait::async_trait]
impl ResolveToString<GetNetworkUsage> for State {
  async fn resolve_to_string(
    &self,
    _: GetNetworkUsage,
    _: (),
  ) -> anyhow::Result<String> {
    let stats = &stats_client().read().await.stats.network;
    serde_json::to_string(&stats)
      .context("failed to serialize response to string")
  }
}

//

#[async_trait::async_trait]
impl ResolveToString<GetSystemProcesses> for State {
  async fn resolve_to_string(
    &self,
    _: GetSystemProcesses,
    _: (),
  ) -> anyhow::Result<String> {
    let stats = &stats_client().read().await.stats.processes;
    serde_json::to_string(&stats)
      .context("failed to serialize response to string")
  }
}

//

#[async_trait::async_trait]
impl ResolveToString<GetSystemComponents> for State {
  async fn resolve_to_string(
    &self,
    _: GetSystemComponents,
    _: (),
  ) -> anyhow::Result<String> {
    let stats = &stats_client().read().await.stats.components;
    serde_json::to_string(&stats)
      .context("failed to serialize response to string")
  }
}
