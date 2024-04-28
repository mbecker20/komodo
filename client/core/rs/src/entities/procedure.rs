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
#[partial(skip_serializing_none, from)]
pub struct ProcedureConfig {
  /// Whether executions in the procedure runs sequentially or in parallel.
  #[serde(default)]
  pub procedure_type: ProcedureType,
  /// The executions to be run by the procedure.
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
  /// Run the executions one after the other, in order of increasing index.
  #[default]
  Sequence,
  /// Start all the executions simultaneously.
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
#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
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
