use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use helpers::handle_anyhow_error;

use crate::{
    helpers::docker::{self, DockerExtension},
    response,
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/list",
            get(|Extension(docker): DockerExtension| async move {
                let images = docker.list_images().await.map_err(handle_anyhow_error)?;
                response!(Json(images))
            }),
        )
        .route(
            "/prune",
            post(|| async move { Json(docker::prune_images().await) }),
        )
}
