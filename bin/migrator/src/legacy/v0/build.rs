use anyhow::{anyhow, Context};
use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

use super::{Command, EnvironmentVar, PermissionsMap};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Build {
    #[serde(
        default,
        rename = "_id",
        skip_serializing_if = "String::is_empty",
        with = "hex_string_as_object_id"
    )]
    pub id: String,

    pub name: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub permissions: PermissionsMap,

    #[serde(default)]
    pub skip_secret_interp: bool,

    pub server_id: Option<String>, // server which this image should be built on

    pub aws_config: Option<AwsBuilderBuildConfig>,

    pub version: Version,

    // git related
    pub repo: Option<String>,

    pub branch: Option<String>,

    pub github_account: Option<String>,

    // build related
    pub pre_build: Option<Command>,

    pub docker_build_args: Option<DockerBuildArgs>,

    pub docker_account: Option<String>,

    pub docker_organization: Option<String>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub last_built_at: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub created_at: String,
    #[serde(default)]
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildActionState {
    pub building: bool,
    pub updating: bool,
}

#[derive(
    Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
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
        let vals =
            value
                .split('.')
                .map(|v| {
                    anyhow::Ok(v.parse().context(
                        "failed at parsing value into i32",
                    )?)
                })
                .collect::<anyhow::Result<Vec<i32>>>()?;
        let version = Version {
            major: *vals.first().ok_or(anyhow!(
                "must include at least major version"
            ))?,
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

#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Default,
)]
pub struct DockerBuildArgs {
    pub build_path: String,
    pub dockerfile_path: Option<String>,
    #[serde(default)]
    pub build_args: Vec<EnvironmentVar>,
    #[serde(default)]
    pub extra_args: Vec<String>,
    #[serde(default)]
    pub use_buildx: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct BuildVersionsReponse {
    pub version: Version,
    pub ts: String,
}

#[derive(
    Serialize, Deserialize, Debug, Clone, PartialEq, Default,
)]
pub struct AwsBuilderBuildConfig {
    pub region: Option<String>,

    pub instance_type: Option<String>,

    pub ami_name: Option<String>,

    pub volume_gb: Option<i32>,

    pub subnet_id: Option<String>,

    pub security_group_ids: Option<Vec<String>>,

    pub key_pair_name: Option<String>,

    pub assign_public_ip: Option<bool>,
}
