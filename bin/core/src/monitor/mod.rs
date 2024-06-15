use async_timing_util::wait_until_timelength;
use futures::future::join_all;
use monitor_client::entities::{
  deployment::{ContainerSummary, DeploymentState},
  server::{
    stats::{ServerHealth, SystemStats},
    Server, ServerState,
  },
};
use mungos::{find::find_collect, mongodb::bson::doc};
use periphery_client::api::{self, git::GetLatestCommit};
use serror::Serror;

use crate::{
  config::core_config,
  helpers::periphery_client,
  monitor::{alert::check_alerts, record::record_server_stats},
  state::{db_client, deployment_status_cache, repo_status_cache},
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
  pub id: String,
  pub state: DeploymentState,
  pub container: Option<ContainerSummary>,
}

#[derive(Default, Clone, Debug)]
pub struct CachedRepoStatus {
  pub latest_hash: Option<String>,
  pub latest_message: Option<String>,
}

pub fn spawn_monitor_loop() -> anyhow::Result<()> {
  let interval: async_timing_util::Timelength =
    core_config().monitoring_interval.try_into()?;
  tokio::spawn(async move {
    loop {
      let ts =
        (wait_until_timelength(interval, 500).await - 500) as i64;
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
  Ok(())
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
      error!("failed to get deployments list from mongo (update status cache) | server id: {} | {e:#}", server.id);
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
      error!("failed to get repos list from mongo (update status cache) | server id: {} | {e:#}", server.id);
      Vec::new()
    }
  };

  // Handle server disabled
  if !server.config.enabled {
    insert_deployments_status_unknown(deployments).await;
    insert_repos_status_unknown(repos).await;
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
    Ok(containers) => {
      let status_cache = deployment_status_cache();
      for deployment in deployments {
        let container = containers
          .iter()
          .find(|c| c.name == deployment.name)
          .cloned();
        let prev = status_cache
          .get(&deployment.id)
          .await
          .map(|s| s.curr.state);
        let state = container
          .as_ref()
          .map(|c| c.state)
          .unwrap_or(DeploymentState::NotDeployed);
        status_cache
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
    }
    Err(e) => {
      warn!("could not get containers list | {e:#}");
      insert_deployments_status_unknown(deployments).await;
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
