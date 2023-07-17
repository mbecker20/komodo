use std::{net::SocketAddr, str::FromStr, sync::Arc};

use anyhow::Context;
use axum::Extension;
use monitor_types::{
    entities::{
        build::BuildActionState, deployment::DeploymentActionState, repo::RepoActionState,
        server::ServerActionState, update::Update,
    },
    requests::auth::GetLoginOptionsResponse,
};

use crate::{
    auth::{GithubOauthClient, GoogleOauthClient, JwtClient},
    config::{CoreConfig, Env},
    helpers::{cache::Cache, channel::BroadcastChannel, db::DbClient},
    monitor::{CachedDeploymentStatus, CachedServerStatus},
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
    pub deployment_status_cache: Cache<Arc<CachedDeploymentStatus>>,
    pub server_status_cache: Cache<Arc<CachedServerStatus>>,

    // cached responses
    pub login_options_response: String,

    // channels
    pub build_cancel: BroadcastChannel<String>, // build id to cancel
    pub update: BroadcastChannel<Update>,
}

impl State {
    pub async fn load() -> anyhow::Result<Arc<State>> {
        let env = Env::load()?;
        let config = CoreConfig::load(&env.config_path);

        logger::init(config.log_level.into())?;

        debug!("loading state");

        let state: Arc<State> = State {
            env,
            db: DbClient::new(&config).await?,
            jwt: JwtClient::new(&config),
            github_auth: GithubOauthClient::new(&config),
            google_auth: GoogleOauthClient::new(&config),
            login_options_response: login_options_response(&config)?,
            action_states: Default::default(),
            deployment_status_cache: Default::default(),
            server_status_cache: Default::default(),
            update: BroadcastChannel::new(100),
            build_cancel: BroadcastChannel::new(10),
            config,
        }
        .into();

        // let state_clone = state.clone();
        // tokio::spawn(async move { state_clone.monitor().await });

        Ok(state)
    }

    pub fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
        SocketAddr::from_str(&format!("0.0.0.0:{}", self.config.port))
            .context("failed to parse socket addr")
    }
}

pub fn login_options_response(config: &CoreConfig) -> anyhow::Result<String> {
    let options = GetLoginOptionsResponse {
        local: config.local_auth,
        github: config.github_oauth.enabled
            && !config.github_oauth.id.is_empty()
            && !config.github_oauth.secret.is_empty(),
        google: config.google_oauth.enabled
            && !config.google_oauth.id.is_empty()
            && !config.google_oauth.secret.is_empty(),
    };
    serde_json::to_string(&options).context("failed to serialize login options")
}

#[derive(Default)]
pub struct ActionStates {
    pub build: Cache<BuildActionState>,
    pub deployment: Cache<DeploymentActionState>,
    pub server: Cache<ServerActionState>,
    pub repo: Cache<RepoActionState>,
    // pub command: Cache<CommandActionState>,
}
