use bson::{doc, Document};
use derive_builder::Builder;
use derive_default_builder::DefaultBuilder;
use partial_derive2::Partial;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString, IntoStaticStr};
use typeshare::typeshare;

use crate::api::execute::Execution;

use super::resource::{Resource, ResourceListItem, ResourceQuery};

// List item

#[typeshare]
pub type ProcedureListItem = ResourceListItem<ProcedureListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcedureListItemInfo {
  /// Sequence or Parallel.
  pub procedure_type: ProcedureType,
  /// Reflect whether last run successful / currently running.
  pub state: ProcedureState,
}

#[typeshare]
#[derive(
  Debug, Clone, Copy, Default, Serialize, Deserialize, Display,
)]
pub enum ProcedureState {
  /// Last run successful
  Ok,
  /// Last run failed
  Failed,
  /// Currently running
  Running,
  /// Other case (never run)
  #[default]
  Unknown,
}

#[typeshare]
pub type Procedure = Resource<ProcedureConfig, ()>;

#[typeshare(serialized_as = "Partial<ProcedureConfig>")]
pub type _PartialProcedureConfig = PartialProcedureConfig;

#[typeshare]
#[derive(
  Debug, Clone, Default, Serialize, Deserialize, Partial, Builder,
)]
#[partial_derive(Debug, Clone, Default, Serialize, Deserialize)]
#[partial(skip_serializing_none, from, diff)]
pub struct ProcedureConfig {
  /// Whether executions in the procedure runs sequentially or in parallel.
  #[serde(default)]
  #[builder(default)]
  pub procedure_type: ProcedureType,
  /// The executions to be run by the procedure.
  #[serde(default)]
  #[builder(default)]
  pub executions: Vec<EnabledExecution>,

  /// Whether incoming webhooks actually trigger action.
  #[serde(default = "default_webhook_enabled")]
  #[builder(default = "default_webhook_enabled()")]
  #[partial_default(default_webhook_enabled())]
  pub webhook_enabled: bool,
}

fn default_webhook_enabled() -> bool {
  true
}

#[typeshare]
#[derive(
  Debug,
  Clone,
  Copy,
  Default,
  PartialEq,
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
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl super::resource::AddFilters for ProcedureQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    let types =
      self.types.iter().map(|t| t.as_ref()).collect::<Vec<_>>();
    if !self.types.is_empty() {
      filters.insert("config.procedure_type", doc! { "$in": types });
    }
  }
}
