use std::sync::Arc;

use axum::{routing::post, Extension, Json, Router};
use helpers::docker;
use tokio::sync::Mutex;
use types::{Build, Log};

use crate::{helpers::get_docker_token, PeripheryConfigExtension};

type BusyExtension = Extension<Arc<Mutex<bool>>>;

pub fn router() -> Router {
    Router::new()
        .route(
            "/",
            post(|config, busy, build| async move { build_image(config, busy, build).await }),
        )
        .layer(Extension(Arc::new(Mutex::new(false))))
}

async fn build_image(
    Extension(config): PeripheryConfigExtension,
    Extension(busy): BusyExtension,
    Json(build): Json<Build>,
) -> Json<Vec<Log>> {
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
        return Json(vec![Log::error(
            "build",
            "builder busy, try again when other build finishes".to_string(),
        )]);
    }
    let logs = match get_docker_token(&build.docker_account, &config) {
        Ok(docker_token) => {
            match docker::build(&build, config.repo_dir.clone(), docker_token).await {
                Ok(logs) => logs,
                Err(e) => vec![Log::error("build", format!("{e:#?}"))],
            }
        }
        Err(e) => vec![Log::error("build", format!("{e:#?}"))],
    };
    let mut lock = busy.lock().await;
    *lock = false;
    Json(logs)
}
