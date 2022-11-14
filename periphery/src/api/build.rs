use anyhow::anyhow;
use axum::{routing::post, Extension, Json, Router};
use helpers::{docker, handle_anyhow_error};
use types::{Build, DockerToken, Log, PeripheryConfig};

use crate::PeripheryConfigExtension;

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
    let docker_token = get_docker_token(&build.docker_account, &config)?;
    let logs = docker::build(&build, &config.repo_dir, docker_token).await?;
    Ok(Json(logs))
}

fn get_docker_token(
    docker_account: &Option<String>,
    config: &PeripheryConfig,
) -> anyhow::Result<Option<DockerToken>> {
    match docker_account {
        Some(account) => match config.docker_accounts.get(account) {
            Some(token) => Ok(Some(token.to_owned())),
            None => Err(anyhow!(
                "did not find token in config for docker account {account} "
            )),
        },
        None => Ok(None),
    }
}
