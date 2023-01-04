use std::{collections::HashMap, sync::Arc};

use axum::Extension;
use db::DbClient;
use periphery::PeripheryClient;
use tokio::sync::Mutex;
use types::{BuildActionState, CoreConfig, DeploymentActionState, ServerActionState};

use crate::{monitoring::AlertStatus, ws::update::UpdateWsChannel};

pub type StateExtension = Extension<Arc<State>>;

pub type ActionStateMap<T> = Mutex<HashMap<String, T>>;

pub struct State {
    pub config: CoreConfig,
    pub db: DbClient,
    pub update: UpdateWsChannel,
    pub periphery: PeripheryClient,
    pub slack: Option<slack::Client>,
    pub build_action_states: ActionStateMap<BuildActionState>,
    pub deployment_action_states: ActionStateMap<DeploymentActionState>,
    pub server_action_states: ActionStateMap<ServerActionState>,
    pub server_alert_status: Mutex<HashMap<String, AlertStatus>>, // (server_id, AlertStatus)
}

impl State {
    pub async fn new(config: CoreConfig) -> Arc<State> {
        let state = State {
            db: DbClient::new(config.mongo.clone()).await,
            slack: config.slack_url.clone().map(|url| slack::Client::new(&url)),
            config,
            update: UpdateWsChannel::new(),
            periphery: PeripheryClient::default(),
            build_action_states: Default::default(),
            deployment_action_states: Default::default(),
            server_action_states: Default::default(),
            server_alert_status: Default::default(),
        };
        let state = Arc::new(state);
        let state_clone = state.clone();
        tokio::spawn(async move { state_clone.collect_server_stats().await });
        if state.slack.is_some() {
            let state_clone = state.clone();
            tokio::spawn(async move { state_clone.daily_update().await });
        }
        if state.config.keep_stats_for_days != 0 {
            let state_clone = state.clone();
            tokio::spawn(async move { state_clone.prune_stats_on_mongo().await });
        }
        state
    }

    pub async fn extension(config: CoreConfig) -> StateExtension {
        Extension(State::new(config).await)
    }
}
