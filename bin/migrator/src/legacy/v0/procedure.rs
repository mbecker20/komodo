use mungos::mongodb::bson::serde_helpers::hex_string_as_object_id;
use serde::{Deserialize, Serialize};

use super::PermissionsMap;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Procedure {
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
  pub stages: Vec<ProcedureStage>,

  #[serde(default)]
  pub webhook_branches: Vec<String>,

  #[serde(default)]
  pub permissions: PermissionsMap,

  #[serde(default, skip_serializing_if = "String::is_empty")]
  pub created_at: String,
  #[serde(default)]
  pub updated_at: String,
}

#[derive(
  Serialize, Deserialize, Debug, Clone, Default, PartialEq,
)]
pub struct ProcedureStage {
  pub operation: ProcedureOperation,
  pub target_id: String,
}

#[derive(
  Serialize,
  Deserialize,
  Debug,
  PartialEq,
  Hash,
  Eq,
  Clone,
  Copy,
  Default,
)]
#[serde(rename_all = "snake_case")]
pub enum ProcedureOperation {
  // do nothing
  #[default]
  None,

  // server
  PruneImagesServer,
  PruneContainersServer,
  PruneNetworksServer,

  // build
  BuildBuild,

  // deployment
  DeployContainer,
  StopContainer,
  StartContainer,
  RemoveContainer,
  PullDeployment,
  RecloneDeployment,

  // procedure
  RunProcedure,
}
