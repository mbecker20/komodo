use std::sync::OnceLock;

use anyhow::anyhow;
use axum::http::HeaderMap;
use monitor_client::{
  api::{execute::RunSync, write::RefreshResourceSyncPending},
  entities::{sync::ResourceSync, user::git_webhook_user},
};
use resolver_api::Resolve;

use crate::{
  api::execute::ExecuteRequest,
  helpers::update::init_execution_update, resource, state::State,
};

use super::{extract_branch, verify_gh_signature, ListenerLockCache};

fn sync_locks() -> &'static ListenerLockCache {
  static SYNC_LOCKS: OnceLock<ListenerLockCache> = OnceLock::new();
  SYNC_LOCKS.get_or_init(Default::default)
}

pub async fn handle_sync_refresh_webhook(
  sync_id: String,
  headers: HeaderMap,
  body: String,
) -> anyhow::Result<()> {
  // Acquire and hold lock to make a task queue for
  // subsequent listener calls on same resource.
  // It would fail if we let it go through from action state busy.
  let lock = sync_locks().get_or_insert_default(&sync_id).await;
  let _lock = lock.lock().await;

  verify_gh_signature(headers, &body).await?;
  let request_branch = extract_branch(&body)?;
  let sync = resource::get::<ResourceSync>(&sync_id).await?;
  if !sync.config.webhook_enabled {
    return Err(anyhow!("sync does not have webhook enabled"));
  }
  if request_branch != sync.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }
  let user = git_webhook_user().to_owned();
  State
    .resolve(RefreshResourceSyncPending { sync: sync_id }, user)
    .await?;
  Ok(())
}

pub async fn handle_sync_execute_webhook(
  sync_id: String,
  headers: HeaderMap,
  body: String,
) -> anyhow::Result<()> {
  // Acquire and hold lock to make a task queue for
  // subsequent listener calls on same resource.
  // It would fail if we let it go through from action state busy.
  let lock = sync_locks().get_or_insert_default(&sync_id).await;
  let _lock = lock.lock().await;

  verify_gh_signature(headers, &body).await?;
  let request_branch = extract_branch(&body)?;
  let sync = resource::get::<ResourceSync>(&sync_id).await?;
  if !sync.config.webhook_enabled {
    return Err(anyhow!("sync does not have webhook enabled"));
  }
  if request_branch != sync.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }
  let user = git_webhook_user().to_owned();
  let req = ExecuteRequest::RunSync(RunSync { sync: sync_id });
  let update = init_execution_update(&req, &user).await?;
  let ExecuteRequest::RunSync(req) = req else {
    unreachable!()
  };
  State.resolve(req, (user, update)).await?;
  Ok(())
}
