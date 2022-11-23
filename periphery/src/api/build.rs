use std::sync::Arc;

use anyhow::anyhow;
use axum::{routing::post, Extension, Json, Router};
use helpers::{docker, handle_anyhow_error};
use tokio::sync::Mutex;
use types::{Build, DockerToken, Log, PeripheryConfig, PERIPHERY_BUILDER_BUSY};

use crate::PeripheryConfigExtension;

type BusyExtension = Extension<Arc<Mutex<bool>>>;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            post(|config, busy, build| async move {
                build_image(config, busy, build)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .layer(Extension(Arc::new(Mutex::new(false))))
}

async fn build_image(
    Extension(config): PeripheryConfigExtension,
    Extension(busy): BusyExtension,
    Json(build): Json<Build>,
) -> anyhow::Result<Json<Vec<Log>>> {
    let is_busy = {
        let mut lock = busy.lock().await;
        if *lock {
            true
        } else {
            *lock = true;
            false
        }
    };
    if is_busy {
        return Err(anyhow!("{PERIPHERY_BUILDER_BUSY}"));
    }
    let docker_token = get_docker_token(&build.docker_account, &config)?;
    let logs = docker::build(&build, &config.repo_dir, docker_token).await?;
    let mut lock = busy.lock().await;
    *lock = false;
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
