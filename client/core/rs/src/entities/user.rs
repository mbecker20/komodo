use mongo_indexed::derive::MongoIndexed;
use mungos::mongodb::bson::{
  serde_helpers::hex_string_as_object_id, Document,
};
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{MongoId, I64};

use super::update::ResourceTarget;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, MongoIndexed,
)]
pub struct User {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: MongoId,

  #[unique_index]
  pub username: String,

  #[serde(default)]
  #[index]
  pub enabled: bool,

  #[serde(default)]
  pub admin: bool,

  #[serde(default)]
  pub create_server_permissions: bool,

  #[serde(default)]
  pub create_build_permissions: bool,

  pub avatar: Option<String>,

  pub password: Option<String>,

  #[sparse_index]
  pub github_id: Option<String>,

  #[sparse_index]
  pub google_id: Option<String>,

  #[serde(default)]
  pub last_update_view: I64,

  #[serde(default)]
  pub recently_viewed: Vec<ResourceTarget>,

  #[serde(default)]
  pub updated_at: I64,
}

impl User {
  /// Prepares user object for transport by removing any sensitive fields
  pub fn sanitize(&mut self) {
    self.password = None;
  }
}
