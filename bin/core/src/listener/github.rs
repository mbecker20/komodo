use anyhow::{anyhow, Context};
use axum::{extract::Path, http::HeaderMap, routing::post, Router};
use hex::ToHex;
use hmac::{Hmac, Mac};
use monitor_client::{
  api::execute,
  entities::{build::Build, repo::Repo},
};
use resolver_api::Resolve;
use serde::Deserialize;
use sha2::Sha256;

use crate::{
  auth::InnerRequestUser,
  config::core_config,
  helpers::{random_duration, resource::StateResource},
  state::State,
};

type HmacSha256 = Hmac<Sha256>;

#[derive(Deserialize)]
struct Id {
  id: String,
}

pub fn router() -> Router {
  Router::new()
		.route(
			"/build/:id",
			post(
				|Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
						let res = handle_build_webhook(id.clone(), headers, body).await;
						if let Err(e) = res {
							warn!("failed to run build webook for build {id} | {e:#?}");
						}
					});
				},
			),
		)
		.route(
			"/repo/:id/clone", 
			post(
				|Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
						let res = handle_repo_clone_webhook(id.clone(), headers, body).await;
						if let Err(e) = res {
							warn!("failed to run repo clone webook for repo {id} | {e:#?}");
						}
					});
				},
			)
		)
		.route(
			"/repo/:id/pull", 
			post(
				|Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
						let res = handle_repo_pull_webhook(id.clone(), headers, body).await;
						if let Err(e) = res {
							warn!("failed to run repo clone webook for repo {id} | {e:#?}");
						}
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
  verify_gh_signature(headers, &body).await?;
  let request_branch = extract_branch(&body)?;
  let build: Build = State.get_resource(&build_id).await?;
  if request_branch != build.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }
  State
    .resolve(
      execute::RunBuild { build_id },
      InnerRequestUser {
        id: String::from("github"),
        username: String::from("github"),
        is_admin: true,
        create_server_permissions: false,
        create_build_permissions: false,
      }
      .into(),
    )
    .await?;
  Ok(())
}

async fn handle_repo_clone_webhook(
  repo_id: String,
  headers: HeaderMap,
  body: String,
) -> anyhow::Result<()> {
  verify_gh_signature(headers, &body).await?;
  let request_branch = extract_branch(&body)?;
  let repo: Repo = State.get_resource(&repo_id).await?;
  if request_branch != repo.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }
  State
    .resolve(
      execute::CloneRepo { id: repo_id },
      InnerRequestUser {
        id: String::from("github"),
        username: String::from("github"),
        is_admin: true,
        create_server_permissions: false,
        create_build_permissions: false,
      }
      .into(),
    )
    .await?;
  Ok(())
}

async fn handle_repo_pull_webhook(
  repo_id: String,
  headers: HeaderMap,
  body: String,
) -> anyhow::Result<()> {
  verify_gh_signature(headers, &body).await?;
  let request_branch = extract_branch(&body)?;
  let repo: Repo = State.get_resource(&repo_id).await?;
  if request_branch != repo.config.branch {
    return Err(anyhow!("request branch does not match expected"));
  }
  State
    .resolve(
      execute::PullRepo { id: repo_id },
      InnerRequestUser {
        id: String::from("github"),
        username: String::from("github"),
        is_admin: true,
        create_server_permissions: false,
        create_build_permissions: false,
      }
      .into(),
    )
    .await?;
  Ok(())
}

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
