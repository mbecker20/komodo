use bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

use crate::{monitor_timestamp, Build, Deployment, Group, Operation, Procedure, Server, Version};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Update {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    pub id: String,
    pub target: UpdateTarget,
    pub operation: Operation,
    pub logs: Vec<Log>,
    pub start_ts: String,
    pub end_ts: Option<String>,
    pub status: UpdateStatus,
    pub success: bool,
    pub operator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<Version>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Log {
    pub stage: String,
    pub command: String,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub start_ts: String,
    pub end_ts: String,
}

impl Log {
    pub fn simple(stage: &str, msg: String) -> Log {
        let ts = monitor_timestamp();
        Log {
            stage: stage.to_string(),
            stdout: msg,
            success: true,
            start_ts: ts.clone(),
            end_ts: ts,
            ..Default::default()
        }
    }

    pub fn error(stage: &str, msg: String) -> Log {
        let ts = monitor_timestamp();
        Log {
            stage: stage.to_string(),
            stderr: msg,
            start_ts: ts.clone(),
            end_ts: ts,
            success: false,
            ..Default::default()
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "id")]
pub enum UpdateTarget {
    System,
    Build(String),
    Deployment(String),
    Server(String),
    Procedure(String),
    Group(String),
}

impl Default for UpdateTarget {
    fn default() -> Self {
        UpdateTarget::System
    }
}

impl From<&Build> for UpdateTarget {
    fn from(build: &Build) -> Self {
        Self::Build(build.id.clone())
    }
}

impl From<&Build> for Option<UpdateTarget> {
    fn from(build: &Build) -> Self {
        Some(UpdateTarget::Build(build.id.clone()))
    }
}

impl From<&Deployment> for UpdateTarget {
    fn from(deployment: &Deployment) -> Self {
        Self::Deployment(deployment.id.clone())
    }
}

impl From<&Deployment> for Option<UpdateTarget> {
    fn from(deployment: &Deployment) -> Self {
        Some(UpdateTarget::Deployment(deployment.id.clone()))
    }
}

impl From<&Server> for UpdateTarget {
    fn from(server: &Server) -> Self {
        Self::Server(server.id.clone())
    }
}

impl From<&Server> for Option<UpdateTarget> {
    fn from(server: &Server) -> Self {
        Some(UpdateTarget::Server(server.id.clone()))
    }
}

impl From<&Procedure> for UpdateTarget {
    fn from(procedure: &Procedure) -> Self {
        Self::Procedure(procedure.id.clone())
    }
}

impl From<&Procedure> for Option<UpdateTarget> {
    fn from(procedure: &Procedure) -> Self {
        Some(UpdateTarget::Procedure(procedure.id.clone()))
    }
}

impl From<&Group> for UpdateTarget {
    fn from(group: &Group) -> Self {
        Self::Group(group.id.clone())
    }
}

impl From<&Group> for Option<UpdateTarget> {
    fn from(group: &Group) -> Self {
        Some(UpdateTarget::Group(group.id.clone()))
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum UpdateStatus {
    Queued,
    InProgress,
    Complete,
}

impl Default for UpdateStatus {
    fn default() -> Self {
        UpdateStatus::Complete
    }
}
