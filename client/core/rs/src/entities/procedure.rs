use derive_default_builder::DefaultBuilder;
use derive_variants::EnumVariants;
use mungos::mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};
use typeshare::typeshare;

use crate::api::execute::Execution;

use super::resource::{
  AddFilters, Resource, ResourceListItem, ResourceQuery,
};

#[typeshare]
pub type Procedure = Resource<ProcedureConfig, ()>;

#[typeshare]
pub type ProcedureListItem = ResourceListItem<ProcedureListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcedureListItemInfo {
  pub procedure_type: ProcedureConfigVariant,
}

// #[typeshare(serialized_as = "ProcedureConfig['type']")]
// pub type _ProcedureConfigVariant = ProcedureConfigVariant;

/// Allows to enable / disabled procedures in the sequence / parallel vec on the fly
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EnabledExecution {
  pub execution: Execution,
  pub enabled: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, EnumVariants)]
#[variant_derive(
  Serialize,
  Deserialize,
  Debug,
  Clone,
  Copy,
  Display,
  EnumString,
  AsRefStr
)]
#[serde(tag = "type", content = "data")]
pub enum ProcedureConfig {
  Sequence(Vec<EnabledExecution>),
  Parallel(Vec<EnabledExecution>),
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProcedureActionState {
  pub running: bool,
}

#[typeshare]
pub type ProcedureQuery = ResourceQuery<ProcedureQuerySpecifics>;

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, DefaultBuilder,
)]
pub struct ProcedureQuerySpecifics {
  pub types: Vec<ProcedureConfigVariant>,
}

impl AddFilters for ProcedureQuerySpecifics {
  fn add_filters(&self, filters: &mut Document) {
    let types =
      self.types.iter().map(|t| t.as_ref()).collect::<Vec<_>>();
    if !self.types.is_empty() {
      filters.insert("config.type", doc! { "$in": types });
    }
  }
}
