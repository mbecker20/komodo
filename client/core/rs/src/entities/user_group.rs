use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::{
  permission::PermissionLevel, update::ResourceTargetVariant,
  MongoId, I64,
};

/// Permission users at the group level.
///
/// All users that are part of a group inherit the group's permissions.
/// A user can be a part of multiple groups. A user's permission on a particular resource
/// will be resolved to be the maximum permission level between the user's own permissions and
/// any groups they are a part of.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(
  feature = "mongo",
  derive(mongo_indexed::derive::MongoIndexed)
)]
pub struct UserGroup {
  /// The Mongo ID of the UserGroup.
  /// This field is de/serialized from/to JSON as
  /// `{ "_id": { "$oid": "..." }, ...(rest of serialized User) }`
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "bson::serde_helpers::hex_string_as_object_id"
  )]
  pub id: MongoId,

  /// A name for the user group
  #[cfg_attr(feature = "mongo", unique_index)]
  pub name: String,

  /// User ids of group members
  #[cfg_attr(feature = "mongo", index)]
  pub users: Vec<String>,

  /// Give the user group elevated permissions on all resources of a certain type
  #[serde(default)]
  pub all: HashMap<ResourceTargetVariant, PermissionLevel>,

  /// Unix time (ms) when user group last updated
  #[serde(default)]
  pub updated_at: I64,
}
