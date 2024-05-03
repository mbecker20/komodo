use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

use super::PermissionsMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Group {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: String,

  pub name: String,

  #[serde(default)]
  pub description: String,

  #[serde(default)]
  pub permissions: PermissionsMap,

  pub builds: Vec<String>,

  pub deployments: Vec<String>,

  pub servers: Vec<String>,

  pub procedures: Vec<String>,

  pub groups: Vec<String>,

  #[serde(default, skip_serializing_if = "String::is_empty")]
  pub created_at: String,
  #[serde(default)]
  pub updated_at: String,
}
