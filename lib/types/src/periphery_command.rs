use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use diff::Diff;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{diff::command_diff_no_change, Command, PermissionsMap};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Diff, Builder)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct PeripheryCommand {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    #[builder(setter(skip))]
    pub id: String,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub name: String, // must be formatted to be compat with docker

    #[serde(default)]
    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub description: String,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub server_id: String,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing)]))]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[serde(default)]
    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "command_diff_no_change")]))]
    pub command: Command,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[diff(attr(#[serde(skip)]))]
    #[builder(setter(skip))]
    pub created_at: String,
    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    #[builder(setter(skip))]
    pub updated_at: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommandActionState {
    pub running: bool,
}
