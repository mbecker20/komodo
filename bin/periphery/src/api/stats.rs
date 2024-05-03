use anyhow::Context;
use periphery_client::api::stats::{
  GetSystemInformation, GetSystemProcesses, GetSystemStats,
};
use resolver_api::ResolveToString;

use crate::{stats::stats_client, State};

#[async_trait::async_trait]
impl ResolveToString<GetSystemInformation> for State {
  #[instrument(
    name = "GetSystemInformation",
    level = "debug",
    skip(self)
  )]
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
impl ResolveToString<GetSystemStats> for State {
  #[instrument(name = "GetSystemStats", level = "debug", skip(self))]
  async fn resolve_to_string(
    &self,
    _: GetSystemStats,
    _: (),
  ) -> anyhow::Result<String> {
    let stats = &stats_client().read().await.stats;
    serde_json::to_string(stats)
      .context("failed to serialize response to string")
  }
}

//

#[async_trait::async_trait]
impl ResolveToString<GetSystemProcesses> for State {
  #[instrument(
    name = "GetSystemProcesses",
    level = "debug",
    skip(self)
  )]
  async fn resolve_to_string(
    &self,
    _: GetSystemProcesses,
    _: (),
  ) -> anyhow::Result<String> {
    let stats = &stats_client().read().await.get_processes();
    serde_json::to_string(&stats)
      .context("failed to serialize response to string")
  }
}
