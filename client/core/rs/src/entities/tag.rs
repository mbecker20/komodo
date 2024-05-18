use derive_builder::Builder;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::MongoId;

#[typeshare(serialized_as = "Partial<Tag>")]
pub type _PartialTag = PartialTag;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Builder, Partial)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(
  feature = "mongo",
  derive(mongo_indexed::derive::MongoIndexed)
)]
pub struct Tag {
  /// The Mongo ID of the tag.
  /// This field is de/serialized from/to JSON as
  /// `{ "_id": { "$oid": "..." }, ...(rest of serialized Tag) }`
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "bson::serde_helpers::hex_string_as_object_id"
  )]
  #[builder(setter(skip))]
  pub id: MongoId,

  #[cfg_attr(feature = "mongo", unique_index)]
  pub name: String,

  #[serde(default)]
  #[builder(default)]
  #[cfg_attr(feature = "mongo", index)]
  pub owner: String,
}
