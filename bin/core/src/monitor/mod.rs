use async_timing_util::wait_until_timelength;
use futures::future::join_all;
use helpers::insert_stacks_status_unknown;
use monitor_client::entities::{
  deployment::{ContainerSummary, DeploymentState},
  server::{
    docker_image::ImageSummary,
    docker_network::DockerNetwork,
    stats::{ServerHealth, SystemStats},
    Server, ServerState,
  },
  stack::{ComposeProject, Stack, StackService, StackState},
};
use mungos::{find::find_collect, mongodb::bson::doc};
use periphery_client::api::{self, git::GetLatestCommit};
use serror::Serror;

use crate::{
  config::core_config,
  helpers::periphery_client,
  monitor::{alert::check_alerts, record::record_server_stats},
  resource,
  state::{db_client, deployment_status_cache, repo_status_cache},
};

use self::helpers::{
  insert_deployments_status_unknown, insert_repos_status_unknown,
  insert_server_status,
};

mod alert;
mod helpers;
mod lists;
mod record;
mod resources;

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
  pub containers: Option<Vec<ContainerSummary>>,
  pub networks: Option<Vec<DockerNetwork>>,
  pub images: Option<Vec<ImageSummary>>,
  pub projects: Option<Vec<ComposeProject>>,
  /// Store the error in reaching periphery
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
  /// The stack state
  pub state: StackState,
  /// The services connected to the stack
  pub services: Vec<StackService>,
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
  let (deployments, repos, stacks) = tokio::join!(
    find_collect(
      &db_client().await.deployments,
      doc! { "config.server_id": &server.id },
      None,
    ),
    find_collect(
      &db_client().await.repos,
      doc! { "config.server_id": &server.id },
      None,
    ),
    find_collect(
      &db_client().await.stacks,
      doc! { "config.server_id": &server.id },
      None,
    )
  );

  let deployments =  deployments.inspect_err(|e| error!("failed to get deployments list from db (update status cache) | server : {} | {e:#}", server.name)).unwrap_or_default();
  let repos = repos.inspect_err(|e|  error!("failed to get repos list from db (update status cache) | server: {} | {e:#}", server.name)).unwrap_or_default();
  let stacks = stacks.inspect_err(|e|  error!("failed to get stacks list from db (update status cache) | server: {} | {e:#}", server.name)).unwrap_or_default();

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
      (None, None, None, None),
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
        (None, None, None, None),
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
          (None, None, None, None),
          Serror::from(&e),
        )
        .await;
        return;
      }
    }
  } else {
    None
  };

  match lists::get_docker_lists(&periphery).await {
    Ok((containers, networks, images, projects)) => {
      tokio::join!(
        resources::update_deployment_cache(deployments, &containers),
        resources::update_stack_cache(stacks, &containers),
      );
      insert_server_status(
        server,
        ServerState::Ok,
        version,
        stats,
        (
          Some(containers.clone()),
          Some(networks),
          Some(images),
          Some(projects),
        ),
        None,
      )
      .await;
    }
    Err(e) => {
      warn!(
        "could not get docker lists | (update status cache) | {e:#}"
      );
      insert_deployments_status_unknown(deployments).await;
      insert_stacks_status_unknown(stacks).await;
      insert_server_status(
        server,
        ServerState::Ok,
        version,
        stats,
        (None, None, None, None),
        Some(e.into()),
      )
      .await;
    }
  }

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
