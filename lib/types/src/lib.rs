use std::collections::HashMap;

use ::diff::Diff;
use anyhow::Context;
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

pub use bollard::service::{ImageSummary, Network};

pub mod traits;

mod build;
mod config;
mod deployment;
mod diff;
mod procedure;
mod server;
mod update;
mod user;

pub use build::*;
pub use config::*;
pub use deployment::*;
pub use procedure::*;
pub use server::*;
pub use update::*;
pub use user::*;

pub const PERIPHERY_BUILDER_BUSY: &str = "builder is busy";

#[typeshare]
pub type PermissionsMap = HashMap<String, PermissionLevel>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Diff)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub struct Command {
    pub path: String,
    pub command: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Diff)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub struct EnvironmentVar {
    pub variable: String,
    pub value: String,
}

#[typeshare]
#[derive(Deserialize, Debug)]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AccountType {
    Github,
    Docker,
}

#[typeshare]
#[derive(
    Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy, Diff,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub enum Operation {
    // do nothing
    None,

    // server
    CreateServer,
    UpdateServer,
    DeleteServer,
    PruneImagesServer,
    PruneContainersServer,
    PruneNetworksServer,

    // build
    CreateBuild,
    UpdateBuild,
    DeleteBuild,
    BuildBuild,
    RecloneBuild,

    // deployment
    CreateDeployment,
    UpdateDeployment,
    DeleteDeployment,
    DeployContainer,
    StopContainer,
    StartContainer,
    RemoveContainer,
    PullDeployment,
    RecloneDeployment,

    // procedure
    CreateProcedure,
    UpdateProcedure,
    DeleteProcedure,

    // user
    ModifyUserEnabled,
    ModifyUserPermissions,
}

impl Default for Operation {
    fn default() -> Self {
        Operation::None
    }
}

#[typeshare]
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Display,
    EnumString,
    Hash,
    Clone,
    Copy,
    Diff,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub enum PermissionLevel {
    None,
    Read,
    Execute,
    Update,
}

impl Default for PermissionLevel {
    fn default() -> Self {
        PermissionLevel::None
    }
}

impl Default for &PermissionLevel {
    fn default() -> Self {
        &PermissionLevel::None
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PermissionsTarget {
    Server,
    Deployment,
    Build,
    Procedure,
}

pub fn monitor_timestamp() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, false)
}

pub fn unix_from_monitor_ts(ts: &str) -> anyhow::Result<i64> {
    Ok(DateTime::parse_from_rfc3339(ts)
        .context("failed to parse rfc3339 timestamp")?
        .timestamp_millis())
}
