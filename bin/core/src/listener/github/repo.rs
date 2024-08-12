use std::sync::OnceLock;

use anyhow::anyhow;
use axum::http::HeaderMap;
use monitor_client::{
  api::execute::{BuildRepo, CloneRepo, PullRepo},
  entities::{repo::Repo, user::git_webhook_user},
};
use resolver_api::Resolve;

use crate::{
  helpers::update::init_execution_update, resource, state::State,
};

use super::{extract_branch, verify_gh_signature, ListenerLockCache};

fn repo_locks() -> &'static ListenerLockCache {
  static REPO_LOCKS: OnceLock<ListenerLockCache> = OnceLock::new();
  REPO_LOCKS.get_or_init(Default::default)
}

pub async fn handle_repo_clone_webhook(
  repo_id: String,
  headers: HeaderMap,
  body: String,
) -> anyhow::Result<()> {
  // Acquire and hold lock to make a task queue for
  // subsequent listener calls on same resource.
  // It would fail if we let it go through from action state busy.
  let lock = repo_locks().get_or_insert_default(&repo_id).await;
  let _lock = lock.lock().await;

  let repo = resource::get::<Repo>(&repo_id).await?;

  verify_gh_signature(headers, &body, &repo.config.webhook_secret)
    .await?;

  if !repo.config.webhook_enabled {
    return Err(anyhow!("repo does not have webhook enabled"));
  }

  let request_branch = extract_branch(&body)?;
  if request_branch != repo.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }

  let user = git_webhook_user().to_owned();
  let req =
    crate::api::execute::ExecuteRequest::CloneRepo(CloneRepo {
      repo: repo_id,
    });
  let update = init_execution_update(&req, &user).await?;
  let crate::api::execute::ExecuteRequest::CloneRepo(req) = req
  else {
    unreachable!()
  };
  State.resolve(req, (user, update)).await?;
  Ok(())
}

pub async fn handle_repo_pull_webhook(
  repo_id: String,
  headers: HeaderMap,
  body: String,
) -> anyhow::Result<()> {
  // Acquire and hold lock to make a task queue for
  // subsequent listener calls on same resource.
  // It would fail if we let it go through from action state busy.
  let lock = repo_locks().get_or_insert_default(&repo_id).await;
  let _lock = lock.lock().await;

  let repo = resource::get::<Repo>(&repo_id).await?;

  verify_gh_signature(headers, &body, &repo.config.webhook_secret)
    .await?;

  if !repo.config.webhook_enabled {
    return Err(anyhow!("repo does not have webhook enabled"));
  }

  let request_branch = extract_branch(&body)?;
  if request_branch != repo.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }

  let user = git_webhook_user().to_owned();
  let req = crate::api::execute::ExecuteRequest::PullRepo(PullRepo {
    repo: repo_id,
  });
  let update = init_execution_update(&req, &user).await?;
  let crate::api::execute::ExecuteRequest::PullRepo(req) = req else {
    unreachable!()
  };
  State.resolve(req, (user, update)).await?;
  Ok(())
}

pub async fn handle_repo_build_webhook(
  repo_id: String,
  headers: HeaderMap,
  body: String,
) -> anyhow::Result<()> {
  // Acquire and hold lock to make a task queue for
  // subsequent listener calls on same resource.
  // It would fail if we let it go through from action state busy.
  let lock = repo_locks().get_or_insert_default(&repo_id).await;
  let _lock = lock.lock().await;

  let repo = resource::get::<Repo>(&repo_id).await?;

  verify_gh_signature(headers, &body, &repo.config.webhook_secret)
    .await?;

  if !repo.config.webhook_enabled {
    return Err(anyhow!("repo does not have webhook enabled"));
  }

  let request_branch = extract_branch(&body)?;
  if request_branch != repo.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }

  let user = git_webhook_user().to_owned();
  let req =
    crate::api::execute::ExecuteRequest::BuildRepo(BuildRepo {
      repo: repo_id,
    });
  let update = init_execution_update(&req, &user).await?;
  let crate::api::execute::ExecuteRequest::BuildRepo(req) = req
  else {
    unreachable!()
  };
  State.resolve(req, (user, update)).await?;
  Ok(())
}
