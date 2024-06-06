use anyhow::Context;
use command::run_monitor_command;
use monitor_client::entities::{update::Log, SystemCommand};
use periphery_client::api::{
  build::*, container::*, git::*, network::*, stats::*, GetAccounts,
  GetHealth, GetSecrets, GetVersion, GetVersionResponse, PruneSystem,
  RunCommand,
};
use resolver_api::{derive::Resolver, Resolve, ResolveToString};
use serde::{Deserialize, Serialize};

use crate::{
  config::{accounts_response, secrets_response},
  docker, State,
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
  GetVersion(GetVersion),
  #[to_string_resolver]
  GetHealth(GetHealth),

  // Config
  #[to_string_resolver]
  GetAccounts(GetAccounts),
  #[to_string_resolver]
  GetSecrets(GetSecrets),

  // Stats / Info
  #[to_string_resolver]
  GetSystemInformation(GetSystemInformation),
  #[to_string_resolver]
  GetSystemStats(GetSystemStats),
  #[to_string_resolver]
  GetSystemProcesses(GetSystemProcesses),
  GetLatestCommit(GetLatestCommit),

  // Docker
  GetContainerList(GetContainerList),
  GetContainerLog(GetContainerLog),
  GetContainerLogSearch(GetContainerLogSearch),
  GetContainerStats(GetContainerStats),
  GetContainerStatsList(GetContainerStatsList),
  GetNetworkList(GetNetworkList),

  // Actions
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
  PruneAll(PruneSystem),
}

//

impl ResolveToString<GetHealth> for State {
  #[instrument(name = "GetHealth", level = "debug", skip(self))]
  async fn resolve_to_string(
    &self,
    _: GetHealth,
    _: (),
  ) -> anyhow::Result<String> {
    Ok(String::from("{}"))
  }
}

//

impl Resolve<GetVersion> for State {
  #[instrument(name = "GetVersion", level = "debug", skip(self))]
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

impl ResolveToString<GetAccounts> for State {
  #[instrument(name = "GetAccounts", level = "debug", skip(self))]
  async fn resolve_to_string(
    &self,
    _: GetAccounts,
    _: (),
  ) -> anyhow::Result<String> {
    Ok(accounts_response().clone())
  }
}

//

impl ResolveToString<GetSecrets> for State {
  #[instrument(name = "GetSecrets", level = "debug", skip(self))]
  async fn resolve_to_string(
    &self,
    _: GetSecrets,
    _: (),
  ) -> anyhow::Result<String> {
    Ok(secrets_response().clone())
  }
}

impl Resolve<RunCommand> for State {
  #[instrument(name = "RunCommand", skip(self))]
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

impl Resolve<PruneSystem> for State {
  #[instrument(name = "PruneSystem", skip(self))]
  async fn resolve(
    &self,
    PruneSystem {}: PruneSystem,
    _: (),
  ) -> anyhow::Result<Log> {
    Ok(docker::prune_system().await)
  }
}
