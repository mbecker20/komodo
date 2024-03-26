use mongo_indexed::derive::MongoIndexed;
use mungos::mongodb::bson::{
  doc, serde_helpers::hex_string_as_object_id, Document,
};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use typeshare::typeshare;

use super::{update::ResourceTarget, MongoId};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize, MongoIndexed)]
// To query for all permissions on a target
#[doc_index(doc! { "target.type": 1, "target.id": 1 })]
// Only one permission allowed per user / target
#[unique_doc_index(doc! { "user_id": 1, "target.type": 1, "target.id": 1 })]
pub struct Permission {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: MongoId,
  #[index]
  pub user_id: String,
  pub target: ResourceTarget,
  #[serde(default)]
  pub level: PermissionLevel,
}

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
  #[default]
  None,
  Read,
  Execute,
  Write,
}

impl Default for &PermissionLevel {
  fn default() -> Self {
    &PermissionLevel::None
  }
}
