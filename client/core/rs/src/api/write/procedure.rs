use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::procedure::{
  Procedure, _PartialProcedureConfig,
};

use super::MonitorWriteRequest;

//

/// Create a procedure. Response: [Procedure].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateProcedureResponse)]
pub struct CreateProcedure {
  /// The name given to newly created build.
  pub name: String,
  /// Optional partial config to initialize the procedure with.
  pub config: _PartialProcedureConfig,
}

#[typeshare]
pub type CreateProcedureResponse = Procedure;

//

/// Creates a new procedure with given `name` and the configuration
/// of the procedure at the given `id`. Response: [Procedure].
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CopyProcedureResponse)]
pub struct CopyProcedure {
  /// The name of the new procedure.
  pub name: String,
  /// The id of the procedure to copy.
  pub id: String,
}

#[typeshare]
pub type CopyProcedureResponse = Procedure;

//

/// Deletes the procedure at the given id, and returns the deleted procedure.
/// Response: [Procedure]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(DeleteProcedureResponse)]
pub struct DeleteProcedure {
  /// The id of the procedure to delete.
  pub id: String,
}

#[typeshare]
pub type DeleteProcedureResponse = Procedure;

//

/// Update the procedure at the given id, and return the updated procedure.
/// Response: [Procedure].
/// 
/// Note. This method updates only the fields which are set in the [_PartialProcedureConfig],
/// effectively merging diffs into the final document.
/// This is helpful when multiple users are using
/// the same resources concurrently by ensuring no unintentional
/// field changes occur from out of date local state.
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UpdateProcedureResponse)]
pub struct UpdateProcedure {
  pub id: String,
  pub config: _PartialProcedureConfig,
}

#[typeshare]
pub type UpdateProcedureResponse = Procedure;
