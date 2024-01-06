use mongo_indexed::derive::MongoIndexed;
use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::{MongoId, I64};

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

  #[serde(default)]
  pub secrets: Vec<ApiSecret>,

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

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
pub struct ApiSecret {
  pub name: String,
  #[serde(default, skip_serializing_if = "String::is_empty")]
  pub hash: String,
  pub created_at: I64,
  pub expires: Option<I64>,
}
