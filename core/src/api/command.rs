use anyhow::Context;
use axum::{Router, routing::{get, post, delete, patch}, Json, extract::{Path, Query}};
use helpers::handle_anyhow_error;
use mungos::mongodb::bson::Document;
use serde::{Serialize, Deserialize};
use types::{PeripheryCommand, PermissionLevel, traits::Permissioned, CommandActionState};
use typeshare::typeshare;

use crate::{state::{State, StateExtension}, auth::{RequestUser, RequestUserExtension}, response, api::spawn_request_action};

#[derive(Serialize, Deserialize)]
pub struct CommandId {
    id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct CreateCommandBody {
    name: String,
    server_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct CopyCommandBody {
    name: String,
    server_id: String,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/:id",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(CommandId { id })| async move {
                    let command = state
                        .get_command_check_permissions(&id, &user, PermissionLevel::Read)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(command))
                },
            ),
        )
        .route(
            "/list",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Query(query): Query<Document>| async move {
                    let commands = state
                        .list_commands(&user, query)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(commands))
                },
            ),
        )
        .route(
            "/create",
            post(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Json(command): Json<CreateCommandBody>| async move {
                    let command = state
                        .create_command(&command.name, command.server_id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(command))
                },
            ),
        )
        .route(
            "/create_full",
            post(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Json(command): Json<PeripheryCommand>| async move {
                    let command = spawn_request_action(async move {
                        state
                            .create_full_command(command, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(command))
                },
            ),
        )
        .route(
            "/:id/copy",
            post(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(CommandId { id }),
                 Json(command): Json<CopyCommandBody>| async move {
                    let command = spawn_request_action(async move {
                        state
                            .copy_command(&id, command.name, command.server_id, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(command))
                },
            ),
        )
        .route(
            "/:id/delete",
            delete(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(CommandId { id })| async move {
                    let build = spawn_request_action(async move {
                        state
                            .delete_command(&id, &user)
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
                 user: RequestUserExtension,
                 Json(command): Json<PeripheryCommand>| async move {
                    let command = spawn_request_action(async move {
                        state
                            .update_command(command, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(command))
                },
            ),
        )
        .route(
            "/:id/action_state",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(CommandId { id })| async move {
                    let action_state = state
                        .get_command_action_states(id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(action_state))
                },
            ),
        )
        .route(
            "/:id/run",
            post(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(CommandId { id })| async move {
                    let update = spawn_request_action(async move {
                        state
                            .run_command(&id, &user)
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
    async fn list_commands(
        &self,
        user: &RequestUser,
        query: impl Into<Option<Document>>,
    ) -> anyhow::Result<Vec<PeripheryCommand>> {
        let commands: Vec<PeripheryCommand> = self
            .db
            .commands
            .get_some(query, None)
            .await
            .context("failed at get all commands query")?
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
        Ok(commands)
    }

    async fn get_command_action_states(
        &self,
        id: String,
        user: &RequestUser,
    ) -> anyhow::Result<CommandActionState> {
        self.get_command_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        let action_state = self
            .command_action_states
            .lock()
            .await
            .entry(id)
            .or_default()
            .clone();
        Ok(action_state)
    }
}