use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use async_timing_util::{wait_until_timelength, Timelength};
use axum::Extension;
use db::DbClient;
use futures_util::future::join_all;
use mungos::doc;
use periphery::PeripheryClient;
use types::{
    BuildActionState, CoreConfig, DeploymentActionState, ServerActionState, SystemStatsQuery,
    SystemStatsRecord,
};

use crate::ws::update::UpdateWsChannel;

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
        };
        let state = Arc::new(state);
        let state_clone = state.clone();
        tokio::spawn(async move { state_clone.collect_server_stats().await });
        state
    }

    pub async fn extension(config: CoreConfig) -> StateExtension {
        Extension(State::new(config).await)
    }

    pub async fn collect_server_stats(&self) {
        loop {
            let ts = wait_until_timelength(Timelength::OneMinute, 0).await as i64;
            let servers = self
                .db
                .servers
                .get_some(doc! { "enabled": true }, None)
                .await;
            if let Err(e) = servers {
                eprintln!("failed to get server list from db: {e:?}");
                continue;
            }
            let futures = servers.unwrap().into_iter().map(|server| async move {
                let stats = self
                    .periphery
                    .get_system_stats(
                        &server,
                        &SystemStatsQuery {
                            networks: true,
                            components: true,
                            processes: false,
                        },
                    )
                    .await;
                (server, stats)
            });
            for (server, res) in join_all(futures).await {
                if let Err(e) = res {
                    if let Some(slack) = &self.slack {
                        let res = slack
                            .send_message_with_header(format!(""), format!(""))
                            .await;
                    }
                    continue;
                }
                let stats = res.unwrap();
                let res = self
                    .db
                    .stats
                    .create_one(SystemStatsRecord::from_stats(server.id, ts, stats))
                    .await;
            }
        }
    }
}
