use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use diff::Diff;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::PermissionsMap;

pub mod docker_network;
pub mod docker_image;
pub mod stats;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Diff, Builder)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct Server {
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
