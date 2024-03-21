use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::procedure::{Procedure, ProcedureConfig};

use super::MonitorWriteRequest;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateProcedureResponse)]
pub struct CreateProcedure {
  pub name: String,
  pub config: ProcedureConfig,
}

#[typeshare]
pub type CreateProcedureResponse = Procedure;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CopyProcedureResponse)]
pub struct CopyProcedure {
  pub name: String,
  pub id: String,
}

#[typeshare]
pub type CopyProcedureResponse = Procedure;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(DeleteProcedureResponse)]
pub struct DeleteProcedure {
  pub id: String,
}

#[typeshare]
pub type DeleteProcedureResponse = Procedure;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UpdateProcedureResponse)]
pub struct UpdateProcedure {
  pub id: String,
  pub config: ProcedureConfig,
}

#[typeshare]
pub type UpdateProcedureResponse = Procedure;
