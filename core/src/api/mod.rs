use axum::{body::Body, http::Request, middleware, routing::get, Extension, Json, Router};
use helpers::handle_anyhow_error;
use types::User;

use crate::auth::{auth_request, JwtExtension};

pub mod build;
pub mod deployment;
pub mod permissions;
pub mod procedure;
pub mod secret;
pub mod server;
pub mod update;
pub mod group;

pub fn router() -> Router {
    Router::new()
        .route(
            "/user",
            get(|jwt, req| async { get_user(jwt, req).await.map_err(handle_anyhow_error) }),
        )
        .nest(
            "/",
            Router::new()
                .nest("/build", build::router())
                .nest("/deployment", deployment::router())
                .nest("/server", server::router())
                .nest("/procedure", procedure::router())
                .nest("/group", group::router())
                .nest("/update", update::router())
                .nest("/permissions", permissions::router())
                .nest("/secret", secret::router())
                .layer(middleware::from_fn(auth_request)),
        )
}

async fn get_user(Extension(jwt): JwtExtension, req: Request<Body>) -> anyhow::Result<Json<User>> {
    let mut user = jwt.authenticate(&req).await?;
    user.password = None;
    for secret in &mut user.secrets {
        secret.hash = String::new();
    }
    Ok(Json(user))
}
