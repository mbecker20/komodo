use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::{
  deployment::TerminationSignal, update::Update,
};

use super::MonitorExecuteRequest;

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct Deploy {
  /// Name or id
  pub deployment: String,
  pub stop_signal: Option<TerminationSignal>,
  pub stop_time: Option<i32>,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct StartContainer {
  /// Name or id
  pub deployment: String,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct StopContainer {
  /// Name or id
  pub deployment: String,
  pub signal: Option<TerminationSignal>,
  pub time: Option<i32>,
}

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct StopAllContainers {
  /// Name or id
  pub server: String,
}

//

#[typeshare]
#[derive(
  Serialize, Deserialize, Debug, Clone, Request, EmptyTraits,
)]
#[empty_traits(MonitorExecuteRequest)]
#[response(Update)]
pub struct RemoveContainer {
  /// Name or id
  pub deployment: String,
  pub signal: Option<TerminationSignal>,
  pub time: Option<i32>,
}
