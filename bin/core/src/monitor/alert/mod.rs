mod deployment;
mod server;

use crate::state::State;

impl State {
    // called after cache update
    pub async fn check_alerts(&self) {
        tokio::join!(self.alert_servers(), self.alert_deployments());
    }
}
