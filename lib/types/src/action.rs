use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use diff::Diff;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{diff::*, Command, PermissionsMap};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Diff, Builder)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct Action {
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
    pub name: String,

    #[serde(default)]
    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub description: String,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub path: String,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub command: String,

    // run action on all servers in this array
    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "vec_diff_no_change")]))]
    pub server_ids: Vec<String>,

    // run action on all servers in the group
    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "vec_diff_no_change")]))]
    pub group_ids: Vec<String>,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub run_on_all: bool,

    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing)]))]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[diff(attr(#[serde(skip)]))]
    #[builder(setter(skip))]
    pub created_at: String,
    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    #[builder(setter(skip))]
    pub updated_at: String,
}

impl From<Action> for Command {
    fn from(value: Action) -> Command {
        Command {
            path: value.path,
            command: value.command,
        }
    }
}
