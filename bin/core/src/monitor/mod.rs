use async_timing_util::{wait_until_timelength, Timelength};
use futures::future::join_all;
use monitor_types::entities::{
  deployment::{ContainerSummary, DockerContainerState},
  server::{
    stats::{AllSystemStats, ServerHealth},
    Server, ServerStatus,
  },
};
use mungos::mongodb::bson::doc;
use periphery_client::requests;

use crate::state::State;

mod alert;
mod helpers;
mod record;

#[derive(Default)]
pub struct History<Curr: Default, Prev> {
  pub curr: Curr,
  pub prev: Option<Prev>,
}

#[derive(Default, Clone)]
pub struct CachedServerStatus {
  pub id: String,
  pub status: ServerStatus,
  pub version: String,
  pub stats: Option<AllSystemStats>,
  pub health: Option<ServerHealth>,
}

#[derive(Default, Clone)]
pub struct CachedDeploymentStatus {
  pub id: String,
  pub state: DockerContainerState,
  pub container: Option<ContainerSummary>,
}

impl State {
  pub async fn monitor(&self) {
    loop {
      let ts = (wait_until_timelength(Timelength::FiveSeconds, 500)
        .await
        - 500) as i64;
      let servers = self.db.servers.get_some(None, None).await;
      if let Err(e) = &servers {
        error!(
          "failed to get server list (manage status cache) | {e:#?}"
        )
      }
      let servers = servers.unwrap();
      let futures = servers.into_iter().map(|server| async move {
        self.update_cache_for_server(&server).await;
      });
      join_all(futures).await;
      tokio::join!(
        self.check_alerts(ts),
        self.record_server_stats(ts)
      );
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
      self
        .insert_server_status(
          server,
          ServerStatus::Disabled,
          String::from("unknown"),
          None,
        )
        .await;
      return;
    }
    // already handle server disabled case above, so using unwrap here
    let periphery = self.periphery_client(server).unwrap();
    let version = periphery.request(requests::GetVersion {}).await;
    if version.is_err() {
      self.insert_deployments_status_unknown(deployments).await;
      self
        .insert_server_status(
          server,
          ServerStatus::NotOk,
          String::from("unknown"),
          None,
        )
        .await;
      return;
    }
    let stats =
      periphery.request(requests::GetAllSystemStats {}).await;
    if stats.is_err() {
      self.insert_deployments_status_unknown(deployments).await;
      self
        .insert_server_status(
          server,
          ServerStatus::NotOk,
          String::from("unknown"),
          None,
        )
        .await;
      return;
    }
    let stats = stats.unwrap();
    self
      .insert_server_status(
        server,
        ServerStatus::Ok,
        version.unwrap().version,
        stats.into(),
      )
      .await;
    let containers =
      periphery.request(requests::GetContainerList {}).await;
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
      let prev = self
        .deployment_status_cache
        .get(&deployment.id)
        .await
        .map(|s| s.curr.state);
      let state = container
        .as_ref()
        .map(|c| c.state)
        .unwrap_or(DockerContainerState::NotDeployed);
      self
        .deployment_status_cache
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
}
