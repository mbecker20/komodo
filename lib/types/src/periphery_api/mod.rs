use serde::{Serialize, Deserialize};

use crate::SystemCommand;

use self::requests::{GetVersion, GetHealth};

pub mod requests;

#[derive(Serialize, Deserialize, Debug, Clone)]
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