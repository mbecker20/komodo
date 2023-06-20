use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use mungos::MungosIndexed;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{I64, i64_is_zero};

use super::{EnvironmentVar, PermissionsMap, SystemCommand, Version};

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, MungosIndexed)]
pub struct Build {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    #[builder(setter(skip))]
    pub id: String,

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

    pub config: BuildConfig,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial, MungosIndexed)]
#[partial_derive(Serialize, Deserialize, Debug, Clone)]
#[skip_serializing_none]
pub struct BuildConfig {
    #[index]
    #[serde(default)]
    #[builder(default)]
    pub server_id: String,

    #[serde(default)]
    #[builder(default)]
    pub skip_secret_interp: bool,

    #[serde(default)]
    #[builder(default)]
    pub version: Version,

    #[serde(default)]
    #[builder(default)]
    pub repo: String,

    #[serde(default)]
    #[builder(default)]
    pub branch: String,

    #[serde(default)]
    #[builder(default)]
    pub github_account: String,

    #[serde(default)]
    #[builder(default)]
    pub docker_account: String,

    #[serde(default)]
    #[builder(default)]
    pub docker_organization: String,

    #[serde(default)]
    #[builder(default)]
    pub pre_build: SystemCommand,

    #[serde(default)]
    #[builder(default)]
    pub build_path: String,

    #[serde(default)]
    #[builder(default)]
    pub dockerfile_path: String,

    #[serde(default)]
    #[builder(default)]
    pub build_args: Vec<EnvironmentVar>,

    #[serde(default)]
    #[builder(default)]
    pub extra_args: Vec<String>,

    #[serde(default)]
    #[builder(default)]
    pub use_buildx: bool,

    #[serde(default)]
    #[builder(default)]
    pub tags: Vec<String>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildActionState {
    pub building: bool,
    pub updating: bool,
}