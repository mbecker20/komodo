use anyhow::Context;
use axum::{
    extract::Path,
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use helpers::handle_anyhow_error;
use mungos::Deserialize;
use types::{traits::Permissioned, Build, PermissionLevel};

use crate::{auth::RequestUserExtension, response, state::StateExtension};

#[derive(Deserialize)]
struct BuildId {
    id: String,
}

#[derive(Deserialize)]
struct CreateBuildBody {
    name: String,
    server_id: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/list",
            get(|state, user| async { list(state, user).await.map_err(handle_anyhow_error) }),
        )
        .route(
            "/create",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(build): Json<CreateBuildBody>| async move {
                    let build = state
                        .create_build(build.name, build.server_id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(build))
                },
            ),
        )
        .route(
            "/delete/:id",
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
            "/reclone/:id",
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
}

async fn list(
    Extension(state): StateExtension,
    Extension(user): RequestUserExtension,
) -> anyhow::Result<Json<Vec<Build>>> {
    let mut builds: Vec<Build> = state
        .db
        .builds
        .get_some(None, None)
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
    Ok(Json(builds))
}
