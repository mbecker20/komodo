use std::collections::HashMap;

use anyhow::Context;
use axum::{
    extract::{Path, Query},
    routing::{delete, get, patch, post},
    Json, Router,
};
use futures_util::future::join_all;
use helpers::handle_anyhow_error;
use mungos::{doc, options::FindOneOptions, Deserialize, Document, Serialize};
use types::{
    traits::Permissioned, Deployment, DeploymentActionState, DeploymentWithContainerState,
    DockerContainerState, DockerContainerStats, Log, Operation, PermissionLevel, Server,
    TerminationSignal, UpdateStatus,
};
use typeshare::typeshare;

use crate::{
    auth::{RequestUser, RequestUserExtension},
    response,
    state::{State, StateExtension},
};

use super::spawn_request_action;

#[derive(Serialize, Deserialize)]
pub struct DeploymentId {
    id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct CreateDeploymentBody {
    name: String,
    server_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct CopyDeploymentBody {
    name: String,
    server_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
pub struct RenameDeploymentBody {
    new_name: String,
}

#[typeshare]
#[derive(Deserialize)]
pub struct GetContainerLogQuery {
    tail: Option<u32>,
}

#[typeshare]
#[derive(Deserialize)]
pub struct StopContainerQuery {
    stop_signal: Option<TerminationSignal>,
    stop_time: Option<i32>,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/:id",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(DeploymentId { id })| async move {
                    let res = state
                        .get_deployment_with_container_state(&user, &id)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(res))
                },
            ),
        )
        .route(
            "/list",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Query(query): Query<Document>| async move {
                    let deployments = state
                        .list_deployments_with_container_state(&user, query)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(deployments))
                },
            ),
        )
        .route(
            "/create",
            post(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Json(deployment): Json<CreateDeploymentBody>| async move {
                    let deployment = state
                        .create_deployment(&deployment.name, deployment.server_id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(deployment))
                },
            ),
        )
        .route(
            "/create_full",
            post(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Json(full_deployment): Json<Deployment>| async move {
                    let deployment = spawn_request_action(async move {
                        state
                            .create_full_deployment(full_deployment, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(deployment))
                },
            ),
        )
        .route(
            "/:id/copy",
            post(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(DeploymentId { id }),
                 Json(deployment): Json<CopyDeploymentBody>| async move {
                    let deployment = spawn_request_action(async move {
                        state
                            .copy_deployment(&id, deployment.name, deployment.server_id, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(deployment))
                },
            ),
        )
        .route(
            "/:id/delete",
            delete(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(DeploymentId{ id }),
                 Query(StopContainerQuery { stop_signal, stop_time })| async move {
                    let deployment = spawn_request_action(async move {
                        state
                            .delete_deployment(&id, &user, stop_signal, stop_time)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(deployment))
                },
            ),
        )
        .route(
            "/update",
            patch(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Json(deployment): Json<Deployment>| async move {
                    let deployment = spawn_request_action(async move {
                        state
                            .update_deployment(deployment, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(deployment))
                },
            ),
        )
        .route(
            "/:id/rename",
            patch(
                |state: StateExtension,
                 user: RequestUserExtension,
                 deployment: Path<DeploymentId>,
                 body: Json<RenameDeploymentBody>| async move {
                    let update = spawn_request_action(async move {
                        state
                            .rename_deployment(&deployment.id, &body.new_name, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(update))
                },
            ),
        )
        .route(
            "/:id/reclone",
            post(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(DeploymentId { id })| async move {
                    let update = spawn_request_action(async move {
                        state
                            .reclone_deployment(&id, &user, true)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(update))
                },
            ),
        )
        .route(
            "/:id/deploy",
            post(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(DeploymentId { id }),
                 Query(StopContainerQuery { stop_signal, stop_time })| async move {
                    let update = spawn_request_action(async move {
                        state
                            .deploy_container(&id, &user, stop_signal, stop_time)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(update))
                },
            ),
        )
        .route(
            "/:id/start_container",
            post(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(DeploymentId { id })| async move {
                    let update = spawn_request_action(async move {
                        state
                            .start_container(&id, &user)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(update))
                },
            ),
        )
        .route(
            "/:id/stop_container",
            post(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(DeploymentId { id }),
                 Query(StopContainerQuery { stop_signal, stop_time })| async move {
                    let update = spawn_request_action(async move {
                        state
                            .stop_container(&id, &user, stop_signal, stop_time)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(update))
                },
            ),
        )
        .route(
            "/:id/remove_container",
            post(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(DeploymentId { id }),
                 Query(StopContainerQuery { stop_signal, stop_time })| async move {
                    let update = spawn_request_action(async move {
                        state
                            .remove_container(&id, &user, stop_signal, stop_time)
                            .await
                            .map_err(handle_anyhow_error)
                    })
                    .await??;
                    response!(Json(update))
                },
            ),
        )
        .route(
            "/:id/pull",
            post(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(DeploymentId { id })| async move {
                    let update = spawn_request_action(async move {
                        state
                            .pull_deployment_repo(&id, &user)
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
                 user: RequestUserExtension,
                 Path(DeploymentId { id }): Path<DeploymentId>| async move {
                    let action_state = state
                        .get_deployment_action_states(id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(action_state))
                },
            ),
        )
        .route(
            "/:id/log",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(DeploymentId { id }),
                 Query(query): Query<GetContainerLogQuery>| async move {
                    let log = state
                        .get_deployment_container_log(&id, &user, query.tail)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(log))
                },
            ),
        )
        .route(
            "/:id/stats",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(DeploymentId { id })| async move {
                    let stats = state
                        .get_deployment_container_stats(&id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(Json(stats))
                },
            ),
        )
        .route(
            "/:id/deployed_version",
            get(
                |state: StateExtension,
                 user: RequestUserExtension,
                 Path(DeploymentId { id })| async move {
                    let version = state
                        .get_deployment_deployed_version(&id, &user)
                        .await
                        .map_err(handle_anyhow_error)?;
                    response!(version)
                },
            ),
        )
}

impl State {
    pub async fn get_deployment_with_container_state(
        &self,
        user: &RequestUser,
        id: &str,
    ) -> anyhow::Result<DeploymentWithContainerState> {
        let deployment = self
            .get_deployment_check_permissions(id, user, PermissionLevel::Read)
            .await?;
        let server = self.db.get_server(&deployment.server_id).await?;
        let (state, container) = match self.periphery.container_list(&server).await {
            Ok(containers) => match containers.into_iter().find(|c| c.name == deployment.name) {
                Some(container) => (container.state, Some(container)),
                None => (DockerContainerState::NotDeployed, None),
            },
            Err(_) => (DockerContainerState::Unknown, None),
        };
        Ok(DeploymentWithContainerState {
            deployment,
            state,
            container,
        })
    }

    async fn list_deployments_with_container_state(
        &self,
        user: &RequestUser,
        query: impl Into<Option<Document>>,
    ) -> anyhow::Result<Vec<DeploymentWithContainerState>> {
        let deployments: Vec<Deployment> = self
            .db
            .deployments
            .get_some(query, None)
            .await
            .context("failed at get all deployments query")?
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
        let mut servers: Vec<Server> = Vec::new();
        for d in &deployments {
            if servers.iter().find(|s| s.id == d.server_id).is_none() {
                servers.push(self.db.get_server(&d.server_id).await?)
            }
        }
        let containers_futures = servers
            .into_iter()
            .map(|server| async { (self.periphery.container_list(&server).await, server.id) });

        let containers = join_all(containers_futures)
            .await
            .into_iter()
            .map(|(container, server_id)| (server_id, container.ok()))
            .collect::<HashMap<_, _>>();
        let deployments_with_containers = deployments
            .into_iter()
            .map(|deployment| {
                let (state, container) = match containers.get(&deployment.server_id).unwrap() {
                    Some(container) => {
                        match container
                            .iter()
                            .find(|c| c.name == deployment.name)
                            .map(|c| c.to_owned())
                        {
                            Some(container) => (container.state, Some(container)),
                            None => (DockerContainerState::NotDeployed, None),
                        }
                    }
                    None => (DockerContainerState::Unknown, None),
                };
                DeploymentWithContainerState {
                    container,
                    deployment,
                    state,
                }
            })
            .collect::<Vec<DeploymentWithContainerState>>();
        Ok(deployments_with_containers)
    }

    async fn get_deployment_action_states(
        &self,
        id: String,
        user: &RequestUser,
    ) -> anyhow::Result<DeploymentActionState> {
        self.get_deployment_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        let action_state = self
            .deployment_action_states
            .lock()
            .await
            .entry(id)
            .or_default()
            .clone();
        Ok(action_state)
    }

    async fn get_deployment_container_log(
        &self,
        id: &str,
        user: &RequestUser,
        tail: Option<u32>,
    ) -> anyhow::Result<Log> {
        let deployment = self
            .get_deployment_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        let server = self.db.get_server(&deployment.server_id).await?;
        let log = self
            .periphery
            .container_log(&server, &deployment.name, tail)
            .await?;
        Ok(log)
    }

    async fn get_deployment_container_stats(
        &self,
        id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<DockerContainerStats> {
        let deployment = self
            .get_deployment_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        let server = self.db.get_server(&deployment.server_id).await?;
        let stats = self
            .periphery
            .container_stats(&server, &deployment.name)
            .await?;
        Ok(stats)
    }

    async fn get_deployment_deployed_version(
        &self,
        id: &str,
        user: &RequestUser,
    ) -> anyhow::Result<String> {
        let deployment = self
            .get_deployment_check_permissions(&id, &user, PermissionLevel::Read)
            .await?;
        if deployment.build_id.is_some() {
            let latest_deploy_update = self
                .db
                .updates
                .find_one(
                    doc! {
                        "target": {
                            "type": "Deployment",
                            "id": id
                        },
                        "operation": Operation::DeployContainer.to_string(),
                        "status": UpdateStatus::Complete.to_string(),
                        "success": true,
                    },
                    FindOneOptions::builder().sort(doc! { "_id": -1 }).build(),
                )
                .await
                .context("failed at query to get latest deploy update from mongo")?;
            if let Some(update) = latest_deploy_update {
                if let Some(version) = update.version {
                    Ok(version.to_string())
                } else {
                    Ok("unknown".to_string())
                }
            } else {
                Ok("unknown".to_string())
            }
        } else {
            let split = deployment
                .docker_run_args
                .image
                .split(':')
                .collect::<Vec<&str>>();
            if let Some(version) = split.get(1) {
                Ok(version.to_string())
            } else {
                Ok("unknown".to_string())
            }
        }
    }
}
