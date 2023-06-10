use anyhow::{Context, anyhow};
use diff::Diff;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

pub mod server;
pub mod update;
pub mod deployment;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq, Diff)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub struct SystemCommand {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub command: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Diff)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl TryFrom<&str> for Version {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let vals = value
            .split('.')
            .map(|v| anyhow::Ok(v.parse().context("failed at parsing value into i32")?))
            .collect::<anyhow::Result<Vec<i32>>>()?;
        let version = Version {
            major: *vals
                .first()
                .ok_or(anyhow!("must include at least major version"))?,
            minor: *vals.get(1).unwrap_or(&0),
            patch: *vals.get(2).unwrap_or(&0),
        };
        Ok(version)
    }
}

impl Version {
    pub fn increment(&mut self) {
        self.patch += 1;
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Diff)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub struct EnvironmentVar {
    pub variable: String,
    pub value: String,
}

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
    Diff,
    Default,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
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
    #[serde(rename = "15-sec")]
    #[strum(serialize = "15-sec")]
    FifteenSeconds,
    #[serde(rename = "30-sec")]
    #[strum(serialize = "30-sec")]
    ThirtySeconds,
    #[default]
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
    Default,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub enum PermissionLevel {
    #[default]
    None,
    Read,
    Execute,
    Update,
}

#[typeshare]
#[derive(
    Serialize,
    Deserialize,
    Debug,
    Default,
    Display,
    EnumString,
    PartialEq,
    Hash,
    Eq,
    Clone,
    Copy,
    Diff,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub enum Operation {
    // do nothing
    #[default]
    None,

    // server
    CreateServer,
    UpdateServer,
    DeleteServer,
    PruneImagesServer,
    PruneContainersServer,
    PruneNetworksServer,
    RenameServer,

    // build
    CreateBuild,
    UpdateBuild,
    DeleteBuild,
    BuildBuild,

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
    RenameDeployment,

    // procedure
    CreateProcedure,
    UpdateProcedure,
    DeleteProcedure,

    // command
    CreateCommand,
    UpdateCommand,
    DeleteCommand,
    RunCommand,

    // group
    CreateGroup,
    UpdateGroup,
    DeleteGroup,

    // user
    ModifyUserEnabled,
    ModifyUserCreateServerPermissions,
    ModifyUserCreateBuildPermissions,
    ModifyUserPermissions,

    // github webhook automation
    AutoBuild,
    AutoPull,
}
