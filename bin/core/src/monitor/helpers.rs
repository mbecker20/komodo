use monitor_client::entities::{
  deployment::{Deployment, DeploymentState},
  repo::Repo,
  server::{
    stats::{
      ServerHealth, SeverityLevel, SingleDiskUsage, SystemStats,
    },
    Server, ServerConfig, ServerState,
  },
  stack::{Stack, StackState},
};
use serror::Serror;

use crate::state::{
  deployment_status_cache, repo_status_cache, server_status_cache,
  stack_status_cache,
};

use super::{
  CachedDeploymentStatus, CachedRepoStatus, CachedServerStatus,
  CachedStackStatus, History,
};

#[instrument(level = "debug", skip_all)]
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
            state: DeploymentState::Unknown,
            container: None,
          },
          prev,
        }
        .into(),
      )
      .await;
  }
}

#[instrument(level = "debug", skip_all)]
pub async fn insert_repos_status_unknown(repos: Vec<Repo>) {
  let status_cache = repo_status_cache();
  for repo in repos {
    status_cache
      .insert(
        repo.id.clone(),
        CachedRepoStatus {
          latest_hash: None,
          latest_message: None,
        }
        .into(),
      )
      .await;
  }
}

#[instrument(level = "debug", skip_all)]
pub async fn insert_stacks_status_unknown(stacks: Vec<Stack>) {
  let status_cache = stack_status_cache();
  for stack in stacks {
    let prev =
      status_cache.get(&stack.id).await.map(|s| s.curr.state);
    status_cache
      .insert(
        stack.id.clone(),
        History {
          curr: CachedStackStatus {
            // id: stack.id,
            state: StackState::Unknown,
            services: Vec::new(),
          },
          prev,
        }
        .into(),
      )
      .await;
  }
}

#[instrument(level = "debug", skip_all)]
pub async fn insert_server_status(
  server: &Server,
  state: ServerState,
  version: String,
  stats: Option<SystemStats>,
  err: impl Into<Option<Serror>>,
) {
  let health = stats.as_ref().map(|s| get_server_health(server, s));
  server_status_cache()
    .insert(
      server.id.clone(),
      CachedServerStatus {
        id: server.id.clone(),
        state,
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
  SystemStats {
    cpu_perc,
    mem_used_gb,
    mem_total_gb,
    disks,
    ..
  }: &SystemStats,
) -> ServerHealth {
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

  for SingleDiskUsage {
    mount,
    used_gb,
    total_gb,
    ..
  } in disks
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

  health
}
