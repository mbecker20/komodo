use std::sync::Arc;

use anyhow::{anyhow, Context};
use axum::{extract::Path, http::HeaderMap, routing::post, Router};
use hex::ToHex;
use hmac::{Hmac, Mac};
use serde::Deserialize;
use sha2::Sha256;
use tokio::sync::Mutex;
use tracing::Instrument;

use crate::{
  config::core_config,
  helpers::{cache::Cache, random_duration},
};

mod build;
mod procedure;
mod repo;
mod stack;
mod sync;

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
              let res = build::handle_build_webhook(id.clone(), headers, body).await;
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
              let res = repo::handle_repo_clone_webhook(id.clone(), headers, body).await;
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
              let res = repo::handle_repo_pull_webhook(id.clone(), headers, body).await;
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
			"/repo/:id/build", 
			post(
				|Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
            let span = info_span!("repo_build_webhook", id);
            async {
              let res = repo::handle_repo_build_webhook(id.clone(), headers, body).await;
              if let Err(e) = res {
                warn!("failed to run repo build webook for repo {id} | {e:#}");
              }
            }
              .instrument(span)
              .await
					});
				},
			)
		)
    .route(
			"/stack/:id/refresh", 
			post(
				|Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
						let span = info_span!("stack_clone_webhook", id);
            async {
              let res = stack::handle_stack_refresh_webhook(id.clone(), headers, body).await;
              if let Err(e) = res {
                warn!("failed to run stack clone webook for stack {id} | {e:#}");
              }
            }
              .instrument(span)
              .await
					});
				},
			)
		)
		.route(
			"/stack/:id/deploy", 
			post(
				|Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
            let span = info_span!("stack_pull_webhook", id);
            async {
              let res = stack::handle_stack_deploy_webhook(id.clone(), headers, body).await;
              if let Err(e) = res {
                warn!("failed to run stack pull webook for stack {id} | {e:#}");
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
              let res = procedure::handle_procedure_webhook(
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
    .route(
			"/sync/:id/refresh", 
			post(
				|Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
            let span = info_span!("sync_refresh_webhook", id);
            async {
              let res = sync::handle_sync_refresh_webhook(
                id.clone(),
                headers,
                body
              ).await;
              if let Err(e) = res {
                warn!("failed to run sync webook for sync {id} | {e:#}");
              }
            }
              .instrument(span)
              .await
					});
				},
			)
		)
    .route(
			"/sync/:id/sync", 
			post(
				|Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
            let span = info_span!("sync_execute_webhook", id);
            async {
              let res = sync::handle_sync_execute_webhook(
                id.clone(),
                headers,
                body
              ).await;
              if let Err(e) = res {
                warn!("failed to run sync webook for sync {id} | {e:#}");
              }
            }
              .instrument(span)
              .await
					});
				},
			)
		)
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
    core_config().webhook_secret.as_bytes(),
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
