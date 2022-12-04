use anyhow::anyhow;
use axum::{
    extract::Path,
    routing::{get, post},
    Extension, Json, Router,
};
use helpers::{
    docker::{self, DockerClient, DockerExtension},
    handle_anyhow_error, to_monitor_name,
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
        // .route(
        //     "/stats/:name",
        //     get(
        //         |Extension(dc): DockerExtension, Path(Container { name }): Path<Container>| async move {
        //             let stats = dc
        //                 .get_container_stats(&name)
        //                 .await
        //                 .map_err(handle_anyhow_error)?;
        //             response!(Json(stats))
        //         },
        //     ),
        // )
        // .route(
        //     "/stats/list",
        //     get(
        //         |Extension(dc): DockerExtension| async move {
        //             let stats_list = dc
        //                 .get_container_stats_list()
        //                 .await
        //                 .map_err(handle_anyhow_error)?;
        //             response!(Json(stats_list))
        //         },
        //     ),
        // )
        .route(
            "/stats/:name",
            get(|Path(c): Path<Container>| async move {
                let stats = docker::container_stats(Some(c.name.clone()))
                    .await
                    .map_err(handle_anyhow_error)?
                    .pop()
                    .ok_or(anyhow!("no stats for container {}", c.name))
                    .map_err(handle_anyhow_error)?;
                response!(Json(stats))
            }),
        )
        .route(
            "/stats/list",
            get(|| async {
                let stats = docker::container_stats(None)
                    .await
                    .map_err(handle_anyhow_error)?;
                response!(Json(stats))
            }),
        )
        .route(
            "/start",
            post(|Json(container): Json<Container>| async move {
                Json(docker::start_container(&to_monitor_name(&container.name)).await)
            }),
        )
        .route(
            "/stop",
            post(|Json(container): Json<Container>| async move {
                Json(docker::stop_container(&to_monitor_name(&container.name)).await)
            }),
        )
        .route(
            "/remove",
            post(|Json(container): Json<Container>| async move {
                Json(docker::stop_and_remove_container(&to_monitor_name(&container.name)).await)
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
