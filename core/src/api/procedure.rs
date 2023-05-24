use anyhow::Context;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Extension, Json, Router,
};
use helpers::handle_anyhow_error;
use mungos::mongodb::bson::Document;
use serde::{Deserialize, Serialize};
use types::{traits::Permissioned, PermissionLevel, Procedure};
use typeshare::typeshare;

use crate::{
    auth::{RequestUser, RequestUserExtension},
    response,
    state::{State, StateExtension},
};

use super::spawn_request_action;

#[derive(Serialize, Deserialize)]
pub struct ProcedureId {
    id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct CreateProcedureBody {
    name: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/:id",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(procedure_id): Path<ProcedureId>| async move {
                    let procedure = state
                        .get_procedure_check_permissions(
                            &procedure_id.id,
                            &user,
                            PermissionLevel::Read,
                        )
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(procedure))
                },
            ),
        )
        .route(
            "/list",
            get(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Query(query): Query<Document>| async move {
                    let procedures = state
                        .list_procedures(&user, query)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(procedures))
                },
            ),
        )
        .route(
            "/create",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(procedure): Json<CreateProcedureBody>| async move {
                    let procedure = state
                        .create_procedure(&procedure.name, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(procedure))
                },
            ),
        )
        .route(
            "/create_full",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(procedure): Json<Procedure>| async move {
                    let procedure = state
                        .create_full_procedure(procedure, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(procedure))
                },
            ),
        )
        .route(
            "/:id/delete",
            delete(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(procedure_id): Path<ProcedureId>| async move {
                    let procedure = state
                        .delete_procedure(&procedure_id.id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(procedure))
                },
            ),
        )
        .route(
            "/update",
            patch(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Json(procedure): Json<Procedure>| async move {
                    let procedure = state
                        .update_procedure(procedure, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(procedure))
                },
            ),
        )
        .route(
            "/:id/run",
            post(
                |Extension(state): StateExtension,
                 Extension(user): RequestUserExtension,
                 Path(procedure_id): Path<ProcedureId>| async move {
                    let update = spawn_request_action(async move {
                        state
                            .run_procedure(&procedure_id.id, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(update))
                },
            ),
        )
}

impl State {
    async fn list_procedures(
        &self,
        user: &RequestUser,
        query: impl Into<Option<Document>>,
    ) -> anyhow::Result<Vec<Procedure>> {
        let procedures: Vec<Procedure> = self
            .db
            .procedures
            .get_some(query, None)
            .await
            .context("failed at get all procedures query")?
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
        // procedures.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        Ok(procedures)
    }
}
