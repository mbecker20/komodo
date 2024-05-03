use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::update::ResourceTarget;

use super::MonitorReadRequest;

/// Response containing pretty formatted toml contents.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlResponse {
  pub toml: String,
}

//

/// Get pretty formatted monrun sync toml for all resources
/// which the user has permissions to view.
/// Response: [TomlResponse].
#[typeshare]
#[derive(
  Debug, Clone, Default, Serialize, Deserialize, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ExportAllResourcesToTomlResponse)]
pub struct ExportAllResourcesToToml {}

#[typeshare]
pub type ExportAllResourcesToTomlResponse = TomlResponse;

//

/// Get pretty formatted monrun sync toml for specific resources and user groups.
/// Response: [TomlResponse].
#[typeshare]
#[derive(
  Debug, Clone, Default, Serialize, Deserialize, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ExportResourcesToTomlResponse)]
pub struct ExportResourcesToToml {
  /// The targets to include in the export.
  pub targets: Vec<ResourceTarget>,
  /// The user group names or ids to include in the export.
  pub user_groups: Vec<String>,
}

#[typeshare]
pub type ExportResourcesToTomlResponse = TomlResponse;
