use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  procedure::{Procedure, ProcedureActionState, ProcedureListItem},
  MongoDocument,
};

use super::MonitorReadRequest;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetProcedureResponse)]
pub struct GetProcedure {
  pub id: String,
}

#[typeshare]
pub type GetProcedureResponse = Procedure;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListProceduresResponse)]
pub struct ListProcedures {
  pub query: Option<MongoDocument>,
}

#[typeshare]
pub type ListProceduresResponse = Vec<ProcedureListItem>;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListProceduresByIdsResponse)]
pub struct ListProceduresByIds {
  pub ids: Vec<String>,
}

#[typeshare]
pub type ListProceduresByIdsResponse = Vec<ProcedureListItem>;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetProceduresSummaryResponse)]
pub struct GetProceduresSummary {}

#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetProceduresSummaryResponse {
  pub total: u32,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetProcedureActionStateResponse)]
pub struct GetProcedureActionState {
  pub id: String,
}

#[typeshare]
pub type GetProcedureActionStateResponse = ProcedureActionState;

//
