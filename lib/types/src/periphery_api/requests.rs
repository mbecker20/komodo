use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{
    entities::server::{BasicSystemStats, DiskUsage, SystemInformation, SystemProcess, NetworkUsage, AllSystemStats, SystemComponent, CpuUsage},
    impl_has_response,
};

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHealth {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetHealthResponse {}

impl_has_response!(GetHealth, GetHealthResponse);

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersion {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVersionResponse {
    pub version: String,
}

impl_has_response!(GetVersion, GetVersionResponse);

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSystemInformation {}

impl_has_response!(GetSystemInformation, SystemInformation);

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAllSystemStats {}

impl_has_response!(GetAllSystemStats, AllSystemStats);

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetBasicSystemStats {}

impl_has_response!(GetBasicSystemStats, BasicSystemStats);

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetCpuUsage {}

impl_has_response!(GetCpuUsage, CpuUsage);

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetDiskUsage {}

impl_has_response!(GetDiskUsage, DiskUsage);

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetNetworkUsage {}

impl_has_response!(GetNetworkUsage, NetworkUsage);

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSystemProcesses {}

impl_has_response!(GetSystemProcesses, Vec<SystemProcess>);

//

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetSystemComponents {}

impl_has_response!(GetSystemComponents, Vec<SystemComponent>);