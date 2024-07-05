use std::sync::{Arc, OnceLock};

use anyhow::{anyhow, Context};
use monitor_client::entities::{
  build::BuildState,
  config::core::{CoreConfig, GithubWebhookAppConfig},
  deployment::DeploymentState,
  procedure::ProcedureState,
  repo::RepoState,
  sync::ResourceSyncState,
};
use octorust::auth::{
  Credentials, InstallationTokenGenerator, JWTCredentials,
};
use tokio::sync::{Mutex, OnceCell};

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

pub fn github_client() -> Option<&'static octorust::Client> {
  static GITHUB_CLIENT: OnceLock<Option<octorust::Client>> =
    OnceLock::new();
  GITHUB_CLIENT
    .get_or_init(|| {
      let CoreConfig {
        github_webhook_app:
          GithubWebhookAppConfig {
            app_id,
            installation_id,
            pk_path,
          },
        ..
      } = core_config();
      if *app_id == 0 || *installation_id == 0 {
        return None;
      }
      let private_key = std::fs::read(pk_path)
        .context("github webhook app | failed to load private key")
        .unwrap();

      let private_key = nom_pem::decode_block(&private_key)
        .map_err(|e| anyhow!("{e:?}"))
        .context("github webhook app | failed to decode private key")
        .unwrap();

      let jwt = JWTCredentials::new(*app_id, private_key.data)
        .context(
          "github webhook app | failed to make github JWTCredentials",
        )
        .unwrap();

      let token_generator =
        InstallationTokenGenerator::new(*installation_id, jwt);

      Some(
        octorust::Client::new(
          "github-app",
          Credentials::InstallationToken(token_generator),
        )
        .context("failed to initialize github client")
        .unwrap(),
      )
    })
    .as_ref()
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

pub type ResourceSyncStateCache = Cache<String, ResourceSyncState>;

pub fn resource_sync_state_cache() -> &'static ResourceSyncStateCache
{
  static RESOURCE_SYNC_STATE_CACHE: OnceLock<ResourceSyncStateCache> =
    OnceLock::new();
  RESOURCE_SYNC_STATE_CACHE.get_or_init(Default::default)
}

pub type ResourceSyncLockCache = Cache<String, Arc<Mutex<()>>>;

pub fn resource_sync_lock_cache() -> &'static ResourceSyncLockCache {
  static RESOURCE_SYNC_LOCK_CACHE: OnceLock<ResourceSyncLockCache> =
    OnceLock::new();
  RESOURCE_SYNC_LOCK_CACHE.get_or_init(Default::default)
}
