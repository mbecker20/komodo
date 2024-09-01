use async_timing_util::{wait_until_timelength, Timelength};
use komodo_client::{
  api::write::RefreshResourceSyncPending, entities::user::sync_user,
};
use mungos::find::find_collect;
use resolver_api::Resolve;

use crate::{
  config::core_config,
  state::{db_client, State},
};

// pub mod deployment;
pub mod deploy;
pub mod remote;
pub mod resource;
pub mod user_groups;
pub mod variables;

mod file;
mod resources;

pub fn spawn_sync_refresh_loop() {
  let interval: Timelength = core_config()
    .sync_poll_interval
    .try_into()
    .expect("Invalid sync poll interval");
  tokio::spawn(async move {
    refresh_syncs().await;
    loop {
      wait_until_timelength(interval, 0).await;
      refresh_syncs().await;
    }
  });
}

async fn refresh_syncs() {
  let Ok(syncs) =
    find_collect(&db_client().await.resource_syncs, None, None)
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
        warn!("failed to refresh resource sync in refresh task | sync: {} | {e:#}", sync.name)
      })
      .ok();
  }
}
