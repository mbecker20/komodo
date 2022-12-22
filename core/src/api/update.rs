use std::str::FromStr;

use anyhow::{anyhow, Context};
use axum::{extract::Query, routing::get, Extension, Json, Router};
use helpers::handle_anyhow_error;
use mungos::{doc, to_bson, ObjectId};
use serde_json::Value;
use types::{PermissionLevel, Update, UpdateTarget};

use crate::{
    auth::{RequestUser, RequestUserExtension},
    response,
    state::{State, StateExtension},
};

const NUM_UPDATES_PER_PAGE: usize = 20;

pub fn router() -> Router {
    Router::new().route(
        "/list",
        get(
            |Extension(state): StateExtension,
             Extension(user): RequestUserExtension,
             Query(value): Query<Value>| async move {
                let offset = value
                    .get("offset")
                    .map(|v| v.as_u64().unwrap_or(0))
                    .unwrap_or(0);
                let target = serde_json::from_str::<UpdateTarget>(&value.to_string()).ok();
                let updates = state
                    .list_updates(target, offset, &user)
                    .await
                    .map_err(handle_anyhow_error)?;
                response!(Json(updates))
            },
        ),
    )
}

impl State {
    async fn permission_on_update_target(
        &self,
        update_target: &UpdateTarget,
        user: &RequestUser,
    ) -> anyhow::Result<()> {
        if user.is_admin {
            Ok(())
        } else {
            match update_target {
                UpdateTarget::System => {
                    if user.is_admin {
                        Ok(())
                    } else {
                        Err(anyhow!("user must be admin to see system updates"))
                    }
                }
                UpdateTarget::Build(id) => self
                    .get_build_check_permissions(id, user, PermissionLevel::Read)
                    .await
                    .map(|_| ()),
                UpdateTarget::Deployment(id) => self
                    .get_deployment_check_permissions(id, user, PermissionLevel::Read)
                    .await
                    .map(|_| ()),
                UpdateTarget::Server(id) => self
                    .get_server_check_permissions(id, user, PermissionLevel::Read)
                    .await
                    .map(|_| ()),
                UpdateTarget::Procedure(id) => self
                    .get_procedure_check_permissions(id, user, PermissionLevel::Read)
                    .await
                    .map(|_| ()),
            }
        }
    }

    pub async fn list_updates(
        &self,
        target: Option<UpdateTarget>,
        offset: u64,
        user: &RequestUser,
    ) -> anyhow::Result<Vec<Update>> {
        let filter = match target {
            Some(target) => {
                self.permission_on_update_target(&target, user).await?;
                Some(doc! {
                    "target": to_bson(&target).unwrap()
                })
            }
            None => {
                if user.is_admin {
                    None
                } else {
                    let permissions_field = format!("permissions.{}", user.id);
                    let target_filter = doc! {
                        "$or": [
                            { &permissions_field: "update" },
                            { &permissions_field: "execute" },
                            { &permissions_field: "read" },
                        ]
                    };
                    let build_ids = self
                        .db
                        .builds
                        .get_some(target_filter.clone(), None)
                        .await
                        .context("failed at query to get users builds")?
                        .into_iter()
                        .map(|e| ObjectId::from_str(&e.id).unwrap())
                        .collect::<Vec<_>>();
                    let deployment_ids = self
                        .db
                        .deployments
                        .get_some(target_filter.clone(), None)
                        .await
                        .context("failed at query to get users deployments")?
                        .into_iter()
                        .map(|e| ObjectId::from_str(&e.id).unwrap())
                        .collect::<Vec<_>>();
                    let server_ids = self
                        .db
                        .servers
                        .get_some(target_filter.clone(), None)
                        .await
                        .context("failed at query to get users servers")?
                        .into_iter()
                        .map(|e| ObjectId::from_str(&e.id).unwrap())
                        .collect::<Vec<_>>();
                    let procedure_ids = self
                        .db
                        .procedures
                        .get_some(target_filter, None)
                        .await
                        .context("failed at query to get users procedures")?
                        .into_iter()
                        .map(|e| ObjectId::from_str(&e.id).unwrap())
                        .collect::<Vec<_>>();
                    let filter = doc! {
                        "$or": [
                           { "target.type": "Build", "target.id": { "$in": &build_ids } },
                           { "target.type": "Deployment", "target.id": { "$in": &deployment_ids } },
                           { "target.type": "Server", "target.id": { "$in": &server_ids } },
                           { "target.type": "Procedure", "target.id": { "$in": &procedure_ids } }
                        ]
                    };
                    Some(filter)
                }
            }
        };
        let mut updates = self
            .db
            .updates
            .get_most_recent(NUM_UPDATES_PER_PAGE as i64, offset, filter, None)
            .await
            .context("mongo get most recent updates query failed")?;
        updates.reverse();
        Ok(updates)
    }
}
