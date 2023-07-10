use derive_builder::Builder;
use derive_variants::EnumVariants;
use mungos::{
    mongodb::bson::{doc, serde_helpers::hex_string_as_object_id},
    MungosIndexed,
};
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use typeshare::typeshare;

use crate::{i64_is_zero, MongoId, I64};

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
    pub last_built_at: I64,

    #[serde(default)]
    #[builder(default)]
    pub tags: Vec<String>,

    pub config: BuildConfig,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial, MungosIndexed)]
#[partial_derive(Serialize, Deserialize, Debug, Clone)]
#[skip_serializing_none]
pub struct BuildConfig {
    pub builder: BuildBuilderConfig,

    #[serde(default)]
    #[builder(default)]
    pub skip_secret_interp: bool,

    #[serde(default)]
    #[builder(default)]
    pub version: Version,

    #[serde(default)]
    #[builder(default)]
    pub repo: String,

    #[serde(default = "default_branch")]
    #[builder(default = "default_branch()")]
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

    #[serde(default = "default_build_path")]
    #[builder(default = "default_build_path()")]
    pub build_path: String,

    #[serde(default = "default_dockerfile_path")]
    #[builder(default = "default_dockerfile_path()")]
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
}

fn default_branch() -> String {
    String::from("main")
}

fn default_build_path() -> String {
    String::from(".")
}

fn default_dockerfile_path() -> String {
    String::from("Dockerfile")
}

impl From<PartialBuildConfig> for BuildConfig {
    fn from(value: PartialBuildConfig) -> BuildConfig {
        BuildConfig {
            builder: value.builder.unwrap_or_default(),
            skip_secret_interp: value.skip_secret_interp.unwrap_or_default(),
            version: value.version.unwrap_or_default(),
            repo: value.repo.unwrap_or_default(),
            branch: value.branch.unwrap_or(default_branch()),
            github_account: value.github_account.unwrap_or_default(),
            docker_account: value.docker_account.unwrap_or_default(),
            docker_organization: value.docker_organization.unwrap_or_default(),
            pre_build: value.pre_build.unwrap_or_default(),
            build_path: value.build_path.unwrap_or(default_build_path()),
            dockerfile_path: value.dockerfile_path.unwrap_or(default_dockerfile_path()),
            build_args: value.build_args.unwrap_or_default(),
            extra_args: value.extra_args.unwrap_or_default(),
            use_buildx: value.use_buildx.unwrap_or_default(),
        }
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildActionState {
    pub building: bool,
    pub updating: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, MungosIndexed, EnumVariants)]
#[variant_derive(Serialize, Deserialize, Debug, Clone, Copy, Display, EnumString)]
#[serde(tag = "type", content = "params")]
#[doc_index(doc! { "type": 1 })]
#[sparse_doc_index(doc! { "params.server_id": 1 })]
#[sparse_doc_index(doc! { "params.builder_id": 1 })]
pub enum BuildBuilderConfig {
    Server { server_id: String },
    Builder { builder_id: String },
}

impl Default for BuildBuilderConfig {
    fn default() -> Self {
        Self::Server {
            server_id: Default::default(),
        }
    }
}
