use anyhow::Context;
use async_timing_util::{wait_until_timelength, Timelength};
use futures::future::join_all;
use monitor_types::entities::{
    alerter::Alerter,
    deployment::{ContainerSummary, Deployment, DockerContainerState},
    server::{
        stats::{
            AllSystemStats, BasicSystemStats, ServerHealth, SingleDiskUsage, StatsState,
            SystemComponent, SystemStatsRecord,
        },
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
    pub container: Option<ContainerSummary>,
}

impl State {
    pub async fn monitor(&self) {
        loop {
            let ts = (wait_until_timelength(Timelength::FiveSeconds, 500).await - 500) as i64;
            let servers = self.db.servers.get_some(None, None).await;
            if let Err(e) = &servers {
                error!("failed to get server list (manage status cache) | {e:#?}")
            }
            let servers = servers.unwrap();
            let futures = servers.into_iter().map(|server| async move {
                self.update_cache_for_server(&server).await;
            });
            join_all(futures).await;
            self.record_server_stats(ts).await;
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
        let prev_server_status = self.server_status_cache.get(&server.id).await;
        let periphery = self.periphery_client(server);
        let version = periphery.request(requests::GetVersion {}).await;
        if version.is_err() {
            self.insert_deployments_status_unknown(deployments).await;
            self.insert_server_status(server, ServerStatus::NotOk, String::from("unknown"), None)
                .await;
            let alerters = self.db.alerters.get_some(None, None).await;
            if let Err(e) = &alerters {
                error!("failed to get alerters from db | {e:#?}");
            }
            let alerters = alerters.unwrap();
            self.handle_server_unreachable(server, &alerters).await;
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
        tokio::join!(
            self.handle_server_stats(server, stats.clone()),
            self.insert_server_status(
                server,
                ServerStatus::Ok,
                version.unwrap().version,
                stats.into(),
            )
        );
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

    async fn record_server_stats(&self, ts: i64) {
        let status = self.server_status_cache.get_list().await;
        let records = status
            .into_iter()
            .filter(|status| status.stats.is_some())
            .map(|status| {
                let BasicSystemStats {
                    system_load,
                    cpu_perc,
                    cpu_freq_mhz,
                    mem_total_gb,
                    mem_used_gb,
                    disk_total_gb,
                    disk_used_gb,
                    ..
                } = status.stats.as_ref().unwrap().basic;
                SystemStatsRecord {
                    ts,
                    sid: status.id.clone(),
                    system_load,
                    cpu_perc,
                    cpu_freq_mhz,
                    mem_total_gb,
                    mem_used_gb,
                    disk_total_gb,
                    disk_used_gb,
                }
            })
            .collect::<Vec<_>>();
        let res = self.db.stats.create_many(records).await;
        if let Err(e) = res {
            error!("failed to record server stats | {e:#?}");
        }
    }

    async fn handle_server_unreachable(&self, server: &Server, alerters: &[Alerter]) {
        let inner = || async { anyhow::Ok(()) };

        let res = inner().await.context("failed to handle server unreachable");

        if let Err(e) = res {
            error!("{e:#?}");
        }
    }

    async fn handle_server_rereachable(&self, server: &Server, alerters: &[Alerter]) {
        let inner = || async { anyhow::Ok(()) };

        let res = inner().await.context("failed to handle server rereachable");

        if let Err(e) = res {
            error!("{e:#?}");
        }
    }

    async fn handle_server_stats(&self, server: &Server, stats: AllSystemStats) {
        let inner = || async {
            let health = get_server_health(server, &stats);

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

    for SingleDiskUsage {
        mount,
        used_gb,
        total_gb,
    } in &stats.disk.disks
    {
        let perc = 100.0 * used_gb / total_gb;
        let stats_state = if perc >= *disk_critical {
            StatsState::Critical
        } else if perc >= *disk_warning {
            StatsState::Warning
        } else {
            StatsState::Ok
        };
        health.disks.insert(mount.clone(), stats_state);
    }

    for SystemComponent {
        label,
        temp,
        critical,
        ..
    } in &stats.components
    {
        let stats_state = if let Some(critical) = critical {
            let perc = temp / critical;
            if perc >= 0.95 {
                StatsState::Critical
            } else if perc >= 0.85 {
                StatsState::Warning
            } else {
                StatsState::Ok
            }
        } else {
            StatsState::Ok
        };
        health.temps.insert(label.clone(), stats_state);
    }

    health
}
