use std::sync::OnceLock;

use monitor_client::entities::{
  build::BuildActionState, deployment::DeploymentActionState,
  procedure::ProcedureActionState, repo::RepoActionState,
  server::ServerActionState,
};

use crate::helpers::cache::Cache;

pub struct State;

pub fn action_states() -> &'static ActionStates {
  static ACTION_STATES: OnceLock<ActionStates> = OnceLock::new();
  ACTION_STATES.get_or_init(ActionStates::default)
}

#[derive(Default)]
pub struct ActionStates {
  pub build: Cache<String, BuildActionState>,
  pub deployment: Cache<String, DeploymentActionState>,
  pub server: Cache<String, ServerActionState>,
  pub repo: Cache<String, RepoActionState>,
  pub procedure: Cache<String, ProcedureActionState>,
  // pub command: Cache<CommandActionState>,
}
