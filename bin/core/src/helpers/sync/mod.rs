use async_timing_util::{wait_until_timelength, Timelength};
use monitor_client::{
  api::write::RefreshResourceSyncPending, entities::user::sync_user,
};
use mungos::find::find_collect;
use resolver_api::Resolve;

use crate::state::{db_client, State};

pub mod remote;
pub mod resource;
pub mod user_groups;
pub mod variables;

mod file;
mod resources;

pub fn spawn_sync_refresh_loop() {
  tokio::spawn(async move {
    let db = db_client().await;
    let user = sync_user();
    loop {
      wait_until_timelength(Timelength::FiveMinutes, 0).await;
      let Ok(syncs) = find_collect(&db.resource_syncs, None, None)
        .await
        .inspect_err(|e| warn!("failed to get resource syncs from db in refresh task | {e:#}")) else {
          continue;
        };
      for sync in syncs {
        State
          .resolve(
            RefreshResourceSyncPending { sync: sync.id },
            user.clone(),
          )
          .await
          .inspect_err(|e| {
            warn!("failed to refresh resource sync in refresh task | sync: {} | {e:#}", sync.name)
          })
          .ok();
      }
    }
  });
}

fn muted(content: &str) -> String {
  format!("<span class=\"text-muted-foreground\">{content}</span>")
}

fn bold(content: &str) -> String {
  format!("<span class=\"font-bold\">{content}</span>")
}

pub fn colored(content: &str, color: &str) -> String {
  format!("<span class=\"text-{color}-500\">{content}</span>")
}
