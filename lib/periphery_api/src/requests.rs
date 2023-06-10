use resolver_api::derive::Response;
use serde::{Deserialize, Serialize};

use monitor_types::entities::server::{
    AllSystemStats, BasicSystemStats, CpuUsage, DiskUsage, NetworkUsage, SystemComponent,
    SystemInformation, SystemProcess,
};

//

#[derive(Serialize, Deserialize, Debug, Clone, Response)]
#[response(GetHealthResponse)]
pub struct GetHealth {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHealthResponse {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Response)]
#[response(GetVersionResponse)]
pub struct GetVersion {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
    pub version: String,
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Response)]
#[response(SystemInformation)]
pub struct GetSystemInformation {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Response)]
#[response(AllSystemStats)]
pub struct GetAllSystemStats {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Response)]
#[response(BasicSystemStats)]
pub struct GetBasicSystemStats {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Response)]
#[response(CpuUsage)]
pub struct GetCpuUsage {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Response)]
#[response(DiskUsage)]
pub struct GetDiskUsage {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Response)]
#[response(NetworkUsage)]
pub struct GetNetworkUsage {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Response)]
#[response(Vec<SystemProcess>)]
pub struct GetSystemProcesses {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Response)]
#[response(Vec<SystemComponent>)]
pub struct GetSystemComponents {}

//

#[derive(Serialize, Deserialize, Debug, Clone, Response)]
#[response(GetAccountsResponse)]
pub struct GetAccounts {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountsResponse {
    pub docker: Vec<String>,
    pub github: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Response)]
#[response(Vec<String>)]
pub struct GetSecrets {}
