use monitor_client::entities::{
  docker::network::Network, update::Log,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

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
  /// Id or name
  pub name: String,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Log)]
pub struct PruneNetworks {}
