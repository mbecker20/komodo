use std::sync::OnceLock;

use anyhow::anyhow;
use axum::http::HeaderMap;
use monitor_client::{
  api::{execute::DeployStack, write::RefreshStackCache},
  entities::{stack::Stack, user::git_webhook_user},
};
use resolver_api::Resolve;

use crate::{
  api::execute::ExecuteRequest,
  helpers::update::init_execution_update, resource, state::State,
};

use super::{extract_branch, verify_gh_signature, ListenerLockCache};

fn stack_locks() -> &'static ListenerLockCache {
  static STACK_LOCKS: OnceLock<ListenerLockCache> = OnceLock::new();
  STACK_LOCKS.get_or_init(Default::default)
}

pub async fn handle_stack_refresh_webhook(
  stack_id: String,
  headers: HeaderMap,
  body: String,
) -> anyhow::Result<()> {
  // Acquire and hold lock to make a task queue for
  // subsequent listener calls on same resource.
  // It would fail if we let it go through, from "action state busy".
  let lock = stack_locks().get_or_insert_default(&stack_id).await;
  let _lock = lock.lock().await;

  let stack = resource::get::<Stack>(&stack_id).await?;

  verify_gh_signature(headers, &body, &stack.config.webhook_secret)
    .await?;

  if !stack.config.webhook_enabled {
    return Err(anyhow!("stack does not have webhook enabled"));
  }

  let request_branch = extract_branch(&body)?;
  if request_branch != stack.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }

  let user = git_webhook_user().to_owned();
  State
    .resolve(RefreshStackCache { stack: stack.id }, user)
    .await?;
  Ok(())
}

pub async fn handle_stack_deploy_webhook(
  stack_id: String,
  headers: HeaderMap,
  body: String,
) -> anyhow::Result<()> {
  // Acquire and hold lock to make a task queue for
  // subsequent listener calls on same resource.
  // It would fail if we let it go through from action state busy.
  let lock = stack_locks().get_or_insert_default(&stack_id).await;
  let _lock = lock.lock().await;

  let stack = resource::get::<Stack>(&stack_id).await?;

  verify_gh_signature(headers, &body, &stack.config.webhook_secret)
    .await?;

  if !stack.config.webhook_enabled {
    return Err(anyhow!("stack does not have webhook enabled"));
  }

  let request_branch = extract_branch(&body)?;
  if request_branch != stack.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }

  let user = git_webhook_user().to_owned();
  let req = ExecuteRequest::DeployStack(DeployStack {
    stack: stack_id,
    stop_time: None,
  });
  let update = init_execution_update(&req, &user).await?;
  let ExecuteRequest::DeployStack(req) = req else {
    unreachable!()
  };
  State.resolve(req, (user, update)).await?;
  Ok(())
}
