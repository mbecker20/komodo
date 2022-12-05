use std::sync::Arc;

use anyhow::anyhow;
use axum::{middleware, routing::get, Extension, Json, Router};
use db::DbExtension;
use helpers::handle_anyhow_error;
use periphery::PeripheryClient;
use types::User;

use crate::auth::{auth_request, RequestUserExtension};

mod build;
mod deployment;
mod permissions;
mod secret;
mod server;

type PeripheryExtension = Extension<Arc<PeripheryClient>>;

pub fn router() -> Router {
    Router::new()
        .route(
            "/user",
            get(|user, db| async { get_user(user, db).await.map_err(handle_anyhow_error) }),
        )
        .nest("/build", build::router())
        .nest("/deployment", deployment::router())
        .nest("/server", server::router())
        .nest("/permissions", permissions::router())
        .nest("/secret", secret::router())
        .layer(Extension(Arc::new(PeripheryClient::new())))
        .layer(middleware::from_fn(auth_request))
}

async fn get_user(
    Extension(user): RequestUserExtension,
    Extension(db): DbExtension,
) -> anyhow::Result<Json<User>> {
    let mut user = db
        .users
        .find_one_by_id(&user.id)
        .await?
        .ok_or(anyhow!("did not find user"))?;
    user.password = None;
    for secret in &mut user.secrets {
        secret.hash = String::new();
    }
    Ok(Json(user))
}
