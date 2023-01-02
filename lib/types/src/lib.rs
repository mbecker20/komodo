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
mod group;
mod procedure;
mod server;
mod update;
mod user;

pub use build::*;
pub use config::*;
pub use deployment::*;
pub use group::*;
pub use procedure::*;
pub use server::*;
pub use update::*;
pub use user::*;

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

    // group
    CreateGroup,
    UpdateGroup,
    DeleteGroup,

    // user
    ModifyUserEnabled,
    ModifyUserCreateServerPermissions,
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

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Display, EnumString, PartialEq, Hash, Eq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Timelength {
    #[serde(rename = "1-sec")]
    #[strum(serialize = "1-sec")]
    OneSecond,
    #[serde(rename = "5-sec")]
    #[strum(serialize = "5-sec")]
    FiveSeconds,
    #[serde(rename = "10-sec")]
    #[strum(serialize = "10-sec")]
    TenSeconds,
    #[serde(rename = "30-sec")]
    #[strum(serialize = "30-sec")]
    ThirtySeconds,
    #[serde(rename = "1-min")]
    #[strum(serialize = "1-min")]
    OneMinute,
    #[serde(rename = "2-min")]
    #[strum(serialize = "2-min")]
    TwoMinutes,
    #[serde(rename = "5-min")]
    #[strum(serialize = "5-min")]
    FiveMinutes,
    #[serde(rename = "10-min")]
    #[strum(serialize = "10-min")]
    TenMinutes,
    #[serde(rename = "15-min")]
    #[strum(serialize = "15-min")]
    FifteenMinutes,
    #[serde(rename = "30-min")]
    #[strum(serialize = "30-min")]
    ThirtyMinutes,
    #[serde(rename = "1-hr")]
    #[strum(serialize = "1-hr")]
    OneHour,
    #[serde(rename = "2-hr")]
    #[strum(serialize = "2-hr")]
    TwoHours,
    #[serde(rename = "6-hr")]
    #[strum(serialize = "6-hr")]
    SixHours,
    #[serde(rename = "8-hr")]
    #[strum(serialize = "8-hr")]
    EightHours,
    #[serde(rename = "12-hr")]
    #[strum(serialize = "12-hr")]
    TwelveHours,
    #[serde(rename = "1-day")]
    #[strum(serialize = "1-day")]
    OneDay,
    #[serde(rename = "3-day")]
    #[strum(serialize = "3-day")]
    ThreeDay,
    #[serde(rename = "1-wk")]
    #[strum(serialize = "1-wk")]
    OneWeek,
    #[serde(rename = "2-wk")]
    #[strum(serialize = "2-wk")]
    TwoWeeks,
    #[serde(rename = "30-day")]
    #[strum(serialize = "30-day")]
    ThirtyDays,
}

impl Default for Timelength {
    fn default() -> Timelength {
        Timelength::FiveMinutes
    }
}

pub fn monitor_timestamp() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Millis, false)
}

pub fn unix_from_monitor_ts(ts: &str) -> anyhow::Result<i64> {
    Ok(DateTime::parse_from_rfc3339(ts)
        .context("failed to parse rfc3339 timestamp")?
        .timestamp_millis())
}
