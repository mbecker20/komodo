use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::user_group::UserGroup;

use super::MonitorWriteRequest;

/// Admin only
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

/// Admin only
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

/// Admin only
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

/// Admin only
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

/// Admin only
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
