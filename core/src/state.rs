use std::sync::Arc;

use axum::Extension;
use db::DbClient;
use periphery::PeripheryClient;
use types::CoreConfig;

use crate::ws::update::UpdateWsChannel;

pub type StateExtension = Extension<Arc<State>>;

pub struct State {
    pub config: CoreConfig,
    pub db: DbClient,
    pub update: UpdateWsChannel,
    pub periphery: PeripheryClient,
}

impl State {
    pub async fn new(config: CoreConfig) -> State {
        State {
            db: DbClient::new(config.mongo.clone()).await,
            periphery: PeripheryClient::new(),
            update: UpdateWsChannel::new(),
            config,
        }
    }

    pub async fn extension(config: CoreConfig) -> StateExtension {
        Extension(Arc::new(State::new(config).await))
    }
}
