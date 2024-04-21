use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  permission::{Permission, PermissionLevel, UserTarget},
  update::ResourceTarget,
};

use super::MonitorReadRequest;

/// List permissions for the calling user.
/// Does not include any permissions on UserGroups they may be a part of.
/// Response: [ListPermissionsResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListPermissionsResponse)]
pub struct ListPermissions {}

#[typeshare]
pub type ListPermissionsResponse = Vec<Permission>;

//

/// Gets the calling user's permission level on a specific resource.
/// Factors in any UserGroup's permissions they may be a part of.
/// Response: [PermissionLevel]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetPermissionLevelResponse)]
pub struct GetPermissionLevel {
  pub target: ResourceTarget,
}

#[typeshare]
pub type GetPermissionLevelResponse = PermissionLevel;

//

/// List permissions for a specific user. **Admin only**.
/// Response: [ListUserTargetPermissionsResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListUserTargetPermissionsResponse)]
pub struct ListUserTargetPermissions {
  pub user_target: UserTarget,
}

#[typeshare]
pub type ListUserTargetPermissionsResponse = Vec<Permission>;
