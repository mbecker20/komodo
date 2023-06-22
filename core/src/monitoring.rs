use anyhow::Context;
use async_timing_util::{wait_until_timelength, Timelength};
use futures::future::join_all;
use monitor_types::entities::{
    deployment::{BasicContainerInfo, Deployment, DockerContainerState},
    server::{
        stats::{AllSystemStats, BasicSystemStats, CpuUsage, ServerHealth, StatsState},
        Server, ServerConfig, ServerStatus,
    },
};
use mungos::mongodb::bson::doc;
use periphery_client::requests;

use crate::state::State;

#[derive(Default)]
pub struct CachedServerStatus {
    pub id: String,
    pub status: ServerStatus,
    pub version: String,
    pub stats: Option<AllSystemStats>,
    pub health: Option<ServerHealth>,
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
                .map(|server| async move { self.update_cache_for_server(&server).await });
            join_all(futures).await;
        }
    }

    pub async fn update_cache_for_server(&self, server: &Server) {
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
        let prev_state = self.server_status_cache.get(&server.id).await;
        let periphery = self.periphery_client(server);
        let version = periphery.request(requests::GetVersion {}).await;
        if version.is_err() {
            self.insert_deployments_status_unknown(deployments).await;
            self.insert_server_status(server, ServerStatus::NotOk, String::from("unknown"), None)
                .await;
            self.handle_server_unreachable(server).await;
            return;
        }
        let stats = periphery.request(requests::GetAllSystemStats {}).await;
        if stats.is_err() {
            self.insert_deployments_status_unknown(deployments).await;
            self.insert_server_status(server, ServerStatus::NotOk, String::from("unknown"), None)
                .await;
            return;
        }
        let stats = stats.unwrap();
        self.handle_server_stats(server, &stats).await;
        self.insert_server_status(
            server,
            ServerStatus::Ok,
            version.unwrap().version,
            stats.into(),
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
            let prev_state = self
                .deployment_status_cache
                .get(&deployment.id)
                .await
                .map(|s| s.state);
            let state = container
                .as_ref()
                .map(|c| c.state)
                .unwrap_or(DockerContainerState::NotDeployed);

            self.handle_deployment_state_change(&deployment, state, prev_state)
                .await;

            self.deployment_status_cache
                .insert(
                    deployment.id.clone(),
                    CachedDeploymentStatus {
                        id: deployment.id,
                        state,
                        container,
                    }
                    .into(),
                )
                .await;
        }
    }

    async fn handle_server_unreachable(&self, server: &Server) {
        let inner = || async { anyhow::Ok(()) };

        let res = inner().await.context("failed to handle server unreachable");

        if let Err(e) = res {
            error!("{e:#?}");
        }
    }

    async fn handle_server_rereachable(&self, server: &Server) {
        let inner = || async { anyhow::Ok(()) };

        let res = inner().await.context("failed to handle server rereachable");

        if let Err(e) = res {
            error!("{e:#?}");
        }
    }

    async fn handle_server_stats(&self, server: &Server, stats: &AllSystemStats) {
        let inner = || async {
            let health = get_server_health(server, stats);

            anyhow::Ok(())
        };

        let res = inner().await.context("failed to handle server stats");

        if let Err(e) = res {
            error!("{e:#?}");
        }
    }

    async fn handle_deployment_state_change(
        &self,
        deployment: &Deployment,
        state: DockerContainerState,
        prev_state: Option<DockerContainerState>,
    ) {
        if prev_state.is_none() {
            return;
        }

        let prev_state = prev_state.unwrap();

        if state == prev_state {
            return;
        }

        let inner = || async { anyhow::Ok(()) };

        let res = inner()
            .await
            .context("failed to handle deployment state change");

        if let Err(e) = res {
            error!("{e:#?}");
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
        let health = stats.as_ref().map(|s| get_server_health(server, s));
        self.server_status_cache
            .insert(
                server.id.clone(),
                CachedServerStatus {
                    id: server.id.clone(),
                    status,
                    version,
                    stats,
                    health,
                }
                .into(),
            )
            .await;
    }
}

fn get_server_health(server: &Server, stats: &AllSystemStats) -> ServerHealth {
    let BasicSystemStats {
        cpu_perc,
        mem_used_gb,
        mem_total_gb,
        disk_used_gb,
        disk_total_gb,
        ..
    } = &stats.basic;
    let ServerConfig {
        cpu_warning,
        cpu_critical,
        mem_warning,
        mem_critical,
        disk_warning,
        disk_critical,
        ..
    } = &server.config;
    let mut health = ServerHealth::default();

    if cpu_perc >= cpu_critical {
        health.cpu = StatsState::Critical
    } else if cpu_perc >= cpu_warning {
        health.cpu = StatsState::Warning
    }

    let mem_perc = 100.0 * mem_used_gb / mem_total_gb;
    if mem_perc >= *mem_critical {
        health.mem = StatsState::Critical
    } else if mem_perc >= *mem_warning {
        health.mem = StatsState::Warning
    }

    let disk_perc = 100.0 * disk_used_gb / disk_total_gb;
    if disk_perc >= *disk_critical {
        health.disk = StatsState::Critical
    } else if disk_perc >= *disk_warning {
        health.disk = StatsState::Warning
    }

    for disk in &stats.disk.disks {
        let perc = 100.0 * disk.used_gb / disk.total_gb;
        if perc >= *disk_critical {
            health
                .disks
                .insert(disk.mount.clone(), StatsState::Critical);
        } else if perc >= *disk_warning {
            health.disks.insert(disk.mount.clone(), StatsState::Warning);
        } else {
            health.disks.insert(disk.mount.clone(), StatsState::Ok);
        }
    }

    health
}
