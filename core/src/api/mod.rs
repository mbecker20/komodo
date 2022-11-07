use anyhow::anyhow;
use axum::{http::StatusCode, middleware, routing::get, Extension, Json, Router};
use db::DbExtension;
use types::{User, UserId};

use crate::{auth::auth_request, helpers::handle_anyhow_error, ResponseResult};

pub fn router() -> Router {
    Router::new()
        .route(
            "/user",
            get(|user_id, db| async { get_user(user_id, db).await.map_err(handle_anyhow_error) }),
        )
        .layer(middleware::from_fn(auth_request))
}

async fn get_user(
    Extension(user_id): Extension<UserId>,
    Extension(db): DbExtension,
) -> anyhow::Result<Json<User>> {
    let mut user = db
        .users
        .find_one_by_id(&user_id)
        .await?
        .ok_or(anyhow!("did not find user"))?;
    user.password = None;
    Ok(Json(user))
}
