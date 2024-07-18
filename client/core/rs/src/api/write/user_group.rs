use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  permission::PermissionLevel, update::ResourceTargetVariant,
  user_group::UserGroup,
};

use super::MonitorWriteRequest;

/// **Admin only.** Create a user group. Response: [UserGroup]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UserGroup)]
pub struct CreateUserGroup {
  /// The name to assign to the new UserGroup
  pub name: String,
}

//

/// **Admin only.** Rename a user group. Response: [UserGroup]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UserGroup)]
pub struct RenameUserGroup {
  /// The id of the UserGroup
  pub id: String,
  /// The new name for the UserGroup
  pub name: String,
}

//

/// **Admin only.** Delete a user group. Response: [UserGroup]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UserGroup)]
pub struct DeleteUserGroup {
  /// The id of the UserGroup
  pub id: String,
}

//

/// **Admin only.** Add a user to a user group. Response: [UserGroup]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UserGroup)]
pub struct AddUserToUserGroup {
  /// The name or id of UserGroup that user should be added to.
  pub user_group: String,
  /// The id or username of the user to add
  pub user: String,
}

//

/// **Admin only.** Remove a user from a user group. Response: [UserGroup]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UserGroup)]
pub struct RemoveUserFromUserGroup {
  /// The name or id of UserGroup that user should be removed from.
  pub user_group: String,
  /// The id or username of the user to remove
  pub user: String,
}

//

/// **Admin only.** Completely override the user in the group.
/// Response: [UserGroup]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UserGroup)]
pub struct SetUsersInUserGroup {
  /// Id or name.
  pub user_group: String,
  /// The user ids or usernames to hard set as the group's users.
  pub users: Vec<String>,
}

/// **Admin only.** Set the user group base permission levels on resource type
/// Response: [UserGroup]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UserGroup)]
pub struct SetUserGroupResourceBasePermission {
  /// Id or name.
  pub user_group: String,
  /// The resource type: eg. Server, Build, Deployment, etc.
  pub resource_type: ResourceTargetVariant,
  /// The base permission level.
  pub permission: PermissionLevel,
}
