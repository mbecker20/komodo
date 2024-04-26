use serde::{Deserialize, Serialize};

use super::{
  alerter::PartialAlerterConfig, build::PartialBuildConfig,
  builder::PartialBuilderConfig, deployment::PartialDeploymentConfig,
  permission::PermissionLevel, procedure::PartialProcedureConfig,
  repo::PartialRepoConfig, resource::Resource,
  server::PartialServerConfig, update::ResourceTarget,
};

/// Specifies resources to sync on monitor
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourcesToml {
  #[serde(
    default,
    rename = "server",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub servers: Vec<Resource<PartialServerConfig>>,

  #[serde(
    default,
    rename = "build",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub builds: Vec<Resource<PartialBuildConfig>>,

  #[serde(
    default,
    rename = "deployment",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub deployments: Vec<Resource<PartialDeploymentConfig>>,

  #[serde(
    default,
    rename = "builder",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub builders: Vec<Resource<PartialBuilderConfig>>,

  #[serde(
    default,
    rename = "repo",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub repos: Vec<Resource<PartialRepoConfig>>,

  #[serde(
    default,
    rename = "alerter",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub alerters: Vec<Resource<PartialAlerterConfig>>,

  #[serde(
    default,
    rename = "procedure",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub procedures: Vec<Resource<PartialProcedureConfig>>,

  #[serde(
    default,
    rename = "user_group",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub user_groups: Vec<UserGroupToml>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroupToml {
  pub name: String,

  #[serde(default)]
  pub users: Vec<String>,

  #[serde(default, rename = "permission")]
  pub permissions: Vec<PermissionToml>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionToml {
  pub target: ResourceTarget,
  pub level: PermissionLevel,
}
