use axum::{
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Extension, Json, Router,
};
use docker::{
    deploy, docker_start, docker_stop, docker_stop_and_remove, parse_container_name, DockerClient,
    DockerExtension,
};
use serde::Deserialize;
use types::Deployment;

use crate::{helpers::handle_anyhow_error, response};

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
                Json(docker_start(&parse_container_name(&container.name)).await)
            }),
        )
        .route(
            "/stop",
            post(|Json(container): Json<Container>| async move {
                Json(docker_stop(&parse_container_name(&container.name)).await)
            }),
        )
        .route(
            "/remove",
            post(|Json(container): Json<Container>| async move {
                Json(docker_stop_and_remove(&parse_container_name(&container.name)).await)
            }),
        )
        .route(
            "/deploy",
            post(
                |Json(deployment): Json<Deployment>| async move { Json(deploy(&deployment).await) },
            ),
        )
        .layer(DockerClient::extension())
}
