use monitor_types::requests::api::{CreateLoginSecret, DeleteLoginSecret, GetPeripheryVersion, GetServer, ListServers, CreateServer, DeleteServer, UpdateServer};
use resolver_api::{derive::Resolver, Resolve};
use serde::{Deserialize, Serialize};

use crate::{auth::RequestUser, state::State};

mod secret;
mod server;

#[derive(Serialize, Deserialize, Debug, Clone, Resolver)]
#[resolver_target(State)]
#[resolver_args(RequestUser)]
#[serde(tag = "type", content = "params")]
#[allow(clippy::enum_variant_names, clippy::large_enum_variant)]
pub enum ApiRequest {
    CreateLoginSecret(CreateLoginSecret),
    DeleteLoginSecret(DeleteLoginSecret),
    // SERVER
    GetPeripheryVersion(GetPeripheryVersion),
    GetServer(GetServer),
    ListServers(ListServers),
    CreateServer(CreateServer),
    DeleteServer(DeleteServer),
    UpdateServer(UpdateServer),
}
