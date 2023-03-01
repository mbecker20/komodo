use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use helpers::handle_anyhow_error;
use serde::Deserialize;

use crate::{
    helpers::docker::{self, DockerExtension},
    response,
};

#[derive(Deserialize, Clone)]
pub struct NetworkReqBody {
    name: String,
    driver: Option<String>,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/list",
            get(|Extension(docker): DockerExtension| async move {
                let networks = docker.list_networks().await.map_err(handle_anyhow_error)?;
                response!(Json(networks))
            }),
        )
        .route(
            "/create",
            post(|Json(body): Json<NetworkReqBody>| async move {
                Json(docker::create_network(&body.name, body.driver).await)
            }),
        )
        .route(
            "/delete",
            post(|Json(body): Json<NetworkReqBody>| async move {
                Json(docker::delete_network(&body.name).await)
            }),
        )
        .route(
            "/prune",
            post(|| async move { Json(docker::prune_networks().await) }),
        )
}
