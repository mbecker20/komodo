use mongo_indexed::derive::MongoIndexed;
use mungos::mongodb::bson::{
  doc, serde_helpers::hex_string_as_object_id, Document,
};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use typeshare::typeshare;

use super::{update::ResourceTarget, MongoId};

/// Representation of a User or UserGroups permission on a resource.
#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, MongoIndexed)]
// To query for all permissions on a target
#[doc_index(doc! { "target.type": 1, "target.id": 1 })]
// Only one permission allowed per user / target
#[unique_doc_index(doc! { "user_id": 1, "target.type": 1, "target.id": 1 })]
pub struct Permission {
  /// The id of the permission document
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: MongoId,
  /// Attached user
  #[index]
  pub user_target: UserTarget,
  /// The target resource
  pub resource_target: ResourceTarget,
  /// The permission level
  #[serde(default)]
  pub level: PermissionLevel,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "id")]
pub enum UserTarget {
  /// User Id
  User(String),
  /// UserGroup Id
  UserGroup(String),
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
