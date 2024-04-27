use std::sync::{Arc, OnceLock};

use monitor_client::entities::deployment::DockerContainerState;
use tokio::sync::OnceCell;

use crate::{
  auth::jwt::JwtClient, config::core_config, db::DbClient, helpers::{action_state::ActionStates, cache::Cache}, monitor::{CachedDeploymentStatus, CachedServerStatus, History}
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
  Arc<History<CachedDeploymentStatus, DockerContainerState>>,
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
