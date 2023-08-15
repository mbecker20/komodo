use std::{net::SocketAddr, str::FromStr, sync::Arc};

use anyhow::Context;
use axum::Extension;
use monitor_types::entities::{
    alert::{Alert, AlertVariant},
    build::BuildActionState,
    deployment::{DeploymentActionState, DockerContainerState},
    repo::RepoActionState,
    server::ServerActionState,
    update::UpdateListItem,
};

use crate::{
    auth::{GithubOauthClient, GoogleOauthClient, JwtClient},
    config::{CoreConfig, Env},
    helpers::{cache::Cache, channel::BroadcastChannel, db::DbClient},
    monitor::{CachedDeploymentStatus, CachedServerStatus, History},
};

pub type StateExtension = Extension<Arc<State>>;

pub struct State {
    pub env: Env,
    pub config: CoreConfig,
    pub db: DbClient,

    // auth
    pub jwt: JwtClient,
    pub github_auth: Option<GithubOauthClient>,
    pub google_auth: Option<GoogleOauthClient>,

    // cache
    pub action_states: ActionStates,
    pub deployment_status_cache:
        Cache<String, Arc<History<CachedDeploymentStatus, DockerContainerState>>>,
    pub server_status_cache: Cache<String, Arc<CachedServerStatus>>,
    pub alerts: Cache<(String, AlertVariant), Arc<Alert>>,

    // channels
    pub build_cancel: BroadcastChannel<String>, // build id to cancel
    pub update: BroadcastChannel<UpdateListItem>,
}

impl State {
    pub async fn load() -> anyhow::Result<Arc<State>> {
        let env = Env::load()?;
        let mut config = CoreConfig::load(&env.config_path);

        if let Some(port) = env.port {
            config.port = port;
        }

        logger::init(config.log_level.into())?;

        debug!("loading state");

        let state: Arc<_> = State {
            env,
            db: DbClient::new(&config).await?,
            jwt: JwtClient::new(&config),
            github_auth: GithubOauthClient::new(&config),
            google_auth: GoogleOauthClient::new(&config),
            action_states: Default::default(),
            deployment_status_cache: Default::default(),
            server_status_cache: Default::default(),
            alerts: Default::default(),
            update: BroadcastChannel::new(100),
            build_cancel: BroadcastChannel::new(10),
            config,
        }
        .into();

        let state_clone = state.clone();
        tokio::spawn(async move { state_clone.monitor().await });

        Ok(state)
    }

    pub fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
        SocketAddr::from_str(&format!("0.0.0.0:{}", self.config.port))
            .context("failed to parse socket addr")
    }
}

#[derive(Default)]
pub struct ActionStates {
    pub build: Cache<String, BuildActionState>,
    pub deployment: Cache<String, DeploymentActionState>,
    pub server: Cache<String, ServerActionState>,
    pub repo: Cache<String, RepoActionState>,
    // pub command: Cache<CommandActionState>,
}
