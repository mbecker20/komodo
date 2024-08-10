use async_timing_util::{wait_until_timelength, Timelength};
use monitor_client::{
  api::write::{RefreshBuildCache, RefreshRepoCache},
  entities::user::{build_user, repo_user},
};
use mungos::find::find_collect;
use resolver_api::Resolve;

use crate::{
  config::core_config,
  state::{db_client, State},
};

pub fn spawn_repo_refresh_loop() {
  let interval: Timelength = core_config()
    .repo_poll_interval
    .try_into()
    .expect("Invalid repo poll interval");
  tokio::spawn(async move {
    refresh_repos().await;
    loop {
      wait_until_timelength(interval, 1000).await;
      refresh_repos().await;
    }
  });
}

async fn refresh_repos() {
  let Ok(repos) = find_collect(&db_client().await.repos, None, None)
    .await
    .inspect_err(|e| {
      warn!("failed to get repos from db in refresh task | {e:#}")
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
        warn!("failed to refresh repo cache in refresh task | repo: {} | {e:#}", repo.name)
      })
      .ok();
  }
}
