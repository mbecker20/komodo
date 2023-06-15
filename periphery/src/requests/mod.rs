use anyhow::Context;
use monitor_types::entities::{update::Log, SystemCommand};
use resolver_api::{
    derive::{Request, Resolver},
    Resolve, ResolveToString,
};
use serde::{Deserialize, Serialize};

use crate::{helpers::run_monitor_command, state::State};

mod stats;
pub use stats::*;

mod container;
pub use container::*;

mod git;
pub use git::*;

mod build;
pub use build::*;

mod network;
pub use network::*;

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
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetHealthResponse)]
pub struct GetHealth {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHealthResponse {}

#[async_trait::async_trait]
impl ResolveToString<GetHealth> for State {
    async fn resolve_to_string(&self, _: GetHealth) -> anyhow::Result<String> {
        Ok(String::from("{}"))
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetVersionResponse)]
pub struct GetVersion {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
    pub version: String,
}

#[async_trait::async_trait]
impl Resolve<GetVersion> for State {
    async fn resolve(&self, _: GetVersion) -> anyhow::Result<GetVersionResponse> {
        Ok(GetVersionResponse {
            version: env!("CARGO_PKG_VERSION").to_string(),
        })
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(GetAccountsResponse)]
pub struct GetAccounts {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountsResponse {
    pub docker: Vec<String>,
    pub github: Vec<String>,
}

#[async_trait::async_trait]
impl ResolveToString<GetAccounts> for State {
    async fn resolve_to_string(&self, _: GetAccounts) -> anyhow::Result<String> {
        Ok(self.accounts_response.clone())
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<String>)]
pub struct GetSecrets {}

#[async_trait::async_trait]
impl ResolveToString<GetSecrets> for State {
    async fn resolve_to_string(&self, _: GetSecrets) -> anyhow::Result<String> {
        Ok(self.secrets_response.clone())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct RunCommand {
    pub command: SystemCommand,
}

#[async_trait::async_trait]
impl Resolve<RunCommand> for State {
    async fn resolve(
        &self,
        RunCommand {
            command: SystemCommand { path, command },
        }: RunCommand,
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
