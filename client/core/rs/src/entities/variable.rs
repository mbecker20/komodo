use serde::{Deserialize, Serialize};
use typeshare::typeshare;

/// A non-secret global variable which can be interpolated into deployment
/// environment variable values and build argument values.
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[cfg_attr(
  feature = "mongo",
  derive(mongo_indexed::derive::MongoIndexed)
)]
pub struct Variable {
  /// Unique name associated with the variable.
  /// Instances of '[[variable.name]]' in value will be replaced with 'variable.value'.
  #[cfg_attr(feature = "mongo", unique_index)]
  pub name: String,
  /// A description for the variable.
  #[serde(default, skip_serializing_if = "String::is_empty")]
  pub description: String,
  /// The value associated with the variable.
  #[serde(default)]
  pub value: String,
}
