use std::{net::SocketAddr, str::FromStr, sync::Arc};

use anyhow::Context;
use async_timing_util::{unix_timestamp_ms, wait_until_timelength, Timelength, ONE_DAY_MS};
use axum::Extension;
use monitor_types::entities::{
    build::BuildActionState,
    deployment::{DeploymentActionState, DockerContainerState},
    repo::RepoActionState,
    server::ServerActionState,
    update::UpdateListItem,
};
use mungos::mongodb::bson::doc;
use tower_http::cors::{Any, CorsLayer};

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
            update: BroadcastChannel::new(100),
            build_cancel: BroadcastChannel::new(10),
            config,
        }
        .into();

        let state_clone = state.clone();
        tokio::spawn(async move { state_clone.monitor().await });
        let state_clone = state.clone();
        tokio::spawn(async move { state_clone.prune().await });

        Ok(state)
    }

    async fn prune(&self) {
        loop {
            wait_until_timelength(Timelength::OneDay, 5000).await;
            let (stats_res, alerts_res) = tokio::join!(self.prune_stats(), self.prune_alerts());
            if let Err(e) = stats_res {
                error!("error in pruning stats | {e:#?}");
            }
            if let Err(e) = alerts_res {
                error!("error in pruning alerts | {e:#?}");
            }
        }
    }

    async fn prune_stats(&self) -> anyhow::Result<()> {
        if self.config.keep_stats_for_days == 0 {
            return Ok(());
        }
        let delete_before_ts =
            (unix_timestamp_ms() - self.config.keep_stats_for_days * ONE_DAY_MS) as i64;
        let res = self
            .db
            .stats
            .delete_many(doc! {
                "ts": { "$lt": delete_before_ts }
            })
            .await?;
        info!("deleted {} stats from db", res.deleted_count);
        Ok(())
    }

    async fn prune_alerts(&self) -> anyhow::Result<()> {
        if self.config.keep_alerts_for_days == 0 {
            return Ok(());
        }
        let delete_before_ts =
            (unix_timestamp_ms() - self.config.keep_alerts_for_days * ONE_DAY_MS) as i64;
        let res = self
            .db
            .alerts
            .delete_many(doc! {
                "ts": { "$lt": delete_before_ts }
            })
            .await?;
        info!("deleted {} alerts from db", res.deleted_count);
        Ok(())
    }

    pub fn socket_addr(&self) -> anyhow::Result<SocketAddr> {
        SocketAddr::from_str(&format!("0.0.0.0:{}", self.config.port))
            .context("failed to parse socket addr")
    }

    pub fn cors(&self) -> anyhow::Result<CorsLayer> {
        let cors = CorsLayer::new()
            .allow_origin(
                // self.config
                //     .host
                //     .parse::<HeaderValue>()
                //     .context("failed to parse host into origin")?,
                Any,
            )
            .allow_methods(Any)
            .allow_headers(Any);
        Ok(cors)
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
