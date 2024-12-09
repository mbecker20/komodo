use anyhow::Context;
use command::run_komodo_command;
use derive_variants::EnumVariants;
use futures::TryFutureExt;
use komodo_client::entities::{update::Log, SystemCommand};
use periphery_client::api::{
  build::*, compose::*, container::*, git::*, image::*, network::*,
  stats::*, volume::*, GetDockerLists, GetDockerListsResponse,
  GetHealth, GetVersion, GetVersionResponse, ListDockerRegistries,
  ListGitProviders, ListSecrets, PruneSystem, RunCommand,
};
use resolver_api::{derive::Resolver, Resolve, ResolveToString};
use serde::{Deserialize, Serialize};

use crate::{
  config::{
    docker_registries_response, git_providers_response,
    secrets_response,
  },
  docker::docker_client,
  State,
};

mod build;
mod compose;
mod container;
mod deploy;
mod git;
mod image;
mod network;
mod stats;
mod volume;

#[derive(
  Serialize, Deserialize, Debug, Clone, Resolver, EnumVariants,
)]
#[variant_derive(Debug)]
#[serde(tag = "type", content = "params")]
#[resolver_target(State)]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
pub enum PeripheryRequest {
  GetVersion(GetVersion),
  #[to_string_resolver]
  GetHealth(GetHealth),

  // Config (Read)
  #[to_string_resolver]
  ListGitProviders(ListGitProviders),
  #[to_string_resolver]
  ListDockerRegistries(ListDockerRegistries),
  #[to_string_resolver]
  ListSecrets(ListSecrets),

  // Stats / Info (Read)
  #[to_string_resolver]
  GetSystemInformation(GetSystemInformation),
  #[to_string_resolver]
  GetSystemStats(GetSystemStats),
  #[to_string_resolver]
  GetSystemProcesses(GetSystemProcesses),
  GetLatestCommit(GetLatestCommit),
  // Networks Stats (Read)
  // #[to_string_resolver]
  // GetNetworkStatsByInterface(GetNetworkStatsByInterface),
  // #[to_string_resolver]
  // GetTotalNetworkStats(GetTotalNetworkStats),

  // Generic shell execution
  RunCommand(RunCommand),

  // Repo (Write)
  CloneRepo(CloneRepo),
  PullRepo(PullRepo),
  RenameRepo(RenameRepo),
  DeleteRepo(DeleteRepo),

  // Build
  Build(Build),
  PruneBuilders(PruneBuilders),
  PruneBuildx(PruneBuildx),

  // Compose (Read)
  GetComposeContentsOnHost(GetComposeContentsOnHost),
  GetComposeServiceLog(GetComposeServiceLog),
  GetComposeServiceLogSearch(GetComposeServiceLogSearch),

  // Compose (Write)
  WriteComposeContentsToHost(WriteComposeContentsToHost),
  WriteCommitComposeContents(WriteCommitComposeContents),
  ComposePull(ComposePull),
  ComposeUp(ComposeUp),
  ComposeExecution(ComposeExecution),

  // Container (Read)
  InspectContainer(InspectContainer),
  GetContainerLog(GetContainerLog),
  GetContainerLogSearch(GetContainerLogSearch),
  GetContainerStats(GetContainerStats),
  GetContainerStatsList(GetContainerStatsList),

  // Container (Write)
  Deploy(Deploy),
  StartContainer(StartContainer),
  RestartContainer(RestartContainer),
  PauseContainer(PauseContainer),
  UnpauseContainer(UnpauseContainer),
  StopContainer(StopContainer),
  StartAllContainers(StartAllContainers),
  RestartAllContainers(RestartAllContainers),
  PauseAllContainers(PauseAllContainers),
  UnpauseAllContainers(UnpauseAllContainers),
  StopAllContainers(StopAllContainers),
  RemoveContainer(RemoveContainer),
  RenameContainer(RenameContainer),
  PruneContainers(PruneContainers),

  // Networks (Read)
  InspectNetwork(InspectNetwork),

  // Networks (Write)
  CreateNetwork(CreateNetwork),
  DeleteNetwork(DeleteNetwork),
  PruneNetworks(PruneNetworks),

  // Image (Read)
  InspectImage(InspectImage),
  ImageHistory(ImageHistory),

  // Image (Write)
  PullImage(PullImage),
  DeleteImage(DeleteImage),
  PruneImages(PruneImages),

  // Volume (Read)
  InspectVolume(InspectVolume),

  // Volume (Write)
  DeleteVolume(DeleteVolume),
  PruneVolumes(PruneVolumes),

  // All in one (Read)
  GetDockerLists(GetDockerLists),

  // All in one (Write)
  PruneSystem(PruneSystem),
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

impl Resolve<GetDockerLists> for State {
  #[instrument(name = "GetDockerLists", level = "debug", skip(self))]
  async fn resolve(
    &self,
    GetDockerLists {}: GetDockerLists,
    _: (),
  ) -> anyhow::Result<GetDockerListsResponse> {
    let docker = docker_client();
    let containers =
      docker.list_containers().await.map_err(Into::into);
    // Should still try to retrieve other docker lists, but "in_use" will be false for images, networks, volumes
    let _containers = match &containers {
      Ok(containers) => containers.as_slice(),
      Err(_) => &[],
    };
    let (networks, images, volumes, projects) = tokio::join!(
      docker.list_networks(_containers).map_err(Into::into),
      docker.list_images(_containers).map_err(Into::into),
      docker.list_volumes(_containers).map_err(Into::into),
      self.resolve(ListComposeProjects {}, ()).map_err(Into::into)
    );
    Ok(GetDockerListsResponse {
      containers,
      networks,
      images,
      volumes,
      projects,
    })
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
      run_komodo_command("run command", None, command, false).await
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
    let command = String::from("docker system prune -a -f --volumes");
    Ok(run_komodo_command("prune system", None, command, false).await)
  }
}
