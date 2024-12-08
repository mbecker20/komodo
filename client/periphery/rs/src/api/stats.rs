use komodo_client::entities::stats::{
  SystemInformation, SystemProcess, SystemStats,
};
use resolver_api::Resolve;
use serde::{Deserialize, Serialize};

//

#[derive(Serialize, Deserialize, Debug, Clone, Resolve)]
#[response(SystemInformation)]
pub struct GetSystemInformation {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Resolve)]
#[response(SystemStats)]
pub struct GetSystemStats {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Resolve)]
#[response(Vec<SystemProcess>)]
pub struct GetSystemProcesses {}

//
