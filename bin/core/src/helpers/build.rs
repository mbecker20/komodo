use async_timing_util::{wait_until_timelength, Timelength};
use monitor_client::{
  api::write::RefreshBuildCache, entities::user::build_user,
};
use mungos::find::find_collect;
use resolver_api::Resolve;

use crate::{
  config::core_config,
  state::{db_client, State},
};

pub fn spawn_build_refresh_loop() {
  let interval: Timelength = core_config()
    .build_poll_interval
    .try_into()
    .expect("Invalid build poll interval");
  tokio::spawn(async move {
    refresh_builds().await;
    loop {
      wait_until_timelength(interval, 2000).await;
      refresh_builds().await;
    }
  });
}

async fn refresh_builds() {
  let Ok(builds) =
    find_collect(&db_client().await.builds, None, None)
      .await
      .inspect_err(|e| {
        warn!("failed to get builds from db in refresh task | {e:#}")
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
        warn!("failed to refresh build cache in refresh task | build: {} | {e:#}", build.name)
      })
      .ok();
  }
}
