use anyhow::anyhow;
use axum::{routing::post, Extension, Json, Router};
use helpers::{
    git::{self, CloneArgs},
    handle_anyhow_error,
};
use types::{GithubToken, Log, PeripheryConfig};

use crate::PeripheryConfigExtension;

pub fn router() -> Router {
    Router::new().route(
        "/clone",
        post(|config, clone_args| async move {
            clone(config, clone_args).await.map_err(handle_anyhow_error)
        }),
    )
}

async fn clone(
    Extension(config): PeripheryConfigExtension,
    Json(clone_args): Json<CloneArgs>,
) -> anyhow::Result<Json<Vec<Log>>> {
    let access_token = get_github_token(&clone_args.github_account, &config)?;
    let logs = git::clone_repo(clone_args, &config.repo_dir, access_token).await?;
    Ok(Json(logs))
}

fn get_github_token(
    github_account: &Option<String>,
    config: &PeripheryConfig,
) -> anyhow::Result<Option<GithubToken>> {
    match github_account {
        Some(account) => match config.github_accounts.get(account) {
            Some(token) => Ok(Some(token.to_owned())),
            None => Err(anyhow!(
                "did not find token in config for github account {account} "
            )),
        },
        None => Ok(None),
    }
}
