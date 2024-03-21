use monitor_client::entities::server::stats::{
  AllSystemStats, BasicSystemStats, CpuUsage, DiskUsage,
  NetworkUsage, SystemComponent, SystemInformation, SystemProcess,
};
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(SystemInformation)]
pub struct GetSystemInformation {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(AllSystemStats)]
pub struct GetAllSystemStats {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(BasicSystemStats)]
pub struct GetBasicSystemStats {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(CpuUsage)]
pub struct GetCpuUsage {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(DiskUsage)]
pub struct GetDiskUsage {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(NetworkUsage)]
pub struct GetNetworkUsage {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<SystemProcess>)]
pub struct GetSystemProcesses {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Request)]
#[response(Vec<SystemComponent>)]
pub struct GetSystemComponents {}
