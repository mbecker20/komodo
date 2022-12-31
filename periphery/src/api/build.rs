use std::sync::Arc;

use anyhow::anyhow;
use axum::{routing::post, Extension, Json, Router};
use helpers::{docker, handle_anyhow_error};
use tokio::sync::Mutex;
use types::{Build, Log, PERIPHERY_BUILDER_BUSY};

use crate::{helpers::get_docker_token, PeripheryConfigExtension};

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
    let logs = docker::build(&build, config.repo_dir.clone(), docker_token).await?;
    let mut lock = busy.lock().await;
    *lock = false;
    Ok(Json(logs))
}
