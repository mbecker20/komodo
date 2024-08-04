use anyhow::Context;
use async_timing_util::wait_until_timelength;
use futures::future::join_all;
use helpers::insert_stacks_status_unknown;
use monitor_client::entities::{
  deployment::{ContainerSummary, DeploymentState},
  server::{
    stats::{ServerHealth, SystemStats},
    Server, ServerState,
  },
  stack::{Stack, StackState},
};
use mungos::{find::find_collect, mongodb::bson::doc};
use periphery_client::api::{self, git::GetLatestCommit};
use regex::Regex;
use serror::Serror;

use crate::{
  config::core_config,
  helpers::{periphery_client, query::get_stack_stack_from_containers},
  monitor::{alert::check_alerts, record::record_server_stats},
  resource,
  state::{
    db_client, deployment_status_cache, repo_status_cache,
    stack_status_cache,
  },
};

use self::helpers::{
  insert_deployments_status_unknown, insert_repos_status_unknown,
  insert_server_status,
};

mod alert;
mod helpers;
mod record;

#[derive(Default, Debug)]
pub struct History<Curr: Default, Prev> {
  pub curr: Curr,
  pub prev: Option<Prev>,
}

#[derive(Default, Clone, Debug)]
pub struct CachedServerStatus {
  pub id: String,
  pub state: ServerState,
  pub version: String,
  pub stats: Option<SystemStats>,
  pub health: Option<ServerHealth>,
  pub err: Option<serror::Serror>,
}

#[derive(Default, Clone, Debug)]
pub struct CachedDeploymentStatus {
  /// The deployment id
  pub id: String,
  pub state: DeploymentState,
  pub container: Option<ContainerSummary>,
}

#[derive(Default, Clone, Debug)]
pub struct CachedRepoStatus {
  pub latest_hash: Option<String>,
  pub latest_message: Option<String>,
}

#[derive(Default, Clone, Debug)]
pub struct CachedStackStatus {
  /// The stack id
  // pub id: String,
  pub state: StackState,
  /// The containers connected to the stack
  pub containers: Vec<ContainerSummary>,
}

pub fn spawn_monitor_loop() {
  let interval: async_timing_util::Timelength = core_config()
    .monitoring_interval
    .try_into()
    .expect("Invalid monitoring interval");
  tokio::spawn(async move {
    loop {
      let ts =
        (wait_until_timelength(interval, 2000).await - 500) as i64;
      let servers =
        match find_collect(&db_client().await.servers, None, None)
          .await
        {
          Ok(servers) => servers,
          Err(e) => {
            error!(
            "failed to get server list (manage status cache) | {e:#}"
          );
            continue;
          }
        };
      let futures = servers.into_iter().map(|server| async move {
        update_cache_for_server(&server).await;
      });
      join_all(futures).await;
      tokio::join!(check_alerts(ts), record_server_stats(ts));
    }
  });
}

#[instrument(level = "debug")]
pub async fn update_cache_for_server(server: &Server) {
  let deployments = match find_collect(
    &db_client().await.deployments,
    doc! { "config.server_id": &server.id },
    None,
  )
  .await
  {
    Ok(deployments) => deployments,
    Err(e) => {
      error!("failed to get deployments list from db (update status cache) | server : {} | {e:#}", server.name);
      Vec::new()
    }
  };

  let repos = match find_collect(
    &db_client().await.repos,
    doc! { "config.server_id": &server.id },
    None,
  )
  .await
  {
    Ok(repos) => repos,
    Err(e) => {
      error!("failed to get repos list from db (update status cache) | server: {} | {e:#}", server.name);
      Vec::new()
    }
  };

  let stacks = match find_collect(
    &db_client().await.stacks,
    doc! { "config.server_id": &server.id },
    None,
  )
  .await
  {
    Ok(stacks) => stacks,
    Err(e) => {
      error!("failed to get stacks list from db (update status cache) | server: {} | {e:#}", server.name);
      Vec::new()
    }
  };

  // Handle server disabled
  if !server.config.enabled {
    insert_deployments_status_unknown(deployments).await;
    insert_repos_status_unknown(repos).await;
    insert_stacks_status_unknown(stacks).await;
    insert_server_status(
      server,
      ServerState::Disabled,
      String::from("unknown"),
      None,
      None,
    )
    .await;
    return;
  }

  let Ok(periphery) = periphery_client(server) else {
    error!(
      "somehow periphery not ok to create. should not be reached."
    );
    return;
  };

  let version = match periphery.request(api::GetVersion {}).await {
    Ok(version) => version.version,
    Err(e) => {
      insert_deployments_status_unknown(deployments).await;
      insert_repos_status_unknown(repos).await;
      insert_stacks_status_unknown(stacks).await;
      insert_server_status(
        server,
        ServerState::NotOk,
        String::from("unknown"),
        None,
        Serror::from(&e),
      )
      .await;
      return;
    }
  };

  let stats = if server.config.stats_monitoring {
    match periphery.request(api::stats::GetSystemStats {}).await {
      Ok(stats) => Some(stats),
      Err(e) => {
        insert_deployments_status_unknown(deployments).await;
        insert_repos_status_unknown(repos).await;
        insert_stacks_status_unknown(stacks).await;
        insert_server_status(
          server,
          ServerState::NotOk,
          String::from("unknown"),
          None,
          Serror::from(&e),
        )
        .await;
        return;
      }
    }
  } else {
    None
  };

  insert_server_status(server, ServerState::Ok, version, stats, None)
    .await;

  match periphery.request(api::container::GetContainerList {}).await {
    Ok(mut containers) => {
      containers.sort_by(|a, b| a.name.cmp(&b.name));

      let deployment_status_cache = deployment_status_cache();
      for deployment in deployments {
        let container = containers
          .iter()
          .find(|container| container.name == deployment.name)
          .cloned();
        let prev = deployment_status_cache
          .get(&deployment.id)
          .await
          .map(|s| s.curr.state);
        let state = container
          .as_ref()
          .map(|c| c.state)
          .unwrap_or(DeploymentState::NotDeployed);
        deployment_status_cache
          .insert(
            deployment.id.clone(),
            History {
              curr: CachedDeploymentStatus {
                id: deployment.id,
                state,
                container,
              },
              prev,
            }
            .into(),
          )
          .await;
      }

      let stack_status_cache = stack_status_cache();
      for stack in stacks {
        let containers = containers
          .iter()
          .filter(|container| {
            stack.info.services.iter().any(|service| {
              if let Some(name) = &service.container_name {
                &container.name == name
              } else {
                match Regex::new(&format!(
                  "compose-{}-[0-9]*$",
                  service.service_name
                )).with_context(|| format!("failed to construct container name matching regex for service {}", service.service_name)) {
                  Ok(regex) => regex,
                  Err(e) => {
                    warn!("{e:#}");
                    return false
                  }
                }.is_match(&container.name)
              }
            })
          })
          .cloned()
          .collect::<Vec<_>>();
        let prev = stack_status_cache
          .get(&stack.id)
          .await
          .map(|s| s.curr.state);
        let status = if containers.is_empty() {
          // stack is down
          CachedStackStatus {
            // id: stack.id.clone(),
            state: StackState::Down,
            containers: Vec::new(),
          }
        } else {
          CachedStackStatus {
            state: get_stack_stack_from_containers(&containers),
            containers,
          }
        };
        stack_status_cache
          .insert(stack.id, History { curr: status, prev }.into())
          .await;
      }
    }
    Err(e) => {
      warn!("could not get containers list | {e:#}");
      insert_deployments_status_unknown(deployments).await;
      insert_stacks_status_unknown(stacks).await;
    }
  };

  let status_cache = repo_status_cache();
  for repo in repos {
    let (latest_hash, latest_message) = periphery
      .request(GetLatestCommit {
        name: repo.name.clone(),
      })
      .await
      .map(|r| (r.hash, r.message))
      .ok()
      .unzip();
    status_cache
      .insert(
        repo.id,
        CachedRepoStatus {
          latest_hash,
          latest_message,
        }
        .into(),
      )
      .await;
  }
}

#[instrument(level = "debug")]
pub async fn update_cache_for_stack(stack: &Stack) {
  if stack.config.server_id.is_empty() {
    return;
  }
  let Ok(server) = resource::get::<Server>(&stack.config.server_id)
    .await
    .inspect_err(|e| {
      warn!("Failed to get server for stack {} | {e:#}", stack.name)
    })
  else {
    return;
  };
  update_cache_for_server(&server).await;
}
