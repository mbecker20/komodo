use derive_default_builder::DefaultBuilder;
use mungos::mongodb::bson::{doc, Document};
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};
use typeshare::typeshare;

use crate::api::execute::Execution;

use super::resource::{
  AddFilters, Resource, ResourceListItem, ResourceQuery,
};

// List item

#[typeshare]
pub type ProcedureListItem = ResourceListItem<ProcedureListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcedureListItemInfo {
  pub procedure_type: ProcedureType,
}

#[typeshare]
pub type Procedure = Resource<ProcedureConfig, ()>;

#[typeshare(serialized_as = "Partial<ProcedureConfig>")]
pub type _PartialProcedureConfig = PartialProcedureConfig;

#[typeshare]
#[derive(Debug, Clone, Default, Serialize, Deserialize, Partial)]
#[partial_derive(Debug, Clone, Default, Serialize, Deserialize)]
#[skip_serializing_none]
#[partial_from]
pub struct ProcedureConfig {
  #[serde(default)]
  pub procedure_type: ProcedureType,
  #[serde(default)]
  pub executions: Vec<EnabledExecution>,
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  Serialize,
  Deserialize,
  Display,
  EnumString,
  AsRefStr,
  IntoStaticStr,
)]
pub enum ProcedureType {
  #[default]
  Sequence,
  Parallel,
}

/// Allows to enable / disabled procedures in the sequence / parallel vec on the fly
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnabledExecution {
  /// The execution request to run.
  pub execution: Execution,
  /// Whether the execution is enabled to run in the procedure.
  pub enabled: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProcedureActionState {
  pub running: bool,
}

// QUERY

#[typeshare]
pub type ProcedureQuery = ResourceQuery<ProcedureQuerySpecifics>;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, DefaultBuilder,
)]
pub struct ProcedureQuerySpecifics {
  pub types: Vec<ProcedureType>,
}

impl AddFilters for ProcedureQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    let types =
      self.types.iter().map(|t| t.as_ref()).collect::<Vec<_>>();
    if !self.types.is_empty() {
      filters.insert("config.procedure_type", doc! { "$in": types });
    }
  }
}
