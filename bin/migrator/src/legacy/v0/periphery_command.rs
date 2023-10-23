use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

use super::{Command, PermissionsMap};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct PeripheryCommand {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: String,

  pub name: String, // must be formatted to be compat with docker

  #[serde(default)]
  pub description: String,

  pub server_id: String,

  #[serde(default)]
  pub permissions: PermissionsMap,

  #[serde(default)]
  pub command: Command,

  #[serde(default, skip_serializing_if = "String::is_empty")]
  pub created_at: String,
  #[serde(default)]
  pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CommandActionState {
  pub running: bool,
}
