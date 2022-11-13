use types::{Server, SystemStats};

use crate::PeripheryClient;

impl PeripheryClient {
    pub async fn get_system_stats(&self, server: &Server) -> anyhow::Result<SystemStats> {
        self.get_json(server, "/stats/system").await
    }

    pub async fn get_docker_stats(&self, server: &Server) -> anyhow::Result<SystemStats> {
        self.get_json(server, "/stats/docker").await
    }
}
