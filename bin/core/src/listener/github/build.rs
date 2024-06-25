use std::sync::OnceLock;

use anyhow::anyhow;
use axum::http::HeaderMap;
use monitor_client::{
  api::execute::RunBuild,
  entities::{build::Build, user::github_user},
};
use resolver_api::Resolve;

use crate::{
  api::execute::ExecuteRequest,
  helpers::update::init_execution_update, resource, state::State,
};

use super::{extract_branch, verify_gh_signature, ListenerLockCache};

fn build_locks() -> &'static ListenerLockCache {
  static BUILD_LOCKS: OnceLock<ListenerLockCache> = OnceLock::new();
  BUILD_LOCKS.get_or_init(Default::default)
}

pub async fn handle_build_webhook(
  build_id: String,
  headers: HeaderMap,
  body: String,
) -> anyhow::Result<()> {
  // Acquire and hold lock to make a task queue for
  // subsequent listener calls on same resource.
  // It would fail if we let it go through from action state busy.
  let lock = build_locks().get_or_insert_default(&build_id).await;
  let _lock = lock.lock().await;

  verify_gh_signature(headers, &body).await?;
  let request_branch = extract_branch(&body)?;
  let build = resource::get::<Build>(&build_id).await?;
  if !build.config.webhook_enabled {
    return Err(anyhow!("build does not have webhook enabled"));
  }
  if request_branch != build.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }
  let user = github_user().to_owned();
  let req = ExecuteRequest::RunBuild(RunBuild { build: build_id });
  let update = init_execution_update(&req, &user).await?;
  let ExecuteRequest::RunBuild(req) = req else {
    unreachable!()
  };
  State.resolve(req, (user, update)).await?;
  Ok(())
}
