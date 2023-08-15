use derive_variants::EnumVariants;
use mungos::{
    derive::{MungosIndexed, StringObjectId},
    mongodb::bson::{doc, serde_helpers::hex_string_as_object_id},
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{MongoId, I64};

use super::{
    deployment::DockerContainerState,
    server::stats::{StatsState, SystemProcess},
    update::ResourceTarget,
};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, MungosIndexed, StringObjectId)]
#[doc_index(doc! { "target.type": 1 })]
#[sparse_doc_index(doc! { "target.id": 1 })]
pub struct AlertRecord {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    pub id: MongoId,

    #[index]
    pub start_ts: I64,

    #[index]
    pub resolved: bool,

    #[index]
    pub alert_type: AlertVariant,

    pub target: ResourceTarget,
    pub alert: Alert,
    pub resolved_ts: Option<I64>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, EnumVariants, MungosIndexed)]
#[variant_derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    None {},
}

impl Default for Alert {
    fn default() -> Self {
        Alert::None {}
    }
}

#[allow(clippy::derivable_impls)]
impl Default for AlertVariant {
    fn default() -> Self {
        AlertVariant::None
    }
}
