use anyhow::Context;
use command::run_monitor_command;
use monitor_client::{
  api::read::ListGitProviders,
  entities::{update::Log, SystemCommand},
};
use periphery_client::api::{
  build::*, compose::*, container::*, git::*, network::*, stats::*,
  GetHealth, GetVersion, GetVersionResponse, ListDockerRegistries,
  ListSecrets, PruneSystem, RunCommand,
};
use resolver_api::{derive::Resolver, Resolve, ResolveToString};
use serde::{Deserialize, Serialize};

use crate::{
  config::{
    docker_registries_response, git_providers_response,
    secrets_response,
  },
  State,
};

mod build;
mod compose;
mod container;
mod deploy;
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
  ListGitProviders(ListGitProviders),
  #[to_string_resolver]
  ListDockerRegistries(ListDockerRegistries),
  #[to_string_resolver]
  ListSecrets(ListSecrets),

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

  // Repo
  CloneRepo(CloneRepo),
  PullRepo(PullRepo),
  DeleteRepo(DeleteRepo),

  // Build
  Build(Build),
  PruneImages(PruneImages),

  // Container
  Deploy(Deploy),
  StartContainer(StartContainer),
  RestartContainer(RestartContainer),
  PauseContainer(PauseContainer),
  StopContainer(StopContainer),
  RemoveContainer(RemoveContainer),
  RenameContainer(RenameContainer),
  PruneContainers(PruneContainers),

  // Compose
  ComposeUp(ComposeUp),
  ComposeStart(ComposeStart),
  ComposeRestart(ComposeRestart),
  ComposePause(ComposePause),
  ComposeStop(ComposeStop),
  ComposeDown(ComposeDown),

  // Compose Service
  ComposeServiceUp(ComposeServiceUp),
  ComposeServiceStart(ComposeServiceStart),
  ComposeServiceRestart(ComposeServiceRestart),
  ComposeServicePause(ComposeServicePause),
  ComposeServiceStop(ComposeServiceStop),
  ComposeServiceDown(ComposeServiceDown),

  // Networks
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

impl ResolveToString<ListGitProviders> for State {
  #[instrument(
    name = "ListGitProviders",
    level = "debug",
    skip(self)
  )]
  async fn resolve_to_string(
    &self,
    _: ListGitProviders,
    _: (),
  ) -> anyhow::Result<String> {
    Ok(git_providers_response().clone())
  }
}

impl ResolveToString<ListDockerRegistries> for State {
  #[instrument(
    name = "ListDockerRegistries",
    level = "debug",
    skip(self)
  )]
  async fn resolve_to_string(
    &self,
    _: ListDockerRegistries,
    _: (),
  ) -> anyhow::Result<String> {
    Ok(docker_registries_response().clone())
  }
}

//

impl ResolveToString<ListSecrets> for State {
  #[instrument(name = "ListSecrets", level = "debug", skip(self))]
  async fn resolve_to_string(
    &self,
    _: ListSecrets,
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
    let command = String::from("docker system prune -a -f");
    Ok(run_monitor_command("prune system", command).await)
  }
}
