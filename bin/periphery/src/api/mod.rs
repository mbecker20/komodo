use anyhow::Context;
use async_trait::async_trait;
use monitor_client::entities::{update::Log, SystemCommand};
use periphery_client::api::{
  build::*, container::*, git::*, network::*, stats::*, GetAccounts,
  GetHealth, GetSecrets, GetVersion, GetVersionResponse, PruneAll,
  RunCommand,
};
use resolver_api::{derive::Resolver, Resolve, ResolveToString};
use serde::{Deserialize, Serialize};

use crate::{
  config::{accounts_response, secrets_response},
  helpers::{docker, run_monitor_command},
  State,
};

mod build;
mod container;
mod git;
mod network;
mod stats;

#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[serde(tag = "type", content = "params")]
#[resolver_target(State)]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
pub enum PeripheryRequest {
  // GET
  GetVersion(GetVersion),
  #[to_string_resolver]
  GetHealth(GetHealth),
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
  GetContainerList(GetContainerList),
  GetContainerLog(GetContainerLog),
  GetContainerStats(GetContainerStats),
  GetContainerStatsList(GetContainerStatsList),
  GetNetworkList(GetNetworkList),
  // ACTIONS
  RunCommand(RunCommand),
  CloneRepo(CloneRepo),
  PullRepo(PullRepo),
  DeleteRepo(DeleteRepo),
  Build(Build),
  PruneImages(PruneImages),
  Deploy(Deploy),
  StartContainer(StartContainer),
  StopContainer(StopContainer),
  RemoveContainer(RemoveContainer),
  RenameContainer(RenameContainer),
  PruneContainers(PruneContainers),
  CreateNetwork(CreateNetwork),
  DeleteNetwork(DeleteNetwork),
  PruneNetworks(PruneNetworks),
  PruneAll(PruneAll),
}

//

#[async_trait]
impl ResolveToString<GetHealth> for State {
  async fn resolve_to_string(
    &self,
    _: GetHealth,
    _: (),
  ) -> anyhow::Result<String> {
    Ok(String::from("{}"))
  }
}

//

#[async_trait]
impl Resolve<GetVersion> for State {
  async fn resolve(
    &self,
    _: GetVersion,
    _: (),
  ) -> anyhow::Result<GetVersionResponse> {
    Ok(GetVersionResponse {
      version: env!("CARGO_PKG_VERSION").to_string(),
    })
  }
}

//

#[async_trait]
impl ResolveToString<GetAccounts> for State {
  async fn resolve_to_string(
    &self,
    _: GetAccounts,
    _: (),
  ) -> anyhow::Result<String> {
    Ok(accounts_response().clone())
  }
}

//

#[async_trait]
impl ResolveToString<GetSecrets> for State {
  async fn resolve_to_string(
    &self,
    _: GetSecrets,
    _: (),
  ) -> anyhow::Result<String> {
    Ok(secrets_response().clone())
  }
}

#[async_trait]
impl Resolve<RunCommand> for State {
  async fn resolve(
    &self,
    RunCommand {
      command: SystemCommand { path, command },
    }: RunCommand,
    _: (),
  ) -> anyhow::Result<Log> {
    tokio::spawn(async move {
      let command = if path.is_empty() {
        command
      } else {
        format!("cd {path} && {command}")
      };
      run_monitor_command("run command", command).await
    })
    .await
    .context("failure in spawned task")
  }
}

#[async_trait]
impl Resolve<PruneAll> for State {
  async fn resolve(
    &self,
    PruneAll {}: PruneAll,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(docker::prune_system().await)
  }
}
