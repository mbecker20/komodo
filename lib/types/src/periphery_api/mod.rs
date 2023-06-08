use serde::{Deserialize, Serialize};

use crate::SystemCommand;

use self::requests::{GetHealth, GetVersion};

pub mod requests;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "params")]
pub enum PeripheryRequest {
    // GET
    GetHealth(GetHealth),
    GetVersion(GetVersion),
    GetSystemInformation {},
    GetSystemStats {},
    GetAccounts {},
    GetSecrets {},
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

impl Default for PeripheryRequest {
    fn default() -> PeripheryRequest {
        PeripheryRequest::GetHealth(GetHealth {})
    }
}
