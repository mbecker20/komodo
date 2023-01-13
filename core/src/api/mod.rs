use anyhow::Context;
use axum::{
    body::Body,
    extract::Path,
    http::{Request, StatusCode},
    middleware,
    routing::get,
    Extension, Json, Router,
};
use futures_util::Future;
use helpers::handle_anyhow_error;
use mungos::Deserialize;
use types::User;

use crate::{
    auth::{auth_request, JwtExtension, RequestUserExtension},
    state::StateExtension,
};

pub mod build;
pub mod deployment;
mod github_listener;
pub mod group;
pub mod permissions;
pub mod procedure;
pub mod secret;
pub mod server;
pub mod update;

pub fn router() -> Router {
    Router::new()
        .route(
            "/user",
            get(|jwt, req| async { get_user(jwt, req).await.map_err(handle_anyhow_error) }),
        )
        .nest("/listener", github_listener::router())
        .nest(
            "/",
            Router::new()
                .route(
                    "/username/:id",
                    get(|state, user_id| async {
                        get_username(state, user_id)
                            .await
                            .map_err(handle_anyhow_error)
                    }),
                )
                .route("/users", get(get_users))
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

#[derive(Deserialize)]
struct UserId {
    id: String,
}

async fn get_username(
    state: StateExtension,
    Path(UserId { id }): Path<UserId>,
) -> anyhow::Result<String> {
    let user = state.db.get_user(&id).await?;
    Ok(user.username)
}

async fn get_users(
    state: StateExtension,
    user: RequestUserExtension,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    if user.is_admin {
        let users = state
            .db
            .users
            .get_some(None, None)
            .await
            .context("failed to get users from db")
            .map_err(handle_anyhow_error)?
            .into_iter()
            .map(|u| User {
                password: None,
                secrets: vec![],
                ..u
            })
            .collect::<Vec<_>>();
        Ok(Json(users))
    } else {
        Err((StatusCode::UNAUTHORIZED, "user is not admin".to_string()))
    }
}

// need to run requested actions in here to prevent them being dropped mid action when user disconnects prematurely
pub async fn spawn_request_action<A>(action: A) -> Result<A::Output, (StatusCode, String)>
where
    A: Future + Send + 'static,
    A::Output: Send + 'static,
{
    let res = tokio::spawn(action)
        .await
        .context("failure at action thread spawn")
        .map_err(handle_anyhow_error)?;
    Ok(res)
}
