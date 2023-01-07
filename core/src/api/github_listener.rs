use anyhow::{anyhow, Context};
use axum::{extract::Path, http::HeaderMap, routing::post, Router};
use axum_oauth2::random_duration;
use helpers::handle_anyhow_error;
use hex::ToHex;
use hmac::{Hmac, Mac};
use mungos::Deserialize;
use sha2::Sha256;
use types::GITHUB_WEBHOOK_USER_ID;

use crate::{
    auth::RequestUser,
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
            post(|state: StateExtension, Path(Id { id }), headers: HeaderMap, body: String| async move {
				state.handle_build_webhook(&id, headers, body).await.map_err(handle_anyhow_error)
			}),
        )
        .route(
            "/deployment/:id",
            post(|state: StateExtension, Path(Id { id }), headers: HeaderMap, body: String| async move {
				state.handle_deployment_webhook(&id, headers, body).await.map_err(handle_anyhow_error)
			}),
        )
		.route(
			"/procedure/:id",
			post(|state: StateExtension, Path(Id { id }), headers: HeaderMap, body: String| async move {
				state.handle_procedure_webhook(&id, headers, body).await.map_err(handle_anyhow_error)
			}),
		)
}

impl State {
    async fn handle_build_webhook(
        &self,
        id: &str,
        headers: HeaderMap,
        body: String,
    ) -> anyhow::Result<()> {
        self.verify_gh_signature(headers, &body).await?;
        let request_branch = extract_branch(&body)?;
        let expected_branch = self
            .db
            .get_build(id)
            .await?
            .branch
            .ok_or(anyhow!("build has no branch attached"))?;
        if request_branch != expected_branch {
            return Err(anyhow!("request branch does not match expected"));
        }
        self.build(
            id,
            &RequestUser {
                id: String::from(GITHUB_WEBHOOK_USER_ID),
                is_admin: true,
                create_server_permissions: false,
            },
        )
        .await?;
        Ok(())
    }

    async fn handle_deployment_webhook(
        &self,
        id: &str,
        headers: HeaderMap,
        body: String,
    ) -> anyhow::Result<()> {
        self.verify_gh_signature(headers, &body).await?;
        let request_branch = extract_branch(&body)?;
        let expected_branch = self
            .db
            .get_deployment(id)
            .await?
            .branch
            .ok_or(anyhow!("deployment has no branch attached"))?;
        if request_branch != expected_branch {
            return Err(anyhow!("request branch does not match expected"));
        }
        self.pull_deployment_repo(
            id,
            &RequestUser {
                id: String::from(GITHUB_WEBHOOK_USER_ID),
                is_admin: true,
                create_server_permissions: false,
            },
        )
        .await?;
        Ok(())
    }

    async fn handle_procedure_webhook(
        &self,
        id: &str,
        headers: HeaderMap,
        body: String,
    ) -> anyhow::Result<()> {
        self.verify_gh_signature(headers, &body).await?;
        let request_branch = extract_branch(&body)?;
        let expected_branches = self.db.get_procedure(id).await?.webhook_branches;
        if !expected_branches.contains(&request_branch) {
            return Err(anyhow!("request branch does not match expected"));
        }
        self.run_procedure(
            id,
            &RequestUser {
                id: String::from(GITHUB_WEBHOOK_USER_ID),
                is_admin: true,
                create_server_permissions: false,
            },
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
