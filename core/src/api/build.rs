use std::collections::HashMap;

use anyhow::Context;
use async_timing_util::unix_timestamp_ms;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use futures_util::TryStreamExt;
use helpers::handle_anyhow_error;
use mungos::{doc, Deserialize, Document, FindOptions, Serialize};
use types::{
    monitor_ts_from_unix, traits::Permissioned, unix_from_monitor_ts, AwsBuilderConfig, Build,
    BuildActionState, BuildVersionsReponse, Operation, PermissionLevel, UpdateStatus,
};
use typeshare::typeshare;

const NUM_VERSIONS_PER_PAGE: u64 = 10;
const ONE_DAY_MS: i64 = 86400000;

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

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct BuildStatsQuery {
    #[serde(default)]
    page: u32,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct BuildStatsResponse {
    pub total_time: f64,  // in hours
    pub total_count: f64, // number of builds
    pub days: Vec<BuildStatsDay>,
}

#[typeshare]
#[derive(Serialize, Deserialize, Default)]
pub struct BuildStatsDay {
    pub time: f64,
    pub count: f64,
    pub ts: f64,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/:id",
            get(
                |state: StateExtension,
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
                |state: StateExtension,
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
                |state: StateExtension,
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
                |state: StateExtension,
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
                |state: StateExtension,
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
                |state: StateExtension,
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
                |state: StateExtension,
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
                |state: StateExtension,
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
                |state: StateExtension,
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
                |state: StateExtension,
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
            get(|state: StateExtension| async move {
                Json(AwsBuilderConfig {
                    access_key_id: String::new(),
                    secret_access_key: String::new(),
                    ..state.config.aws.clone()
                })
            }),
        )
        .route(
            "/docker_organizations",
            get(|state: StateExtension| async move {
                Json(state.config.docker_organizations.clone())
            }),
        )
        .route(
            "/stats",
            get(|state: StateExtension, query: Query<BuildStatsQuery>| async move {
                let stats = state.get_build_stats(query.page).await.map_err(handle_anyhow_error)?;
                response!(Json(stats))
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

    async fn get_build_versions(
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

    async fn get_build_stats(&self, page: u32) -> anyhow::Result<BuildStatsResponse> {
        let curr_ts = unix_timestamp_ms() as i64;
        let next_day = curr_ts - curr_ts % ONE_DAY_MS + ONE_DAY_MS;

        let close_ts = next_day - page as i64 * 30 * ONE_DAY_MS;
        let open_ts = close_ts - 30 * ONE_DAY_MS;

        let mut build_updates = self
            .db
            .updates
            .collection
            .find(
                doc! {
                    "start_ts": {
                        "$gte": monitor_ts_from_unix(open_ts)
                            .context("open_ts out of bounds")?,
                        "$lt": monitor_ts_from_unix(close_ts)
                            .context("close_ts out of bounds")?
                    }
                },
                None,
            )
            .await?;

        let mut days = HashMap::<i64, BuildStatsDay>::with_capacity(32);

        let mut curr = open_ts;

        while curr < close_ts {
            let stats = BuildStatsDay {
                time: curr as f64,
                ..Default::default()
            };
            days.insert(curr, stats);
            curr += ONE_DAY_MS;
        }

        while let Some(update) = build_updates.try_next().await? {
            if let Some(end_ts) = update.end_ts {
                let start_ts = unix_from_monitor_ts(&update.start_ts)
                    .context("failed to parse update start_ts")?;
                let end_ts =
                    unix_from_monitor_ts(&end_ts).context("failed to parse update end_ts")?;
                let day = start_ts - start_ts % ONE_DAY_MS;
                let mut entry = days.entry(day).or_default();
                entry.count += 1.0;
                entry.time += ms_to_hour(end_ts - start_ts);
            }
        }

        Ok(BuildStatsResponse::new(days.into_values().collect()))
    }
}

impl BuildStatsResponse {
    fn new(days: Vec<BuildStatsDay>) -> BuildStatsResponse {
        let mut total_time = 0.0;
        let mut total_count = 0.0;
        for day in &days {
            total_time += day.time;
            total_count += day.count;
        }
        BuildStatsResponse {
            total_time,
            total_count,
            days,
        }
    }
}

const MS_TO_HOUR_DIVISOR: f64 = 1000.0 * 60.0 * 60.0;
fn ms_to_hour(duration: i64) -> f64 {
    duration as f64 / MS_TO_HOUR_DIVISOR
}
