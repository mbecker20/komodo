use async_timing_util::{wait_until_timelength, Timelength};
use futures::future::join_all;
use monitor_types::entities::{
    deployment::{BasicContainerInfo, Deployment, DockerContainerState},
    server::{stats::AllSystemStats, Server, ServerStatus},
};
use mungos::mongodb::bson::doc;
use periphery_client::{requests, PeripheryClient};

use crate::state::State;

#[derive(Default)]
pub struct CachedServerStatus {
    pub id: String,
    pub status: ServerStatus,
    pub version: String,
    pub stats: Option<AllSystemStats>,
}

#[derive(Default)]
pub struct CachedDeploymentStatus {
    pub id: String,
    pub state: DockerContainerState,
    pub container: Option<BasicContainerInfo>,
}

impl State {
    pub async fn monitor(&self) {
        loop {
            wait_until_timelength(Timelength::FiveSeconds, 500).await;
            let servers = self.db.servers.get_some(None, None).await;
            if let Err(e) = &servers {
                error!("failed to get server list (manage status cache) | {e:#?}")
            }
            let servers = servers.unwrap();
            let futures = servers
                .into_iter()
                .map(|server| async move { self.update_cache(&server).await });
            join_all(futures).await;
        }
    }

    pub async fn update_cache(&self, server: &Server) {
        let deployments = self
            .db
            .deployments
            .get_some(doc! { "config.server_id": &server.id }, None)
            .await;
        if let Err(e) = &deployments {
            error!("failed to get deployments list from mongo (update status cache) | server id: {} | {e:#?}", server.id);
            return;
        }
        let deployments = deployments.unwrap();
        if !server.config.enabled {
            self.insert_deployments_status_unknown(deployments).await;
            self.insert_server_status(
                server,
                ServerStatus::Disabled,
                String::from("unknown"),
                None,
            )
            .await;
            return;
        }
        let periphery = PeripheryClient::new(&server.config.address, &self.config.passkey);
        let version = periphery.request(requests::GetVersion {}).await;
        if version.is_err() {
            self.insert_deployments_status_unknown(deployments).await;
            self.insert_server_status(server, ServerStatus::NotOk, String::from("unknown"), None)
                .await;
            return;
        }
        let stats = periphery.request(requests::GetAllSystemStats {}).await;
        if stats.is_err() {
            self.insert_deployments_status_unknown(deployments).await;
            self.insert_server_status(server, ServerStatus::NotOk, String::from("unknown"), None)
                .await;
            return;
        }
        self.insert_server_status(
            server,
            ServerStatus::Ok,
            version.unwrap().version,
            stats.unwrap().into(),
        )
        .await;
        let containers = periphery.request(requests::GetContainerList {}).await;
        if containers.is_err() {
            self.insert_deployments_status_unknown(deployments).await;
            return;
        }
        let containers = containers.unwrap();
        for deployment in deployments {
            let container = containers
                .iter()
                .find(|c| c.name == deployment.name)
                .cloned();
            self.deployment_status_cache
                .insert(
                    deployment.id.clone(),
                    CachedDeploymentStatus {
                        id: deployment.id,
                        state: container
                            .as_ref()
                            .map(|c| c.state)
                            .unwrap_or(DockerContainerState::NotDeployed),
                        container,
                    }
                    .into(),
                )
                .await;
        }
    }

    async fn insert_deployments_status_unknown(&self, deployments: Vec<Deployment>) {
        for deployment in deployments {
            self.deployment_status_cache
                .insert(
                    deployment.id.clone(),
                    CachedDeploymentStatus {
                        id: deployment.id,
                        state: DockerContainerState::Unknown,
                        container: None,
                    }
                    .into(),
                )
                .await;
        }
    }

    async fn insert_server_status(
        &self,
        server: &Server,
        status: ServerStatus,
        version: String,
        stats: Option<AllSystemStats>,
    ) {
        self.server_status_cache
            .insert(
                server.id.clone(),
                CachedServerStatus {
                    id: server.id.clone(),
                    status,
                    version,
                    stats,
                }
                .into(),
            )
            .await;
    }
}
