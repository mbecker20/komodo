use async_timing_util::{wait_until_timelength, Timelength};
use futures::future::join_all;
use monitor_client::entities::{
  deployment::{ContainerSummary, DockerContainerState},
  server::{
    stats::{AllSystemStats, ServerHealth},
    Server, ServerStatus,
  },
};
use mungos::{find::find_collect, mongodb::bson::doc};
use periphery_client::api;
use serror::Serror;

use crate::{
  db::db_client,
  helpers::{cache::deployment_status_cache, periphery_client},
  monitor::{alert::check_alerts, record::record_server_stats},
};

use self::helpers::{
  insert_deployments_status_unknown, insert_server_status,
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
  pub status: ServerStatus,
  pub version: String,
  pub stats: Option<AllSystemStats>,
  pub health: Option<ServerHealth>,
  pub err: Option<serror::Serror>,
}

#[derive(Default, Clone, Debug)]
pub struct CachedDeploymentStatus {
  pub id: String,
  pub state: DockerContainerState,
  pub container: Option<ContainerSummary>,
}

pub fn spawn_monitor_loop() {
  tokio::spawn(async move {
    loop {
      let ts = (wait_until_timelength(Timelength::FiveSeconds, 500)
        .await
        - 500) as i64;
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
      error!("failed to get deployments list from mongo (update status cache) | server id: {} | {e:#}", server.id);
      return;
    }
  };

  if !server.config.enabled {
    insert_deployments_status_unknown(deployments).await;
    insert_server_status(
      server,
      ServerStatus::Disabled,
      String::from("unknown"),
      None,
      None,
    )
    .await;
    return;
  }
  // already handle server disabled case above, so using unwrap here
  let periphery = periphery_client(server).unwrap();

  let version = match periphery.request(api::GetVersion {}).await {
    Ok(version) => version.version,
    Err(e) => {
      insert_deployments_status_unknown(deployments).await;
      insert_server_status(
        server,
        ServerStatus::NotOk,
        String::from("unknown"),
        None,
        Serror::from(&e),
      )
      .await;
      return;
    }
  };

  let stats = if server.config.stats_monitoring {
    match periphery.request(api::stats::GetAllSystemStats {}).await {
      Ok(stats) => Some(stats),
      Err(e) => {
        insert_deployments_status_unknown(deployments).await;
        insert_server_status(
          server,
          ServerStatus::NotOk,
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

  insert_server_status(
    server,
    ServerStatus::Ok,
    version,
    stats,
    None,
  )
  .await;

  let containers =
    periphery.request(api::container::GetContainerList {}).await;
  if containers.is_err() {
    insert_deployments_status_unknown(deployments).await;
    return;
  }

  let containers = containers.unwrap();
  let status_cache = deployment_status_cache();
  for deployment in deployments {
    let container = containers
      .iter()
      .find(|c| c.name == deployment.name)
      .cloned();
    let prev =
      status_cache.get(&deployment.id).await.map(|s| s.curr.state);
    let state = container
      .as_ref()
      .map(|c| c.state)
      .unwrap_or(DockerContainerState::NotDeployed);
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
