use anyhow::anyhow;
use axum::{http::StatusCode, middleware, routing::get, Extension, Json, Router};
use db::DbExtension;
use helpers::handle_anyhow_error;
use types::{User, UserId};

use crate::{
    auth::{auth_request, RequestUserExtension},
    ResponseResult,
};

pub fn router() -> Router {
    Router::new()
        .route(
            "/user",
            get(|user, db| async { get_user(user, db).await.map_err(handle_anyhow_error) }),
        )
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
