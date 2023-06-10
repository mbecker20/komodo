use serde::{Deserialize, Serialize};

use monitor_types::entities::SystemCommand;

use self::requests::{
    GetAllSystemStats, GetBasicSystemStats, GetCpuUsage, GetDiskUsage, GetHealth, GetNetworkUsage,
    GetSystemComponents, GetSystemInformation, GetSystemProcesses, GetVersion, GetAccounts, GetSecrets,
};

pub mod requests;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "params")]
pub enum PeripheryRequest {
    // GET
    GetHealth(GetHealth),
    GetVersion(GetVersion),
    GetSystemInformation(GetSystemInformation),
    GetAllSystemStats(GetAllSystemStats),
    GetBasicSystemStats(GetBasicSystemStats),
    GetCpuUsage(GetCpuUsage),
    GetDiskUsage(GetDiskUsage),
    GetNetworkUsage(GetNetworkUsage),
    GetSystemProcesses(GetSystemProcesses),
    GetSystemComponents(GetSystemComponents),
    GetAccounts(GetAccounts),
    GetSecrets(GetSecrets),
    GetContainerList {},
    GetContainerLog {},
    GetContainerStats {},
    GetContainerStatsList {},
    GetNetworkList {},

    // ACTIONS
    RunCommand(SystemCommand),
    CloneRepo {},
    PullRepo {},
    DeleteRepo {},
    Build {},
    Deploy {},
    StartContainer {},
    StopContainer {},
    RemoveContainer {},
    RenameContainer {},
    PruneContainers {},
}

