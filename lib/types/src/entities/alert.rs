use derive_variants::EnumVariants;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::{
    deployment::DockerContainerState,
    server::stats::{StatsState, SystemProcess},
};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, EnumVariants)]
#[variant_derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(tag = "type", content = "data")]
pub enum Alert {
    ServerUnreachable {
        id: String,
        name: String,
        region: Option<String>,
    },
    ServerCpu {
        id: String,
        name: String,
        region: Option<String>,
        state: StatsState,
        percentage: f64,
        top_procs: Vec<SystemProcess>,
    },
    ServerMem {
        id: String,
        name: String,
        region: Option<String>,
        state: StatsState,
        used_gb: f64,
        total_gb: f64,
        top_procs: Vec<SystemProcess>,
    },
    ServerDisk {
        id: String,
        name: String,
        region: Option<String>,
        state: StatsState,
        path: String,
        used_gb: f64,
        total_gb: f64,
    },
    ServerTemp {
        id: String,
        name: String,
        region: Option<String>,
        state: StatsState,
        temp: f64,
        max: f64,
    },
    ContainerStateChange {
        id: String,
        name: String,
        server: String, // server name
        from: DockerContainerState,
        to: DockerContainerState,
    },
}
