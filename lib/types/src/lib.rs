use diff::Diff;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

pub mod core_api;
pub mod periphery_api;
pub mod entities;

#[typeshare(serialized_as = "number")]
pub type I64 = i64;

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

pub trait HasResponse: Serialize + std::fmt::Debug {
    type Response: DeserializeOwned + std::fmt::Debug;
    fn req_type() -> &'static str;
}

#[macro_export]
macro_rules! impl_has_response {
    ($req:ty, $res:ty) => {
        impl $crate::HasResponse for $req {
            type Response = $res;
            fn req_type() -> &'static str {
                stringify!($req)
            }
        }
    };
}