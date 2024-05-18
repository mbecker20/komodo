use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::{MongoId, I64};

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

  #[cfg_attr(feature = "mongo", unique_index)]
  pub name: String,

  /// User ids
  #[cfg_attr(feature = "mongo", index)]
  pub users: Vec<String>,

  #[serde(default)]
  pub updated_at: I64,
}
