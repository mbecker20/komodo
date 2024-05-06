use std::sync::{Arc, OnceLock};

use anyhow::{anyhow, Context};
use axum::{extract::Path, http::HeaderMap, routing::post, Router};
use hex::ToHex;
use hmac::{Hmac, Mac};
use monitor_client::{
  api::execute,
  entities::{build::Build, repo::Repo, user::github_user},
};
use resolver_api::Resolve;
use serde::Deserialize;
use sha2::Sha256;
use tokio::sync::Mutex;
use tracing::Instrument;

use crate::{
  config::core_config,
  helpers::{cache::Cache, random_duration, resource::StateResource},
  state::State,
};

type HmacSha256 = Hmac<Sha256>;

#[derive(Deserialize)]
struct Id {
  id: String,
}

#[derive(Deserialize)]
struct IdBranch {
  id: String,
  branch: String,
}

pub fn router() -> Router {
  Router::new()
		.route(
			"/build/:id",
			post(
				|Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
            let span = info_span!("build_webhook", id);
            async {
              let res = handle_build_webhook(id.clone(), headers, body).await;
              if let Err(e) = res {
                warn!("failed to run build webook for build {id} | {e:#}");
              }
            }
              .instrument(span)
              .await
					});
				},
			),
		)
		.route(
			"/repo/:id/clone", 
			post(
				|Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
						let span = info_span!("repo_clone_webhook", id);
            async {
              let res = handle_repo_clone_webhook(id.clone(), headers, body).await;
              if let Err(e) = res {
                warn!("failed to run repo clone webook for repo {id} | {e:#}");
              }
            }
              .instrument(span)
              .await
					});
				},
			)
		)
		.route(
			"/repo/:id/pull", 
			post(
				|Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
            let span = info_span!("repo_pull_webhook", id);
            async {
              let res = handle_repo_pull_webhook(id.clone(), headers, body).await;
              if let Err(e) = res {
                warn!("failed to run repo pull webook for repo {id} | {e:#}");
              }
            }
              .instrument(span)
              .await
					});
				},
			)
		)
    .route(
			"/procedure/:id/:branch", 
			post(
				|Path(IdBranch { id, branch }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
            let span = info_span!("procedure_webhook", id, branch);
            async {
              let res = handle_procedure_webhook(
                id.clone(),
                branch,
                headers,
                body
              ).await;
              if let Err(e) = res {
                warn!("failed to run procedure webook for procedure {id} | {e:#}");
              }
            }
              .instrument(span)
              .await
					});
				},
			)
		)
}

async fn handle_build_webhook(
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
  let build = Build::get_resource(&build_id).await?;
  if request_branch != build.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }
  State
    .resolve(
      execute::RunBuild { build: build_id },
      github_user().to_owned(),
    )
    .await?;
  Ok(())
}

async fn handle_repo_clone_webhook(
  repo_id: String,
  headers: HeaderMap,
  body: String,
) -> anyhow::Result<()> {
  // Acquire and hold lock to make a task queue for
  // subsequent listener calls on same resource.
  // It would fail if we let it go through from action state busy.
  let lock = repo_locks().get_or_insert_default(&repo_id).await;
  let _lock = lock.lock().await;

  verify_gh_signature(headers, &body).await?;
  let request_branch = extract_branch(&body)?;
  let repo = Repo::get_resource(&repo_id).await?;
  if request_branch != repo.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }
  State
    .resolve(
      execute::CloneRepo { repo: repo_id },
      github_user().to_owned(),
    )
    .await?;
  Ok(())
}

async fn handle_repo_pull_webhook(
  repo_id: String,
  headers: HeaderMap,
  body: String,
) -> anyhow::Result<()> {
  // Acquire and hold lock to make a task queue for
  // subsequent listener calls on same resource.
  // It would fail if we let it go through from action state busy.
  let lock = repo_locks().get_or_insert_default(&repo_id).await;
  let _lock = lock.lock().await;

  verify_gh_signature(headers, &body).await?;
  let request_branch = extract_branch(&body)?;
  let repo = Repo::get_resource(&repo_id).await?;
  if request_branch != repo.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }
  State
    .resolve(
      execute::PullRepo { repo: repo_id },
      github_user().to_owned(),
    )
    .await?;
  Ok(())
}

async fn handle_procedure_webhook(
  procedure_id: String,
  target_branch: String,
  headers: HeaderMap,
  body: String,
) -> anyhow::Result<()> {
  // Acquire and hold lock to make a task queue for
  // subsequent listener calls on same resource.
  // It would fail if we let it go through from action state busy.
  let lock = procedure_locks().get_or_insert_default(&procedure_id).await;
  let _lock = lock.lock().await;

  verify_gh_signature(headers, &body).await?;
  let request_branch = extract_branch(&body)?;
  if request_branch != target_branch {
    return Err(anyhow!("request branch does not match expected"));
  }
  State
    .resolve(
      execute::RunProcedure {
        procedure: procedure_id,
      },
      github_user().to_owned(),
    )
    .await?;
  Ok(())
}

#[instrument(skip_all)]
async fn verify_gh_signature(
  headers: HeaderMap,
  body: &str,
) -> anyhow::Result<()> {
  // wait random amount of time
  tokio::time::sleep(random_duration(0, 500)).await;

  let signature = headers.get("x-hub-signature-256");
  if signature.is_none() {
    return Err(anyhow!("no signature in headers"));
  }
  let signature = signature.unwrap().to_str();
  if signature.is_err() {
    return Err(anyhow!("failed to unwrap signature"));
  }
  let signature = signature.unwrap().replace("sha256=", "");
  let mut mac = HmacSha256::new_from_slice(
    core_config().github_webhook_secret.as_bytes(),
  )
  .expect("github webhook | failed to create hmac sha256");
  mac.update(body.as_bytes());
  let expected = mac.finalize().into_bytes().encode_hex::<String>();
  if signature == expected {
    Ok(())
  } else {
    Err(anyhow!("signature does not equal expected"))
  }
}

#[derive(Deserialize)]
struct GithubWebhookBody {
  #[serde(rename = "ref")]
  branch: String,
}

fn extract_branch(body: &str) -> anyhow::Result<String> {
  let branch = serde_json::from_str::<GithubWebhookBody>(body)
    .context("failed to parse github request body")?
    .branch
    .replace("refs/heads/", "");
  Ok(branch)
}

type ListenerLockCache = Cache<String, Arc<Mutex<()>>>;

fn build_locks() -> &'static ListenerLockCache {
  static BUILD_LOCKS: OnceLock<ListenerLockCache> = OnceLock::new();
  BUILD_LOCKS.get_or_init(Default::default)
}

fn repo_locks() -> &'static ListenerLockCache {
  static REPO_LOCKS: OnceLock<ListenerLockCache> = OnceLock::new();
  REPO_LOCKS.get_or_init(Default::default)
}

fn procedure_locks() -> &'static ListenerLockCache {
  static BUILD_LOCKS: OnceLock<ListenerLockCache> = OnceLock::new();
  BUILD_LOCKS.get_or_init(Default::default)
}
