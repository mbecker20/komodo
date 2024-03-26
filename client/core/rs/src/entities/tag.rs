use derive_builder::Builder;
use mongo_indexed::derive::MongoIndexed;
use mungos::mongodb::bson::{
  doc, serde_helpers::hex_string_as_object_id, Document,
};
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::MongoId;

#[typeshare(serialized_as = "Partial<Tag>")]
pub type _PartialTag = PartialTag;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Builder, Partial, MongoIndexed,
)]
#[partial_derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tag {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  #[builder(setter(skip))]
  pub id: MongoId,

  #[unique_index]
  pub name: String,

  #[serde(default)]
  #[builder(default)]
  #[index]
  pub owner: String,
}
