use async_timing_util::{wait_until_timelength, Timelength};
use monitor_client::entities::user::stack_user;
use mungos::find::find_collect;

use crate::{config::core_config, state::db_client};

pub mod json;
pub mod remote;

pub fn spawn_stack_refresh_loop() -> anyhow::Result<()> {
  let interval: Timelength =
    core_config().stack_poll_interval.try_into()?;
  tokio::spawn(async move {
    let db = db_client().await;
    let user = stack_user();
    loop {
      wait_until_timelength(interval, 3000).await;
      let Ok(stacks) =
        find_collect(&db.stacks, None, None).await.inspect_err(|e| {
          warn!(
            "failed to get stacks from db in refresh task | {e:#}"
          )
        })
      else {
        continue;
      };
      for stack in stacks {
        // State
        //   .resolve(
        //     RefreshResourceSyncPending { sync: sync.id },
        //     user.clone(),
        //   )
        //   .await
        //   .inspect_err(|e| {
        //     warn!("failed to refresh resource sync in refresh task | sync: {} | {e:#}", sync.name)
        //   })
        //   .ok();
      }
    }
  });
  Ok(())
}
