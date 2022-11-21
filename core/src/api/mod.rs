use std::sync::Arc;

use anyhow::anyhow;
use axum::{middleware, routing::get, Extension, Json, Router};
use db::DbExtension;
use helpers::handle_anyhow_error;
use periphery::PeripheryClient;
use types::User;

use crate::auth::{auth_request, RequestUserExtension};

mod build;

type PeripheryExtension = Extension<Arc<PeripheryClient>>;

pub fn router() -> Router {
    Router::new()
        .route(
            "/user",
            get(|user, db| async { get_user(user, db).await.map_err(handle_anyhow_error) }),
        )
        .nest("/build", build::router())
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
    Ok(Json(user))
}
