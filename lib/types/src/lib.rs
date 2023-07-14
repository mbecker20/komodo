use async_timing_util::unix_timestamp_ms;
use entities::{
    build::{Build, BuildConfig},
    update::Log,
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

pub mod busy;
pub mod entities;
pub mod permissioned;
pub mod requests;

#[typeshare(serialized_as = "number")]
pub type I64 = i64;
#[typeshare(serialized_as = "number")]
pub type U64 = u64;
#[typeshare(serialized_as = "any")]
pub type MongoDocument = mungos::mongodb::bson::Document;
#[typeshare(serialized_as = "MongoIdObj")]
pub type MongoId = String;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MongoIdObj {
    #[serde(rename = "$oid")]
    pub oid: String,
}

pub fn all_logs_success(logs: &[Log]) -> bool {
    for log in logs {
        if !log.success {
            return false;
        }
    }
    true
}

pub fn optional_string(string: &str) -> Option<String> {
    if string.is_empty() {
        None
    } else {
        Some(string.to_string())
    }
}

pub fn get_image_name(
    Build {
        name,
        config:
            BuildConfig {
                docker_organization,
                docker_account,
                ..
            },
        ..
    }: &Build,
) -> String {
    let name = to_monitor_name(name);
    if !docker_organization.is_empty() {
        format!("{docker_organization}/{name}")
    } else if !docker_account.is_empty() {
        format!("{docker_account}/{name}")
    } else {
        name
    }
}

pub fn to_monitor_name(name: &str) -> String {
    name.to_lowercase().replace(' ', "_")
}

pub fn monitor_timestamp() -> i64 {
    unix_timestamp_ms() as i64
}
