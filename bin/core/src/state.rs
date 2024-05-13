use std::sync::{Arc, OnceLock};

use monitor_client::entities::{
  build::BuildState, deployment::DeploymentState,
  procedure::ProcedureState, repo::RepoState,
};
use tokio::sync::OnceCell;

use crate::{
  auth::jwt::JwtClient,
  config::core_config,
  db::DbClient,
  helpers::{action_state::ActionStates, cache::Cache},
  monitor::{
    CachedDeploymentStatus, CachedRepoStatus, CachedServerStatus,
    History,
  },
};

pub struct State;

pub async fn db_client() -> &'static DbClient {
  static DB_CLIENT: OnceCell<DbClient> = OnceCell::const_new();
  DB_CLIENT
    .get_or_init(|| async {
      DbClient::new(&core_config().mongo)
        .await
        .expect("failed to initialize mongo client")
    })
    .await
}

pub fn jwt_client() -> &'static JwtClient {
  static JWT_CLIENT: OnceLock<JwtClient> = OnceLock::new();
  JWT_CLIENT.get_or_init(|| JwtClient::new(core_config()))
}

pub fn action_states() -> &'static ActionStates {
  static ACTION_STATES: OnceLock<ActionStates> = OnceLock::new();
  ACTION_STATES.get_or_init(ActionStates::default)
}

pub type DeploymentStatusCache = Cache<
  String,
  Arc<History<CachedDeploymentStatus, DeploymentState>>,
>;

pub fn deployment_status_cache() -> &'static DeploymentStatusCache {
  static DEPLOYMENT_STATUS_CACHE: OnceLock<DeploymentStatusCache> =
    OnceLock::new();
  DEPLOYMENT_STATUS_CACHE.get_or_init(Default::default)
}

pub type ServerStatusCache = Cache<String, Arc<CachedServerStatus>>;

pub fn server_status_cache() -> &'static ServerStatusCache {
  static SERVER_STATUS_CACHE: OnceLock<ServerStatusCache> =
    OnceLock::new();
  SERVER_STATUS_CACHE.get_or_init(Default::default)
}

pub type RepoStatusCache = Cache<String, Arc<CachedRepoStatus>>;

pub fn repo_status_cache() -> &'static RepoStatusCache {
  static REPO_STATUS_CACHE: OnceLock<RepoStatusCache> =
    OnceLock::new();
  REPO_STATUS_CACHE.get_or_init(Default::default)
}

pub type BuildStateCache = Cache<String, BuildState>;

pub fn build_state_cache() -> &'static BuildStateCache {
  static BUILD_STATE_CACHE: OnceLock<BuildStateCache> =
    OnceLock::new();
  BUILD_STATE_CACHE.get_or_init(Default::default)
}

pub type RepoStateCache = Cache<String, RepoState>;

pub fn repo_state_cache() -> &'static RepoStateCache {
  static REPO_STATE_CACHE: OnceLock<RepoStateCache> = OnceLock::new();
  REPO_STATE_CACHE.get_or_init(Default::default)
}

pub type ProcedureStateCache = Cache<String, ProcedureState>;

pub fn procedure_state_cache() -> &'static ProcedureStateCache {
  static PROCEDURE_STATE_CACHE: OnceLock<ProcedureStateCache> =
    OnceLock::new();
  PROCEDURE_STATE_CACHE.get_or_init(Default::default)
}
