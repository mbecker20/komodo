use derive_variants::EnumVariants;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use typeshare::typeshare;

use super::{ResourceTarget, MongoId};

/// Representation of a User or UserGroups permission on a resource.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(
  feature = "mongo",
  derive(mongo_indexed::derive::MongoIndexed)
)]
// To query for all permissions on user target
#[cfg_attr(feature = "mongo", doc_index({ "user_target.type": 1, "user_target.id": 1 }))]
// To query for all permissions on a resource target
#[cfg_attr(feature = "mongo", doc_index({ "resource_target.type": 1, "resource_target.id": 1 }))]
// Only one permission allowed per user / resource target
#[cfg_attr(feature = "mongo", unique_doc_index({
  "user_target.type": 1,
  "user_target.id": 1,
  "resource_target.type": 1,
  "resource_target.id": 1
}))]
pub struct Permission {
  /// The id of the permission document
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "bson::serde_helpers::hex_string_as_object_id"
  )]
  pub id: MongoId,
  /// The target User / UserGroup
  pub user_target: UserTarget,
  /// The target resource
  pub resource_target: ResourceTarget,
  /// The permission level
  #[serde(default)]
  pub level: PermissionLevel,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, EnumVariants)]
#[variant_derive(
  Debug,
  Clone,
  Copy,
  Serialize,
  Deserialize,
  AsRefStr
)]
#[serde(tag = "type", content = "id")]
pub enum UserTarget {
  /// User Id
  User(String),
  /// UserGroup Id
  UserGroup(String),
}

impl UserTarget {
  pub fn extract_variant_id(self) -> (UserTargetVariant, String) {
    match self {
      UserTarget::User(id) => (UserTargetVariant::User, id),
      UserTarget::UserGroup(id) => (UserTargetVariant::UserGroup, id),
    }
  }
}

/// The levels of permission that a User or UserGroup can have on a resource.
#[typeshare]
#[derive(
  Serialize,
  Deserialize,
  Debug,
  Display,
  EnumString,
  AsRefStr,
  Hash,
  Clone,
  Copy,
  PartialEq,
  Eq,
  PartialOrd,
  Ord,
  Default,
)]
pub enum PermissionLevel {
  /// No permissions.
  #[default]
  None,
  /// Can see the rousource
  Read,
  /// Can execute actions on the resource
  Execute,
  /// Can update the resource configuration
  Write,
}

impl Default for &PermissionLevel {
  fn default() -> Self {
    &PermissionLevel::None
  }
}
