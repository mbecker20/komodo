use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::procedure::{
  Procedure, ProcedureActionState, ProcedureListItem, ProcedureQuery,
};

use super::KomodoReadRequest;

//

/// Get a specific procedure. Response: [Procedure].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(GetProcedureResponse)]
pub struct GetProcedure {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub procedure: String,
}

#[typeshare]
pub type GetProcedureResponse = Procedure;

//

/// List procedures matching optional query. Response: [ListProceduresResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(ListProceduresResponse)]
pub struct ListProcedures {
  /// optional structured query to filter procedures.
  #[serde(default)]
  pub query: ProcedureQuery,
}

#[typeshare]
pub type ListProceduresResponse = Vec<ProcedureListItem>;

//

/// List procedures matching optional query. Response: [ListFullProceduresResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Default, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(ListFullProceduresResponse)]
pub struct ListFullProcedures {
  /// optional structured query to filter procedures.
  #[serde(default)]
  pub query: ProcedureQuery,
}

#[typeshare]
pub type ListFullProceduresResponse = Vec<Procedure>;

//

/// Get current action state for the procedure. Response: [ProcedureActionState].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(GetProcedureActionStateResponse)]
pub struct GetProcedureActionState {
  /// Id or name
  #[serde(alias = "id", alias = "name")]
  pub procedure: String,
}

#[typeshare]
pub type GetProcedureActionStateResponse = ProcedureActionState;

//

/// Gets a summary of data relating to all procedures.
/// Response: [GetProceduresSummaryResponse].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(KomodoReadRequest)]
#[response(GetProceduresSummaryResponse)]
pub struct GetProceduresSummary {}

/// Response for [GetProceduresSummary].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GetProceduresSummaryResponse {
  /// The total number of procedures.
  pub total: u32,
  /// The number of procedures with Ok state.
  pub ok: u32,
  /// The number of procedures currently running.
  pub running: u32,
  /// The number of procedures with failed state.
  pub failed: u32,
  /// The number of procedures with unknown state.
  pub unknown: u32,
}
