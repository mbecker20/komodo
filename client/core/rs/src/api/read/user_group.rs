use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::user_group::UserGroup;

use super::KomodoReadRequest;

/// Get a specific user group by name or id.
/// Response: [UserGroup].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(GetUserGroupResponse)]
pub struct GetUserGroup {
  /// Name or Id
  pub user_group: String,
}

#[typeshare]
pub type GetUserGroupResponse = UserGroup;

//

/// List all user groups which user can see. Response: [ListUserGroupsResponse].
///
/// Admins can see all user groups,
/// and users can see user groups to which they belong.
#[typeshare]
#[derive(
  Debug, Clone, Default, Serialize, Deserialize, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(ListUserGroupsResponse)]
pub struct ListUserGroups {}

#[typeshare]
pub type ListUserGroupsResponse = Vec<UserGroup>;
