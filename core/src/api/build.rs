use anyhow::Context;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use helpers::handle_anyhow_error;
use mungos::{Deserialize, Document, Serialize};
use types::{traits::Permissioned, Build, BuildActionState, PermissionLevel};
use typeshare::typeshare;

use crate::{
    auth::{RequestUser, RequestUserExtension},
    response,
    state::{State, StateExtension},
};

#[derive(Serialize, Deserialize)]
struct BuildId {
    id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
struct CreateBuildBody {
    name: String,
    server_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
struct CopyBuildBody {
    name: String,
    server_id: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/:id",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(build_id): Path<BuildId>| async move {
                    let build = state
                        .get_build_check_permissions(&build_id.id, &user, PermissionLevel::Read)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(build))
                },
            ),
        )
        .route(
            "/list",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Query(query): Query<Document>| async move {
                    let builds = state
                        .list_builds(&user, query)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(builds))
                },
            ),
        )
        .route(
            "/create",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(build): Json<CreateBuildBody>| async move {
                    let build = state
                        .create_build(&build.name, build.server_id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(build))
                },
            ),
        )
        .route(
            "/create_full",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(build): Json<Build>| async move {
                    let build = state
                        .create_full_build(build, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(build))
                },
            ),
        )
        .route(
            "/:id/copy",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(BuildId { id }): Path<BuildId>,
                 Json(build): Json<CopyBuildBody>| async move {
                    let build = state
                        .copy_build(&id, build.name, build.server_id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(build))
                },
            ),
        )
        .route(
            "/:id/delete",
            delete(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(build_id): Path<BuildId>| async move {
                    let build = state
                        .delete_build(&build_id.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(build))
                },
            ),
        )
        .route(
            "/update",
            patch(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(build): Json<Build>| async move {
                    let build = state
                        .update_build(build, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(build))
                },
            ),
        )
        .route(
            "/:id/build",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(build_id): Path<BuildId>| async move {
                    let update = state
                        .build(&build_id.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(update))
                },
            ),
        )
        .route(
            "/:id/reclone",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(build_id): Path<BuildId>| async move {
                    let update = state
                        .reclone_build(&build_id.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(update))
                },
            ),
        )
        .route(
            "/:id/action_state",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(BuildId { id }): Path<BuildId>| async move {
                    let action_state = state
                        .get_build_action_states(id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(action_state))
                },
            ),
        )
}

impl State {
    async fn list_builds(
        &self,
        user: &RequestUser,
        query: impl Into<Option<Document>>,
    ) -> anyhow::Result<Vec<Build>> {
        let mut builds: Vec<Build> = self
            .db
            .builds
            .get_some(query, None)
            .await
            .context("failed at get all builds query")?
            .into_iter()
            .filter(|s| {
                if user.is_admin {
                    true
                } else {
                    let permissions = s.get_user_permissions(&user.id);
                    permissions != PermissionLevel::None
                }
            })
            .collect();
        builds.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        Ok(builds)
    }

    async fn get_build_action_states(
        &self,
        id: String,
        user: &RequestUser,
    ) -> anyhow::Result<BuildActionState> {
        self.get_server_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        let action_state = self
            .build_action_states
            .lock()
            .unwrap()
            .entry(id)
            .or_default()
            .clone();
        Ok(action_state)
    }
}
