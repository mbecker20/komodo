use async_timing_util::unix_timestamp_ms;
use bson::serde_helpers::hex_string_as_object_id;
use mungos::MungosIndexed;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

use crate::{entities::Operation, monitor_timestamp, I64};

use super::Version;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, MungosIndexed)]
pub struct Update {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    pub id: String,
    pub target: ResourceTarget,
    pub operation: Operation,
    pub logs: Vec<Log>,
    pub start_ts: I64,
    pub end_ts: Option<I64>,
    pub status: UpdateStatus,
    pub success: bool,
    pub operator: String,
    pub version: Version,
}

impl Update {
    pub fn finalize(&mut self) {
        self.success = all_logs_success(&self.logs);
        self.end_ts = Some(monitor_timestamp());
        self.status = UpdateStatus::Complete;
    }
}

fn all_logs_success(logs: &Vec<Log>) -> bool {
    for log in logs {
        if !log.success {
            return false;
        }
    }
    true
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Log {
    pub stage: String,
    pub command: String,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub start_ts: I64,
    pub end_ts: I64,
}

impl Log {
    pub fn simple(stage: &str, msg: String) -> Log {
        let ts = unix_timestamp_ms() as i64;
        Log {
            stage: stage.to_string(),
            stdout: msg,
            success: true,
            start_ts: ts,
            end_ts: ts,
            ..Default::default()
        }
    }

    pub fn error(stage: &str, msg: String) -> Log {
        let ts = unix_timestamp_ms() as i64;
        Log {
            stage: stage.to_string(),
            stderr: msg,
            start_ts: ts,
            end_ts: ts,
            success: false,
            ..Default::default()
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, MungosIndexed)]
#[serde(tag = "type", content = "id")]
pub enum ResourceTarget {
    #[default]
    System,
    Build(String),
    Builder(String),
    Deployment(String),
    Server(String),
    Repo(String),
    // Procedure(String),
    // Group(String),
    // Command(String),
}

// impl From<&Build> for UpdateTarget {
//     fn from(build: &Build) -> Self {
//         Self::Build(build.id.clone())
//     }
// }

// impl From<&Build> for Option<UpdateTarget> {
//     fn from(build: &Build) -> Self {
//         Some(UpdateTarget::Build(build.id.clone()))
//     }
// }

// impl From<&Deployment> for UpdateTarget {
//     fn from(deployment: &Deployment) -> Self {
//         Self::Deployment(deployment.id.clone())
//     }
// }

// impl From<&Deployment> for Option<UpdateTarget> {
//     fn from(deployment: &Deployment) -> Self {
//         Some(UpdateTarget::Deployment(deployment.id.clone()))
//     }
// }

// impl From<&Server> for UpdateTarget {
//     fn from(server: &Server) -> Self {
//         Self::Server(server.id.clone())
//     }
// }

// impl From<&Server> for Option<UpdateTarget> {
//     fn from(server: &Server) -> Self {
//         Some(UpdateTarget::Server(server.id.clone()))
//     }
// }

// impl From<&Procedure> for UpdateTarget {
//     fn from(procedure: &Procedure) -> Self {
//         Self::Procedure(procedure.id.clone())
//     }
// }

// impl From<&Procedure> for Option<UpdateTarget> {
//     fn from(procedure: &Procedure) -> Self {
//         Some(UpdateTarget::Procedure(procedure.id.clone()))
//     }
// }

// impl From<&Group> for UpdateTarget {
//     fn from(group: &Group) -> Self {
//         Self::Group(group.id.clone())
//     }
// }

// impl From<&Group> for Option<UpdateTarget> {
//     fn from(group: &Group) -> Self {
//         Some(UpdateTarget::Group(group.id.clone()))
//     }
// }

#[typeshare]
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Display,
    EnumString,
    PartialEq,
    Hash,
    Eq,
    Clone,
    Copy,
    Default,
    MungosIndexed,
)]
pub enum UpdateStatus {
    Queued,
    InProgress,
    #[default]
    Complete,
}
