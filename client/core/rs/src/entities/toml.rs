use serde::{Deserialize, Serialize};

use super::{
  alerter::PartialAlerterConfig, build::PartialBuildConfig,
  builder::PartialBuilderConfig, deployment::PartialDeploymentConfig,
  permission::PermissionLevel, procedure::PartialProcedureConfig,
  repo::PartialRepoConfig, server::PartialServerConfig,
  server_template::PartialServerTemplateConfig,
  update::ResourceTarget, variable::Variable,
};

/// Specifies resources to sync on monitor
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResourcesToml {
  #[serde(
    default,
    rename = "server_template",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub server_templates:
    Vec<ResourceToml<PartialServerTemplateConfig>>,

  #[serde(
    default,
    rename = "server",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub servers: Vec<ResourceToml<PartialServerConfig>>,

  #[serde(
    default,
    rename = "build",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub builds: Vec<ResourceToml<PartialBuildConfig>>,

  #[serde(
    default,
    rename = "deployment",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub deployments: Vec<ResourceToml<PartialDeploymentConfig>>,

  #[serde(
    default,
    rename = "builder",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub builders: Vec<ResourceToml<PartialBuilderConfig>>,

  #[serde(
    default,
    rename = "repo",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub repos: Vec<ResourceToml<PartialRepoConfig>>,

  #[serde(
    default,
    rename = "alerter",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub alerters: Vec<ResourceToml<PartialAlerterConfig>>,

  #[serde(
    default,
    rename = "procedure",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub procedures: Vec<ResourceToml<PartialProcedureConfig>>,

  #[serde(
    default,
    rename = "user_group",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub user_groups: Vec<UserGroupToml>,

  #[serde(
    default,
    rename = "variable",
    skip_serializing_if = "Vec::is_empty"
  )]
  pub variables: Vec<Variable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceToml<PartialConfig> {
  /// The resource name. Required
  pub name: String,

  /// The resource description.
  #[serde(default)]
  pub description: String,

  /// Tag ids or names
  #[serde(default)]
  pub tags: Vec<String>,

  /// Resource specific configuration
  pub config: PartialConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroupToml {
  /// User group name
  pub name: String,

  /// Users in the group
  #[serde(default)]
  pub users: Vec<String>,

  /// Permissions given to the group
  #[serde(default, rename = "permission")]
  pub permissions: Vec<PermissionToml>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PermissionToml {
  pub target: ResourceTarget,
  pub level: PermissionLevel,
}
