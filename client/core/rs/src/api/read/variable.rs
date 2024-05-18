use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::variable::Variable;

use super::MonitorReadRequest;

/// List all available global variables.
/// Response: [Variable]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(GetVariableResponse)]
pub struct GetVariable {
  /// The name of the variable to get.
  pub name: String,
}

#[typeshare]
pub type GetVariableResponse = Variable;

//

/// List all available global variables.
/// Response: [ListVariablesResponse]
#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorReadRequest)]
#[response(ListVariablesResponse)]
pub struct ListVariables {}

/// The response of [ListVariables].
#[typeshare]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListVariablesResponse {
  /// The available global variables.
  pub variables: Vec<Variable>,
  /// The available global secret keys
  pub secrets: Vec<String>,
}
