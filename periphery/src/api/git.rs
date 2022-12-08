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
    let log = match deleted {
        Ok(_) => Log::simple(format!("deleted repo {name}")),
        Err(_) => Log::simple(format!("no repo at {name} to delete")),
    };
    Ok(Json(log))
}
