use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::variable::Variable;

use super::MonitorWriteRequest;

/// **Admin only.** Create variable. Response: [Variable].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(CreateVariableResponse)]
pub struct CreateVariable {
  /// The name of the variable to create.
  pub name: String,
  /// The initial value of the variable. defualt: "".
  #[serde(default)]
  pub value: String,
  /// The initial value of the description. default: "".
  #[serde(default)]
  pub description: String,
}

#[typeshare]
pub type CreateVariableResponse = Variable;

//

/// **Admin only.** Update variable. Response: [Variable].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UpdateVariableValueResponse)]
pub struct UpdateVariableValue {
  /// The name of the variable to update.
  pub name: String,
  /// The value to set.
  pub value: String,
}

#[typeshare]
pub type UpdateVariableValueResponse = Variable;

//

/// **Admin only.** Update variable. Response: [Variable].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(UpdateVariableDescriptionResponse)]
pub struct UpdateVariableDescription {
  /// The name of the variable to update.
  pub name: String,
  /// The description to set.
  pub description: String,
}

#[typeshare]
pub type UpdateVariableDescriptionResponse = Variable;

//

/// **Admin only.** Delete a variable. Response: [Variable].
#[typeshare]
#[derive(
  Debug, Clone, Serialize, Deserialize, Request, EmptyTraits,
)]
#[empty_traits(MonitorWriteRequest)]
#[response(DeleteVariableResponse)]
pub struct DeleteVariable {
  pub name: String,
}

#[typeshare]
pub type DeleteVariableResponse = Variable;
