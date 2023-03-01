use anyhow::Context;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use helpers::handle_anyhow_error;
use mungos::{doc, Deserialize, Document, FindOptions, Serialize};
use types::{
    traits::Permissioned, AwsBuilderConfig, Build, BuildActionState, BuildVersionsReponse,
    Operation, PermissionLevel, UpdateStatus,
};
use typeshare::typeshare;

const NUM_VERSIONS_PER_PAGE: u64 = 10;

use crate::{
    auth::{RequestUser, RequestUserExtension},
    response,
    state::{State, StateExtension},
};

use super::spawn_request_action;

#[derive(Serialize, Deserialize)]
struct BuildId {
    id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
struct CreateBuildBody {
    name: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
struct CopyBuildBody {
    name: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct BuildVersionsQuery {
    #[serde(default)]
    page: u32,
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
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
                        .create_build(&build.name, &user)
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
                    let build = spawn_request_action(async move {
                        state
                            .create_full_build(build, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
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
                    let build = spawn_request_action(async move {
                        state
                            .copy_build(&id, build.name, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
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
                    let build = spawn_request_action(async move {
                        state
                            .delete_build(&build_id.id, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
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
                    let build = spawn_request_action(async move {
                        state
                            .update_build(build, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
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
                    let update = spawn_request_action(async move {
                        state
                            .build(&build_id.id, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
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
        .route(
            "/:id/versions",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(BuildId { id }),
                 Query(query): Query<BuildVersionsQuery>| async move {
                    let versions = state
                        .get_build_versions(&id, &user, query)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(versions))
                },
            ),
        )
        .route(
            "/aws_builder_defaults",
            get(|Extension(state): StateExtension| async move {
                Json(AwsBuilderConfig {
                    access_key_id: String::new(),
                    secret_access_key: String::new(),
                    ..state.config.aws.clone()
                })
            }),
        )
        .route(
            "/docker_organizations",
            get(|Extension(state): StateExtension| async move {
                Json(state.config.docker_organizations.clone())
            }),
        )
}

impl State {
    async fn list_builds(
        &self,
        user: &RequestUser,
        query: impl Into<Option<Document>>,
    ) -> anyhow::Result<Vec<Build>> {
        let builds: Vec<Build> = self
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
        Ok(builds)
    }

    async fn get_build_action_states(
        &self,
        id: String,
        user: &RequestUser,
    ) -> anyhow::Result<BuildActionState> {
        self.get_build_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        let action_state = self
            .build_action_states
            .lock()
            .await
            .entry(id)
            .or_default()
            .clone();
        Ok(action_state)
    }

    pub async fn get_build_versions(
        &self,
        id: &str,
        user: &RequestUser,
        query: BuildVersionsQuery,
    ) -> anyhow::Result<Vec<BuildVersionsReponse>> {
        self.get_build_check_permissions(&id, user, PermissionLevel::Read)
            .await?;
        let mut filter = doc! {
            "target": {
                "type": "Build",
                "id": id
            },
            "operation": Operation::BuildBuild.to_string(),
            "status": UpdateStatus::Complete.to_string(),
            "success": true
        };
        if let Some(major) = query.major {
            filter.insert("version.major", major);
        }
        if let Some(minor) = query.minor {
            filter.insert("version.minor", minor);
        }
        if let Some(patch) = query.patch {
            filter.insert("version.patch", patch);
        }
        let versions = self
            .db
            .updates
            .get_some(
                filter,
                FindOptions::builder()
                    .sort(doc! { "_id": -1 })
                    .limit(NUM_VERSIONS_PER_PAGE as i64)
                    .skip(query.page as u64 * NUM_VERSIONS_PER_PAGE)
                    .build(),
            )
            .await
            .context("failed to pull versions from mongo")?
            .into_iter()
            .map(|u| (u.version, u.start_ts))
            .filter(|(v, _)| v.is_some())
            .map(|(v, ts)| BuildVersionsReponse {
                version: v.unwrap(),
                ts,
            })
            .collect();
        Ok(versions)
    }
}
