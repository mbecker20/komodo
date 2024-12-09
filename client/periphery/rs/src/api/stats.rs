use komodo_client::entities::stats::{
  SystemInformation, SystemProcess, SystemStats,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(SystemInformation)]
pub struct GetSystemInformation {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(SystemStats)]
pub struct GetSystemStats {}

//

// #[derive(Serialize, Deserialize, Debug, Clone, Request)]
// #[response(NetworkStatsByInterface)]
// pub struct GetNetworkStatsByInterface {}

// //

// #[derive(Serialize, Deserialize, Debug, Clone, Request)]
// #[response(TotalNetworkStats)]
// pub struct GetTotalNetworkStats {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<SystemProcess>)]
pub struct GetSystemProcesses {}