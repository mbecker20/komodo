use std::{path::PathBuf, str::FromStr};

use axum::{routing::post, Extension, Json, Router};
use helpers::{
    git::{self, CloneArgs},
    handle_anyhow_error, to_monitor_name,
};
use serde::Deserialize;
use types::Log;

use crate::{helpers::get_github_token, PeripheryConfigExtension};

#[derive(Deserialize)]
pub struct DeleteRepoBody {
    name: String,
}

#[derive(Deserialize)]
pub struct PullBody {
    name: String,
    branch: Option<String>,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/clone",
            post(|config, clone_args| async move {
                clone_repo(config, clone_args)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .route(
            "/pull",
            post(|config, body| async move {
                pull_repo(config, body).await.map_err(handle_anyhow_error)
            }),
        )
        .route(
            "/delete",
            post(|config, body| async move {
                delete_repo(config, body).await.map_err(handle_anyhow_error)
            }),
        )
}

async fn clone_repo(
    Extension(config): PeripheryConfigExtension,
    Json(clone_args): Json<CloneArgs>,
) -> anyhow::Result<Json<Vec<Log>>> {
    let access_token = get_github_token(&clone_args.github_account, &config)?;
    let logs = git::clone_repo(clone_args, &config.repo_dir, access_token).await?;
    Ok(Json(logs))
}

async fn delete_repo(
    Extension(config): PeripheryConfigExtension,
    Json(DeleteRepoBody { name }): Json<DeleteRepoBody>,
) -> anyhow::Result<Json<Log>> {
    let mut repo_dir = PathBuf::from_str(&config.repo_dir)?;
    let name = to_monitor_name(&name);
    repo_dir.push(&name);
    let destination = repo_dir.display().to_string();
    let deleted = std::fs::remove_dir_all(destination);
    let msg = match deleted {
        Ok(_) => format!("deleted repo {name}"),
        Err(_) => format!("no repo at {name} to delete"),
    };
    let log = Log::simple("delete repo", msg);
    Ok(Json(log))
}

async fn pull_repo(
    Extension(config): PeripheryConfigExtension,
    Json(PullBody { name, branch }): Json<PullBody>,
) -> anyhow::Result<Json<Log>> {
    let mut repo_dir = PathBuf::from_str(&config.repo_dir)?;
    let name = to_monitor_name(&name);
    repo_dir.push(&name);
    let path = repo_dir.display().to_string();
    let log = git::pull(&path, &branch).await;
    Ok(Json(log))
}
