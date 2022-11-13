use axum::{routing::post, Json, Router};
use helpers::docker;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct NetworkReqBody {
    name: String,
    driver: Option<String>,
}

pub fn router() -> Router {
    Router::new()
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
