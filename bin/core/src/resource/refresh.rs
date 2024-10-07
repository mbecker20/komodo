use async_timing_util::{wait_until_timelength, Timelength};
use komodo_client::{
  api::write::{
    RefreshBuildCache, RefreshRepoCache, RefreshResourceSyncPending,
    RefreshStackCache,
  },
  entities::user::{build_user, repo_user, stack_user, sync_user},
};
use mungos::find::find_collect;
use resolver_api::Resolve;

use crate::{
  config::core_config,
  state::{db_client, State},
};

pub fn spawn_resource_refresh_loop() {
  let interval: Timelength = core_config()
    .resource_poll_interval
    .try_into()
    .expect("Invalid resource poll interval");
  tokio::spawn(async move {
    refresh_all().await;
    loop {
      wait_until_timelength(interval, 3000).await;
      refresh_all().await;
    }
  });
}

async fn refresh_all() {
  refresh_stacks().await;
  refresh_builds().await;
  refresh_repos().await;
  refresh_syncs().await;
}

async fn refresh_stacks() {
  let Ok(stacks) = find_collect(&db_client().stacks, None, None)
    .await
    .inspect_err(|e| {
      warn!(
        "Failed to get Stacks from database in refresh task | {e:#}"
      )
    })
  else {
    return;
  };
  for stack in stacks {
    State
      .resolve(
        RefreshStackCache { stack: stack.id },
        stack_user().clone(),
      )
      .await
      .inspect_err(|e| {
        warn!("Failed to refresh Stack cache in refresh task | Stack: {} | {e:#}", stack.name)
      })
      .ok();
  }
}

async fn refresh_builds() {
  let Ok(builds) = find_collect(&db_client().builds, None, None)
    .await
    .inspect_err(|e| {
      warn!(
        "Failed to get Builds from database in refresh task | {e:#}"
      )
    })
  else {
    return;
  };
  for build in builds {
    State
      .resolve(
        RefreshBuildCache { build: build.id },
        build_user().clone(),
      )
      .await
      .inspect_err(|e| {
        warn!("Failed to refresh Build cache in refresh task | Build: {} | {e:#}", build.name)
      })
      .ok();
  }
}

async fn refresh_repos() {
  let Ok(repos) = find_collect(&db_client().repos, None, None)
    .await
    .inspect_err(|e| {
      warn!(
        "Failed to get Repos from database in refresh task | {e:#}"
      )
    })
  else {
    return;
  };
  for repo in repos {
    State
      .resolve(
        RefreshRepoCache { repo: repo.id },
        repo_user().clone(),
      )
      .await
      .inspect_err(|e| {
        warn!("Failed to refresh Repo cache in refresh task | Repo: {} | {e:#}", repo.name)
      })
      .ok();
  }
}

async fn refresh_syncs() {
  let Ok(syncs) =
    find_collect(&db_client().resource_syncs, None, None)
      .await
      .inspect_err(|e| {
        warn!(
      "failed to get resource syncs from db in refresh task | {e:#}"
    )
      })
  else {
    return;
  };
  for sync in syncs {
    if sync.config.repo.is_empty() {
      continue;
    }
    State
      .resolve(
        RefreshResourceSyncPending { sync: sync.id },
        sync_user().clone(),
      )
      .await
      .inspect_err(|e| {
        warn!("Failed to refresh ResourceSync in refresh task | Sync: {} | {e:#}", sync.name)
      })
      .ok();
  }
}
