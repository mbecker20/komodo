use anyhow::Context;
use axum::{routing::post, Extension, Json, Router};
use helpers::handle_anyhow_error;
use types::{Build, Log};

use crate::{
    helpers::{docker, get_docker_token},
    PeripheryConfigExtension,
};

pub fn router() -> Router {
    Router::new().route(
        "/",
        post(|config, build| async move {
            build_image(config, build)
                .await
                .map_err(handle_anyhow_error)
        }),
    )
}

async fn build_image(
    Extension(config): PeripheryConfigExtension,
    Json(build): Json<Build>,
) -> anyhow::Result<Json<Vec<Log>>> {
    tokio::spawn(async move {
        let logs = match get_docker_token(&build.docker_account, &config) {
            Ok(docker_token) => {
                match docker::build(
                    &build,
                    config.repo_dir.clone(),
                    docker_token,
                    &config.secrets,
                )
                .await
                {
                    Ok(logs) => logs,
                    Err(e) => vec![Log::error("build", format!("{e:#?}"))],
                }
            }
            Err(e) => vec![Log::error("build", format!("{e:#?}"))],
        };
        Json(logs)
    })
    .await
    .context("build thread panicked")
}
