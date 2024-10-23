use anyhow::{anyhow, Context};
use serde::Deserialize;

use crate::config::core_config;

/// Listener implementation for Gitlab type API
pub struct Gitlab;

impl super::VerifySecret for Gitlab {
  #[instrument("VerifyGitlabSecret", skip_all)]
  fn verify_secret(
    headers: axum::http::HeaderMap,
    _body: &str,
    custom_secret: &str,
  ) -> anyhow::Result<()> {
    let token = headers
      .get("x-gitlab-token")
      .context("No gitlab token in headers")?;
    let token =
      token.to_str().context("Failed to get token as string")?;
    let secret = if custom_secret.is_empty() {
      core_config().webhook_secret.as_str()
    } else {
      custom_secret
    };
    if token == secret {
      Ok(())
    } else {
      Err(anyhow!("Webhook secret does not match expected."))
    }
  }
}

#[derive(Deserialize)]
struct GitlabWebhookBody {
  #[serde(rename = "ref")]
  branch: String,
}

impl super::ExtractBranch for Gitlab {
  fn extract_branch(body: &str) -> anyhow::Result<String> {
    let branch = serde_json::from_str::<GitlabWebhookBody>(body)
      .context("Failed to parse gitlab request body")?
      .branch
      .replace("refs/heads/", "");
    Ok(branch)
  }
}
