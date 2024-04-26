use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::update::ResourceTarget;

use super::MonitorReadRequest;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TomlResponse {
  pub toml: String,
}

//

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
