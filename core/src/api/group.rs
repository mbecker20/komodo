use anyhow::Context;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use helpers::handle_anyhow_error;
use mungos::mongodb::bson::Document;
use serde::{Deserialize, Serialize};
use types::{traits::Permissioned, Group, PermissionLevel};
use typeshare::typeshare;

use crate::{
    auth::{RequestUser, RequestUserExtension},
    response,
    state::{State, StateExtension},
};

#[derive(Serialize, Deserialize)]
pub struct GroupId {
    id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct CreateGroupBody {
    name: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/:id",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(group_id): Path<GroupId>| async move {
                    let group = state
                        .get_group_check_permissions(&group_id.id, &user, PermissionLevel::Read)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(group))
                },
            ),
        )
        .route(
            "/list",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Query(query): Query<Document>| async move {
                    let groups = state
                        .list_groups(&user, query)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(groups))
                },
            ),
        )
        .route(
            "/create",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(group): Json<CreateGroupBody>| async move {
                    let group = state
                        .create_group(&group.name, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(group))
                },
            ),
        )
        .route(
            "/create_full",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(group): Json<Group>| async move {
                    let group = state
                        .create_full_group(group, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(group))
                },
            ),
        )
        .route(
            "/:id/delete",
            delete(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(group_id): Path<GroupId>| async move {
                    let group = state
                        .delete_group(&group_id.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(group))
                },
            ),
        )
        .route(
            "/update",
            patch(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(group): Json<Group>| async move {
                    let group = state
                        .update_group(group, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(group))
                },
            ),
        )
}

impl State {
    async fn list_groups(
        &self,
        user: &RequestUser,
        query: impl Into<Option<Document>>,
    ) -> anyhow::Result<Vec<Group>> {
        let groups: Vec<Group> = self
            .db
            .groups
            .get_some(query, None)
            .await
            .context("failed at get all groups query")?
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
        // groups.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        Ok(groups)
    }
}
