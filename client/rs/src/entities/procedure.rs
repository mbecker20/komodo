use derive_variants::EnumVariants;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::api::execute::Execution;

use super::resource::{Resource, ResourceListItem};

#[typeshare]
pub type Procedure = Resource<ProcedureConfig, ()>;

#[typeshare]
pub type ProcedureListItem = ResourceListItem<ProcedureListItemInfo>;

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProcedureListItemInfo {
  pub procedure_type: ProcedureConfigVariant,
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, EnumVariants)]
#[variant_derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(tag = "type", content = "data")]
pub enum ProcedureConfig {
  Execution(Execution),
  /// Vec<ProcedureId>
  Sequence(Vec<String>),
  /// Vec<ProdecureId>
  Parallel(Vec<String>),
}

impl From<&ProcedureConfig> for ProcedureConfigVariant {
  fn from(value: &ProcedureConfig) -> Self {
    match value {
      ProcedureConfig::Execution(_) => {
        ProcedureConfigVariant::Execution
      }
      ProcedureConfig::Parallel(_) => {
        ProcedureConfigVariant::Parallel
      }
      ProcedureConfig::Sequence(_) => {
        ProcedureConfigVariant::Sequence
      }
    }
  }
}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ProcedureActionState {
  pub running: bool
}