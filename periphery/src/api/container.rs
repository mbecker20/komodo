use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};
use helpers::{
    docker::{self, parse_container_name, DockerClient, DockerExtension},
    handle_anyhow_error,
};
use serde::Deserialize;
use types::Deployment;

use crate::response;

#[derive(Deserialize)]
struct Container {
    name: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/list",
            get(|Extension(dc): DockerExtension| async move {
                let containers = dc.list_containers().await.map_err(handle_anyhow_error)?;
                response!(Json(containers))
            }),
        )
        .route(
            "/start",
            post(|Json(container): Json<Container>| async move {
                Json(docker::start_container(&parse_container_name(&container.name)).await)
            }),
        )
        .route(
            "/stop",
            post(|Json(container): Json<Container>| async move {
                Json(docker::stop_container(&parse_container_name(&container.name)).await)
            }),
        )
        .route(
            "/remove",
            post(|Json(container): Json<Container>| async move {
                Json(
                    docker::stop_and_remove_container(&parse_container_name(&container.name)).await,
                )
            }),
        )
        .route(
            "/deploy",
            post(|Json(deployment): Json<Deployment>| async move {
                Json(docker::deploy(&deployment).await)
            }),
        )
        .route(
            "/prune",
            post(|| async { Json(docker::prune_containers().await) }),
        )
        .layer(DockerClient::extension())
}
