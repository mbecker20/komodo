use serde::{Deserialize, Serialize};

pub mod build;
pub mod resource;

#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
pub struct Version {
  pub major: i32,
  pub minor: i32,
  pub patch: i32,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq,
)]
pub struct SystemCommand {
  #[serde(default)]
  pub path: String,
  #[serde(default)]
  pub command: String,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
pub struct EnvironmentVar {
  pub variable: String,
  pub value: String,
}