use anyhow::Context;
use periphery_client::api::stats::{
  GetSystemInformation, GetSystemProcesses, GetSystemStats,
};
use resolver_api::ResolveToString;

use crate::{stats::stats_client, State};

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

// impl ResolveToString<GetTotalNetworkStats> for State {
//   #[instrument(
//     name = "GetTotalNetworkStats", 
//     level = "debug", 
//     skip(self)
//   )]
//   async fn resolve_to_string(
//       &self,
//       _: GetTotalNetworkStats,
//       _: (),
//   ) -> anyhow::Result<String> {
//       let network_stats = stats_client().read().await.get_network_stats();
//       // Serialize the stats to a string
//       serde_json::to_string(&network_stats)
//           .context("Failed to serialize total network stats")
//   }
// }

// impl ResolveToString<GetNetworkStatsByInterface> for State {
//   #[instrument(
//     name = "GetNetworkStatsByInterface", 
//     level = "debug", 
//     skip(self)
//   )]
//   async fn resolve_to_string(
//       &self,
//       GetNetworkStatsByInterface { interface_name }: GetNetworkStatsByInterface,
//       _: (),
//   ) -> anyhow::Result<String> {
//       let client = stats_client().read().await;
//       match client.get_network_stats_by_interface(&interface_name) {
//         Some(stats) => serde_json::to_string(&stats)
//             .context(format!("Failed to serialize network stats for interface: {}", interface_name)),
//         None => Err(anyhow::anyhow!("Interface not found").into()),
//     }
//   }
// }
