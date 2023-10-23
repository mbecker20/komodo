use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Alert {
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: String,
}
