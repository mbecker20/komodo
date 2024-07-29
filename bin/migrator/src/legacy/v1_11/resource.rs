use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource<Config, Info: Default = ()> {
  /// The Mongo ID of the resource.
  /// This field is de/serialized from/to JSON as
  /// `{ "_id": { "$oid": "..." }, ...(rest of serialized Resource<T>) }`
  #[serde(
    default,
    rename = "_id",
    skip_serializing_if = "String::is_empty",
    with = "hex_string_as_object_id"
  )]
  pub id: String,

  /// The resource name.
  /// This is guaranteed unique among others of the same resource type.
  pub name: String,

  /// A description for the resource
  #[serde(default)]
  pub description: String,

  /// When description last updated
  #[serde(default)]
  pub updated_at: i64,

  /// Tag Ids
  #[serde(default)]
  pub tags: Vec<String>,

  /// Resource-specific information (not user configurable).
  #[serde(default)]
  pub info: Info,

  /// Resource-specific configuration.
  pub config: Config,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourceListItem<Info> {
  /// The resource id
  pub id: String,
  /// The resource type, ie `Server` or `Deployment`
  // #[serde(rename = "type")]
  // pub resource_type: ResourceTargetVariant,
  /// The resource name
  pub name: String,
  /// Tag Ids
  pub tags: Vec<String>,
  /// Resource specific info
  pub info: Info,
}
