use clap::Parser;
use derive_empty_traits::EmptyTraits;
use resolver_api::derive::Request;
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

use crate::entities::update::Update;

use super::{BatchExecutionResponse, KomodoExecuteRequest};

/// Runs the target Procedure. Response: [Update]
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(Update)]
pub struct RunProcedure {
  /// Id or name
  pub procedure: String,
}

/// Runs multiple Procedures in parallel that match pattern. Response: [BatchExecutionResponse].
#[typeshare]
#[derive(
  Debug,
  Clone,
  PartialEq,
  Serialize,
  Deserialize,
  Request,
  EmptyTraits,
  Parser,
)]
#[empty_traits(KomodoExecuteRequest)]
#[response(BatchExecutionResponse)]
pub struct BatchRunProcedure {
  /// Id or name or wildcard pattern or regex.
  /// Supports multiline and comma delineated combinations of the above.
  ///
  /// Example:
  /// ```
  /// # match all foo-* procedures
  /// foo-*
  /// # add some more
  /// extra-procedure-1, extra-procedure-2
  /// ```
  pub pattern: String,
}
