use mongo_indexed::derive::MongoIndexed;
use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use super::{MongoId, I64};

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, MongoIndexed,
)]
pub struct UserGroup {
  /// The Mongo ID of the UserGroup.
  /// This field is de/serialized from/to JSON as
  /// `{ "_id": { "$oid": "..." }, ...(rest of serialized User) }`
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: MongoId,

  #[unique_index]
  pub name: String,

  /// User ids
  #[index]
  pub users: Vec<String>,

  #[serde(default)]
  pub updated_at: I64,
}
