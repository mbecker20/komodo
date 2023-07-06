use anyhow::{anyhow, Context};
use axum::{extract::Path, http::HeaderMap, routing::post, Router};
use hex::ToHex;
use hmac::{Hmac, Mac};
use monitor_types::requests::execute;
use resolver_api::Resolve;
use serde::Deserialize;
use sha2::Sha256;

use crate::{
    auth::InnerRequestUser,
    helpers::random_duration,
    state::{State, StateExtension},
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
				|state: StateExtension, Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
						let res = state.handle_build_webhook(id.clone(), headers, body).await;
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
				|state: StateExtension, Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
						let res = state.handle_repo_clone_webhook(id.clone(), headers, body).await;
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
				|state: StateExtension, Path(Id { id }), headers: HeaderMap, body: String| async move {
					tokio::spawn(async move {
						let res = state.handle_repo_pull_webhook(id.clone(), headers, body).await;
						if let Err(e) = res {
							warn!("failed to run repo clone webook for repo {id} | {e:#?}");
						}
					});
				},
			)
		)
}

impl State {
    async fn handle_build_webhook(
        &self,
        build_id: String,
        headers: HeaderMap,
        body: String,
    ) -> anyhow::Result<()> {
        self.verify_gh_signature(headers, &body).await?;
        let request_branch = extract_branch(&body)?;
        let expected_branch = self.get_build(&build_id).await?.config.branch;
        if request_branch != expected_branch {
            return Err(anyhow!("request branch does not match expected"));
        }
        self.resolve(
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
        &self,
        repo_id: String,
        headers: HeaderMap,
        body: String,
    ) -> anyhow::Result<()> {
        self.verify_gh_signature(headers, &body).await?;
        let request_branch = extract_branch(&body)?;
        let expected_branch = self.get_repo(&repo_id).await?.config.branch;
        if request_branch != expected_branch {
            return Err(anyhow!("request branch does not match expected"));
        }
        self.resolve(
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
        &self,
        repo_id: String,
        headers: HeaderMap,
        body: String,
    ) -> anyhow::Result<()> {
        self.verify_gh_signature(headers, &body).await?;
        let request_branch = extract_branch(&body)?;
        let expected_branch = self.get_repo(&repo_id).await?.config.branch;
        if request_branch != expected_branch {
            return Err(anyhow!("request branch does not match expected"));
        }
        self.resolve(
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

    async fn verify_gh_signature(&self, headers: HeaderMap, body: &str) -> anyhow::Result<()> {
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
        let mut mac = HmacSha256::new_from_slice(self.config.github_webhook_secret.as_bytes())
            .expect("github webhook | failed to create hmac sha256");
        mac.update(body.as_bytes());
        let expected = mac.finalize().into_bytes().encode_hex::<String>();
        if signature == expected {
            Ok(())
        } else {
            Err(anyhow!("signature does not equal expected"))
        }
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
