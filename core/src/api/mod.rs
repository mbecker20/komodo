use anyhow::{anyhow, Context};
use axum::{
    body::Body,
    extract::Path,
    http::{Request, StatusCode},
    middleware,
    routing::{get, post},
    Extension, Json, Router,
};
use futures_util::Future;
use helpers::handle_anyhow_error;
use mungos::{doc, Deserialize};
use types::{PermissionLevel, UpdateTarget, User};
use typeshare::typeshare;

use crate::{
    auth::{auth_request, JwtExtension, RequestUser, RequestUserExtension},
    response,
    state::{State, StateExtension},
    ResponseResult,
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

#[typeshare]
#[derive(Deserialize)]
struct UpdateDescriptionBody {
    target: UpdateTarget,
    description: String,
}

#[derive(Deserialize)]
struct UserId {
    id: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/title",
            get(|state: StateExtension| async move { state.config.title.clone() }),
        )
        .route("/user", get(get_request_user))
        .nest("/listener", github_listener::router())
        .nest(
            "/",
            Router::new()
                .route("/user/:id", get(get_user_at_id))
                .route(
                    "/username/:id",
                    get(|state: StateExtension, Path(UserId { id })| async move {
                        let user = state
                            .db
                            .get_user(&id)
                            .await
                            .context("failed to find user at id")
                            .map_err(handle_anyhow_error)?;
                        response!(Json(user.username))
                    }),
                )
                .route(
                    "/github_webhook_base_url",
                    get(|state: StateExtension| async move {
                        state
                            .config
                            .github_webhook_base_url
                            .as_ref()
                            .unwrap_or(&state.config.host)
                            .to_string()
                    }),
                )
                .route(
                    "/update_description",
                    post(
                        |state: StateExtension,
                         user: RequestUserExtension,
                         body: Json<UpdateDescriptionBody>| async move {
                            state
                                .update_description(&body.target, &body.description, &user)
                                .await
                                .map_err(handle_anyhow_error)
                        },
                    ),
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

async fn get_request_user(
    Extension(jwt): JwtExtension,
    req: Request<Body>,
) -> ResponseResult<Json<User>> {
    let mut user = jwt.authenticate(&req).await.map_err(handle_anyhow_error)?;
    user.password = None;
    for secret in &mut user.secrets {
        secret.hash = String::new();
    }
    Ok(Json(user))
}

async fn get_users(
    state: StateExtension,
    user: RequestUserExtension,
) -> ResponseResult<Json<Vec<User>>> {
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

async fn get_user_at_id(
    state: StateExtension,
    Path(UserId { id }): Path<UserId>,
    user: RequestUserExtension,
) -> ResponseResult<Json<User>> {
    if user.is_admin {
        let mut user = state
            .db
            .users
            .find_one_by_id(&id)
            .await
            .context("failed at query to get user from mongo")
            .map_err(handle_anyhow_error)?
            .ok_or(anyhow!(""))
            .map_err(handle_anyhow_error)?;
        user.password = None;
        for secret in &mut user.secrets {
            secret.hash = String::new();
        }
        Ok(Json(user))
    } else {
        Err((StatusCode::UNAUTHORIZED, "user is not admin".to_string()))
    }
}

// need to run requested actions in here to prevent them being dropped mid action when user disconnects prematurely
pub async fn spawn_request_action<A>(action: A) -> ResponseResult<A::Output>
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

impl State {
    pub async fn update_description(
        &self,
        target: &UpdateTarget,
        description: &str,
        user: &RequestUser,
    ) -> anyhow::Result<()> {
        match target {
            UpdateTarget::Build(id) => {
                self.get_build_check_permissions(id, user, PermissionLevel::Update)
                    .await?;
                self.db
                    .builds
                    .update_one::<()>(id, mungos::Update::Set(doc! { "description": description }))
                    .await?;
            }
            UpdateTarget::Deployment(id) => {
                self.get_deployment_check_permissions(id, user, PermissionLevel::Update)
                    .await?;
                self.db
                    .deployments
                    .update_one::<()>(id, mungos::Update::Set(doc! { "description": description }))
                    .await?;
            }
            UpdateTarget::Server(id) => {
                self.get_server_check_permissions(id, user, PermissionLevel::Update)
                    .await?;
                self.db
                    .servers
                    .update_one::<()>(id, mungos::Update::Set(doc! { "description": description }))
                    .await?;
            }
            UpdateTarget::Group(id) => {
                self.get_group_check_permissions(id, user, PermissionLevel::Update)
                    .await?;
                self.db
                    .groups
                    .update_one::<()>(id, mungos::Update::Set(doc! { "description": description }))
                    .await?;
            }
            UpdateTarget::Procedure(id) => {
                self.get_procedure_check_permissions(id, user, PermissionLevel::Update)
                    .await?;
                self.db
                    .procedures
                    .update_one::<()>(id, mungos::Update::Set(doc! { "description": description }))
                    .await?;
            }
            _ => return Err(anyhow!("invalid target: {target:?}")),
        }
        Ok(())
    }
}
