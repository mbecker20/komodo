use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::{PermissionsMap, SystemCommand};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder)]
pub struct Repo {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    #[builder(setter(skip))]
    pub id: String,

    pub name: String,

    #[serde(default)]
    #[builder(default)]
    pub description: String,

    #[serde(default)]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    #[builder(setter(skip))]
    pub created_at: String,

    #[serde(default)]
    #[builder(setter(skip))]
    pub updated_at: String,

    #[serde(default)]
    pub tags: Vec<String>,

    pub config: RepoConfig,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone)]
#[skip_serializing_none]
pub struct RepoConfig {
    pub repo: String,

    #[builder(default = "default_branch()")]
    pub branch: String,

    #[builder(default)]
    pub github_account: String,

    #[builder(default)]
    pub on_clone: SystemCommand,

    #[builder(default)]
    pub on_pull: SystemCommand,
}

fn default_branch() -> String {
    String::from("main")
}
