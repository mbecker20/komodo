use anyhow::{anyhow, Context};
use bson::serde_helpers::hex_string_as_object_id;
use derive_builder::Builder;
use diff::Diff;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{diff::*, Command, EnvironmentVar, PermissionsMap};

#[typeshare]
pub const PERIPHERY_BUILDER_BUSY: &str = "BUILDER_BUSY";

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, Diff, Builder)]
#[diff(attr(#[derive(Debug, Serialize)]))]
pub struct Build {
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
    #[diff(attr(#[serde(skip_serializing)]))]
    #[builder(setter(skip))]
    pub permissions: PermissionsMap,

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub server_id: Option<String>, // server which this image should be built on

    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub aws_config: Option<AwsBuilderConfig>,

    #[builder(default)]
    pub version: Version,

    // git related
    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub repo: Option<String>,

    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub branch: Option<String>,

    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub github_account: Option<String>,

    // build related
    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub pre_build: Option<Command>,

    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub docker_build_args: Option<DockerBuildArgs>,

    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub docker_account: Option<String>,

    #[serde(default)]
    #[diff(attr(#[serde(skip)]))]
    #[builder(setter(skip))]
    pub last_built_at: String,

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
pub struct BuildActionState {
    pub building: bool,
    pub recloning: bool,
    pub updating: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Diff)]
#[diff(attr(#[derive(Debug, PartialEq, Serialize)]))]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl TryFrom<&str> for Version {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let vals = value
            .split(".")
            .map(|v| anyhow::Ok(v.parse().context("failed at parsing value into i32")?))
            .collect::<anyhow::Result<Vec<i32>>>()?;
        let version = Version {
            major: *vals
                .get(0)
                .ok_or(anyhow!("must include at least major version"))?,
            minor: *vals.get(1).unwrap_or(&0),
            patch: *vals.get(2).unwrap_or(&0),
        };
        Ok(version)
    }
}

impl Version {
    pub fn increment(&mut self) {
        self.patch += 1;
    }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Diff, Builder)]
#[diff(attr(#[derive(Debug, Serialize, PartialEq)]))]
pub struct DockerBuildArgs {
    pub build_path: String,
    #[builder(default)]
    pub dockerfile_path: Option<String>,
    #[serde(default)]
    #[builder(default)]
    pub build_args: Vec<EnvironmentVar>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildVersionsReponse {
    pub version: Version,
    pub ts: String,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default, Diff, Builder)]
#[diff(attr(#[derive(Debug, Serialize, PartialEq)]))]
pub struct AwsBuilderConfig {
    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub region: Option<String>,
    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub instance_type: Option<String>,
    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub ami_id: Option<String>,
    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub volume_gb: Option<i32>,
    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub subnet_id: Option<String>,
    #[builder(default)]
    #[diff(attr(#[serde(skip_serializing_if = "option_diff_no_change")]))]
    pub security_group_ids: Option<Vec<String>>,
}
