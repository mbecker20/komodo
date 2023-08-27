mod deployment;
mod server;

use crate::state::State;

impl State {
    // called after cache update
    pub async fn check_alerts(&self, ts: i64) {
        tokio::join!(self.alert_servers(ts), self.alert_deployments(ts));
    }
}
