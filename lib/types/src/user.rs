use bson::serde_helpers::hex_string_as_object_id;
use diff::Diff;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::diff::*;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Diff)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct User {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub id: String,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub username: String,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub enabled: bool,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub admin: bool,

    #[diff(attr(#[serde(skip_serializing_if = "Option::is_none")]))]
    pub create_server_permissions: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub avatar: Option<String>,

    // used with auth
    #[serde(default)]
    #[diff(attr(#[serde(skip_serializing_if = "vec_diff_no_change")]))]
    pub secrets: Vec<ApiSecret>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub password: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub github_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub google_id: Option<String>,

    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    pub created_at: String,
    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    pub updated_at: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Diff)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct ApiSecret {
    pub name: String,
    pub hash: String,
    pub created_at: String,
    pub expires: Option<String>,
}
