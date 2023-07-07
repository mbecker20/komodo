use derive_builder::Builder;
use mungos::{MungosIndexed, mongodb::bson::serde_helpers::hex_string_as_object_id};
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{i64_is_zero, I64, MongoId};

use super::{PermissionsMap, SystemCommand};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, MungosIndexed)]
pub struct Repo {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    #[builder(setter(skip))]
    pub id: MongoId,

    #[unique_index]
    pub name: String,

    #[serde(default)]
    #[builder(default)]
    pub description: String,

    #[serde(default)]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[serde(default, skip_serializing_if = "i64_is_zero")]
    #[builder(setter(skip))]
    pub created_at: I64,

    #[serde(default)]
    #[builder(setter(skip))]
    pub updated_at: I64,

    #[serde(default)]
    #[builder(setter(skip))]
    pub last_pulled_at: I64,

    #[serde(default)]
    #[builder(default)]
    pub tags: Vec<String>,

    pub config: RepoConfig,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial, MungosIndexed)]
#[partial_derive(Serialize, Deserialize, Debug, Clone)]
#[skip_serializing_none]
pub struct RepoConfig {
    #[index]
    pub server_id: String,

    pub repo: String,

    #[serde(default = "default_branch")]
    #[builder(default = "default_branch()")]
    pub branch: String,

    #[serde(default)]
    #[builder(default)]
    pub github_account: String,

    #[serde(default)]
    #[builder(default)]
    pub on_clone: SystemCommand,

    #[serde(default)]
    #[builder(default)]
    pub on_pull: SystemCommand,
}

fn default_branch() -> String {
    String::from("main")
}

impl From<PartialRepoConfig> for RepoConfig {
    fn from(value: PartialRepoConfig) -> RepoConfig {
        RepoConfig {
            server_id: value.server_id.unwrap_or_default(),
            repo: value.repo.unwrap_or_default(),
            branch: value.branch.unwrap_or(default_branch()),
            github_account: value.github_account.unwrap_or_default(),
            on_clone: value.on_clone.unwrap_or_default(),
            on_pull: value.on_pull.unwrap_or_default(),
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RepoActionState {
    pub cloning: bool,
    pub pulling: bool,
    pub updating: bool,
    pub deleting: bool,
}
