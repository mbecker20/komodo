use std::collections::HashMap;

use monitor_types::entities::{
    alert::{Alert, AlertVariant},
    deployment::DockerContainerState,
    server::ServerStatus,
};
use tokio::sync::RwLock;

use crate::state::State;

impl State {
    // called after cache update
    pub async fn alert_servers(&self) {
        let server_status = self.server_status_cache.get_list().await;

        for v in server_status {
            match v.status {
                ServerStatus::Ok => {}
                ServerStatus::NotOk => {}
                ServerStatus::Disabled => {}
            }
        }
    }

    pub async fn alert_deployments(&self) {
        let deployment_status = self.deployment_status_cache.get_list().await;

        for v in deployment_status {
            match v.curr.state {
                DockerContainerState::Running => {}
                DockerContainerState::Unknown => {}
                DockerContainerState::Exited => {}
                _ => {}
            }
        }
    }
}
