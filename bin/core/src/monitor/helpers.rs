use monitor_client::entities::{
  deployment::{Deployment, DockerContainerState},
  server::{
    stats::{
      AllSystemStats, BasicSystemStats, ServerHealth, SeverityLevel,
      SingleDiskUsage, SystemComponent,
    },
    Server, ServerConfig, ServerStatus,
  },
};
use serror::Serror;

use crate::helpers::cache::{
  deployment_status_cache, server_status_cache,
};

use super::{CachedDeploymentStatus, CachedServerStatus, History};

pub async fn insert_deployments_status_unknown(
  deployments: Vec<Deployment>,
) {
  let status_cache = deployment_status_cache();
  for deployment in deployments {
    let prev =
      status_cache.get(&deployment.id).await.map(|s| s.curr.state);
    status_cache
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
  server: &Server,
  status: ServerStatus,
  version: String,
  stats: Option<AllSystemStats>,
  err: impl Into<Option<Serror>>,
) {
  let health = stats.as_ref().map(|s| get_server_health(server, s));
  server_status_cache()
    .insert(
      server.id.clone(),
      CachedServerStatus {
        id: server.id.clone(),
        status,
        version,
        stats,
        health,
        err: err.into(),
      }
      .into(),
    )
    .await;
}

fn get_server_health(
  server: &Server,
  stats: &AllSystemStats,
) -> ServerHealth {
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
    health.cpu = SeverityLevel::Critical
  } else if cpu_perc >= cpu_warning {
    health.cpu = SeverityLevel::Warning
  }

  let mem_perc = 100.0 * mem_used_gb / mem_total_gb;
  if mem_perc >= *mem_critical {
    health.mem = SeverityLevel::Critical
  } else if mem_perc >= *mem_warning {
    health.mem = SeverityLevel::Warning
  }

  let disk_perc = 100.0 * disk_used_gb / disk_total_gb;
  if disk_perc >= *disk_critical {
    health.disk = SeverityLevel::Critical
  } else if disk_perc >= *disk_warning {
    health.disk = SeverityLevel::Warning
  }

  for SingleDiskUsage {
    mount,
    used_gb,
    total_gb,
  } in &stats.disk.disks
  {
    let perc = 100.0 * used_gb / total_gb;
    let stats_state = if perc >= *disk_critical {
      SeverityLevel::Critical
    } else if perc >= *disk_warning {
      SeverityLevel::Warning
    } else {
      SeverityLevel::Ok
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
        SeverityLevel::Critical
      } else if perc >= 0.85 {
        SeverityLevel::Warning
      } else {
        SeverityLevel::Ok
      }
    } else {
      SeverityLevel::Ok
    };
    health.temps.insert(label.clone(), stats_state);
  }

  health
}
