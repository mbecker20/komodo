use serde::{Deserialize, Serialize};

use monitor_types::{
    entities::server::{
        AllSystemStats, BasicSystemStats, CpuUsage, DiskUsage, NetworkUsage, SystemComponent,
        SystemInformation, SystemProcess,
    },
    impl_has_response,
};

//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHealth {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHealthResponse {}

impl_has_response!(GetHealth, GetHealthResponse);

//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersion {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
    pub version: String,
}

impl_has_response!(GetVersion, GetVersionResponse);

//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSystemInformation {}

impl_has_response!(GetSystemInformation, SystemInformation);

//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAllSystemStats {}

impl_has_response!(GetAllSystemStats, AllSystemStats);

//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBasicSystemStats {}

impl_has_response!(GetBasicSystemStats, BasicSystemStats);

//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCpuUsage {}

impl_has_response!(GetCpuUsage, CpuUsage);

//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDiskUsage {}

impl_has_response!(GetDiskUsage, DiskUsage);

//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetNetworkUsage {}

impl_has_response!(GetNetworkUsage, NetworkUsage);

//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSystemProcesses {}

impl_has_response!(GetSystemProcesses, Vec<SystemProcess>);

//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSystemComponents {}

impl_has_response!(GetSystemComponents, Vec<SystemComponent>);

//

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccounts {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAccountsResponse {
    pub docker: Vec<String>,
    pub github: Vec<String>,
}

impl_has_response!(GetAccounts, GetAccountsResponse);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSecrets {}

impl_has_response!(GetSecrets, Vec<String>);
