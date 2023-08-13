use monitor_types::entities::{
    deployment::{Deployment, DockerContainerState},
    server::{
        stats::{
            AllSystemStats, BasicSystemStats, ServerHealth, SingleDiskUsage, StatsState,
            SystemComponent,
        },
        Server, ServerConfig, ServerStatus,
    },
};

use crate::state::State;

use super::{CachedDeploymentStatus, CachedServerStatus, History};

impl State {
    pub async fn insert_deployments_status_unknown(&self, deployments: Vec<Deployment>) {
        for deployment in deployments {
            let prev = self
                .deployment_status_cache
                .get(&deployment.id)
                .await
                .map(|s| s.curr.state);
            self.deployment_status_cache
                .insert(
                    deployment.id.clone(),
                    History {
                        curr: CachedDeploymentStatus {
                            id: deployment.id,
                            state: DockerContainerState::Unknown,
                            container: None,
                        },
                        prev,
                    }
                    .into(),
                )
                .await;
        }
    }

    pub async fn insert_server_status(
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
