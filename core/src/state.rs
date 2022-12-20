use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::Extension;
use db::DbClient;
use periphery::PeripheryClient;
use types::{BuildActionState, CoreConfig, DeploymentActionState, ServerActionState};

use crate::ws::update::UpdateWsChannel;

pub type StateExtension = Extension<Arc<State>>;

pub type ActionStateMap<T> = Mutex<HashMap<String, T>>;

pub struct State {
    pub config: CoreConfig,
    pub db: DbClient,
    pub update: UpdateWsChannel,
    pub periphery: PeripheryClient,
    pub build_action_states: ActionStateMap<BuildActionState>,
    pub deployment_action_states: ActionStateMap<DeploymentActionState>,
    pub server_action_states: ActionStateMap<ServerActionState>,
}

impl State {
    pub async fn new(config: CoreConfig) -> State {
        State {
            db: DbClient::new(config.mongo.clone()).await,
            config,
            update: UpdateWsChannel::new(),
            periphery: PeripheryClient::default(),
            build_action_states: Default::default(),
            deployment_action_states: Default::default(),
            server_action_states: Default::default(),
        }
    }

    pub async fn extension(config: CoreConfig) -> StateExtension {
        Extension(Arc::new(State::new(config).await))
    }
}
