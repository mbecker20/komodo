use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  permission::{PermissionLevel, UserTarget},
  update::ResourceTarget,
};

use super::MonitorWriteRequest;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UpdatePermissionOnTargetResponse)]
pub struct UpdatePermissionOnTarget {
  pub user_target: UserTarget,
  pub resource_target: ResourceTarget,
  pub permission: PermissionLevel,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatePermissionOnTargetResponse {}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UpdateUserBasePermissionsResponse)]
pub struct UpdateUserBasePermissions {
  pub user_id: String,
  pub enabled: Option<bool>,
  pub create_servers: Option<bool>,
  pub create_builds: Option<bool>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUserBasePermissionsResponse {}
