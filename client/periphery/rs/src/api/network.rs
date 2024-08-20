use monitor_client::entities::{
  docker::network::{Network, NetworkListItem},
  update::Log,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<NetworkListItem>)]
pub struct GetNetworkList {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Network)]
pub struct InspectNetwork {
  pub name: String,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct CreateNetwork {
  pub name: String,
  pub driver: Option<String>,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct DeleteNetwork {
  pub name: String,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneNetworks {}
