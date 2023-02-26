use anyhow::{anyhow, Context};
use axum::{routing::post, Extension, Json, Router};
use helpers::handle_anyhow_error;
use mungos::{doc, Deserialize, Document, Serialize};
use types::{
    monitor_timestamp, Build, Deployment, Log, Operation, PermissionLevel, PermissionsTarget,
    Procedure, Server, Update, UpdateStatus, UpdateTarget,
};
use typeshare::typeshare;

use crate::{auth::RequestUserExtension, response, state::StateExtension};

#[typeshare]
#[derive(Serialize, Deserialize)]
struct PermissionsUpdateBody {
    user_id: String,
    permission: PermissionLevel,
    target_type: PermissionsTarget,
    target_id: String,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
struct ModifyUserEnabledBody {
    user_id: String,
    enabled: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
struct ModifyUserCreateServerBody {
    user_id: String,
    create_server_permissions: bool,
}

#[typeshare]
#[derive(Serialize, Deserialize)]
struct ModifyUserCreateBuildBody {
    user_id: String,
    create_build_permissions: bool,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/update",
            post(|state, user, update| async {
                let update = update_permissions(state, user, update)
                    .await
                    .map_err(handle_anyhow_error)?;
                response!(Json(update))
            }),
        )
        .route(
            "/modify_enabled",
            post(|state, user, body| async {
                let update = modify_user_enabled(state, user, body)
                    .await
                    .map_err(handle_anyhow_error)?;
                response!(Json(update))
            }),
        )
        .route(
            "/modify_create_server",
            post(|state, user, body| async {
                let update = modify_user_create_server_permissions(state, user, body)
                    .await
                    .map_err(handle_anyhow_error)?;
                response!(Json(update))
            }),
        )
        .route(
            "/modify_create_build",
            post(|state, user, body| async {
                let update = modify_user_create_build_permissions(state, user, body)
                    .await
                    .map_err(handle_anyhow_error)?;
                response!(Json(update))
            }),
        )
}

async fn update_permissions(
    Extension(state): StateExtension,
    Extension(user): RequestUserExtension,
    Json(permission_update): Json<PermissionsUpdateBody>,
) -> anyhow::Result<Update> {
    if !user.is_admin {
        return Err(anyhow!(
            "user not authorized for this action (is not admin)"
        ));
    }
    let target_user = state
        .db
        .users
        .find_one_by_id(&permission_update.user_id)
        .await
        .context("failed at find target user query")?
        .ok_or(anyhow!(
            "failed to find a user with id {}",
            permission_update.user_id
        ))?;
    if !target_user.enabled {
        return Err(anyhow!("target user not enabled"));
    }
    let mut update = Update {
        operation: Operation::ModifyUserPermissions,
        start_ts: monitor_timestamp(),
        success: true,
        operator: user.id.clone(),
        status: UpdateStatus::Complete,
        ..Default::default()
    };
    let log_text = match permission_update.target_type {
        PermissionsTarget::Server => {
            let server = state
                .db
                .servers
                .find_one_by_id(&permission_update.target_id)
                .await
                .context("failed at find server query")?
                .ok_or(anyhow!(
                    "failed to find a server with id {}",
                    permission_update.target_id
                ))?;
            state
                .db
                .servers
                .update_one::<Server>(
                    &permission_update.target_id,
                    mungos::Update::Set(doc! {
                        format!("permissions.{}", permission_update.user_id): permission_update.permission.to_string()
                    }),
                )
                .await?;
            update.target = UpdateTarget::Server(server.id);
            format!(
                "user {} given {} permissions on server {}",
                target_user.username, permission_update.permission, server.name
            )
        }
        PermissionsTarget::Deployment => {
            let deployment = state
                .db
                .deployments
                .find_one_by_id(&permission_update.target_id)
                .await
                .context("failed at find deployment query")?
                .ok_or(anyhow!(
                    "failed to find a deployment with id {}",
                    permission_update.target_id
                ))?;
            state
                .db
                .deployments
                .update_one::<Deployment>(
                    &permission_update.target_id,
                    mungos::Update::Set(doc! {
                        format!("permissions.{}", permission_update.user_id): permission_update.permission.to_string()
                    }),
                )
                .await?;
            update.target = UpdateTarget::Deployment(deployment.id);
            format!(
                "user {} (id: {}) given {} permissions on deployment {}",
                target_user.username, target_user.id, permission_update.permission, deployment.name
            )
        }
        PermissionsTarget::Build => {
            let build = state
                .db
                .builds
                .find_one_by_id(&permission_update.target_id)
                .await
                .context("failed at find build query")?
                .ok_or(anyhow!(
                    "failed to find a build with id {}",
                    permission_update.target_id
                ))?;
            state
                .db
                .builds
                .update_one::<Build>(
                    &permission_update.target_id,
                    mungos::Update::Set(doc! {
                        format!("permissions.{}", permission_update.user_id): permission_update.permission.to_string()
                    }),
                )
                .await?;
            update.target = UpdateTarget::Build(build.id);
            format!(
                "user {} given {} permissions on build {}",
                target_user.username, permission_update.permission, build.name
            )
        }
        PermissionsTarget::Procedure => {
            let procedure = state
                .db
                .procedures
                .find_one_by_id(&permission_update.target_id)
                .await
                .context("failed at find build query")?
                .ok_or(anyhow!(
                    "failed to find a build with id {}",
                    permission_update.target_id
                ))?;
            state
                .db
                .procedures
                .update_one::<Procedure>(
                    &permission_update.target_id,
                    mungos::Update::Set(doc! {
                        format!("permissions.{}", permission_update.user_id): permission_update.permission.to_string()
                    }),
                )
                .await?;
            update.target = UpdateTarget::Procedure(procedure.id);
            format!(
                "user {} given {} permissions on procedure {}",
                target_user.username, permission_update.permission, procedure.name
            )
        }
    };
    update
        .logs
        .push(Log::simple("modify permissions", log_text));
    update.end_ts = Some(monitor_timestamp());
    update.id = state.add_update(update.clone()).await?;
    Ok(update)
}

async fn modify_user_enabled(
    Extension(state): StateExtension,
    Extension(user): RequestUserExtension,
    Json(ModifyUserEnabledBody { user_id, enabled }): Json<ModifyUserEnabledBody>,
) -> anyhow::Result<Update> {
    if !user.is_admin {
        return Err(anyhow!(
            "user does not have permissions for this action (not admin)"
        ));
    }
    let user = state
        .db
        .users
        .find_one_by_id(&user_id)
        .await
        .context("failed at mongo query to find target user")?
        .ok_or(anyhow!("did not find any user with user_id {user_id}"))?;
    state
        .db
        .users
        .update_one::<Document>(&user_id, mungos::Update::Set(doc! { "enabled": enabled }))
        .await?;
    let update_type = if enabled { "enabled" } else { "disabled" };
    let ts = monitor_timestamp();
    let mut update = Update {
        target: UpdateTarget::System,
        operation: Operation::ModifyUserEnabled,
        logs: vec![Log::simple(
            "modify user enabled",
            format!("{update_type} {} (id: {})", user.username, user.id),
        )],
        start_ts: ts.clone(),
        end_ts: Some(ts),
        status: UpdateStatus::Complete,
        success: true,
        operator: user.id.clone(),
        ..Default::default()
    };
    update.id = state.add_update(update.clone()).await?;
    Ok(update)
}

async fn modify_user_create_server_permissions(
    Extension(state): StateExtension,
    Extension(user): RequestUserExtension,
    Json(ModifyUserCreateServerBody {
        user_id,
        create_server_permissions,
    }): Json<ModifyUserCreateServerBody>,
) -> anyhow::Result<Update> {
    if !user.is_admin {
        return Err(anyhow!(
            "user does not have permissions for this action (not admin)"
        ));
    }
    let user = state
        .db
        .users
        .find_one_by_id(&user_id)
        .await
        .context("failed at mongo query to find target user")?
        .ok_or(anyhow!("did not find any user with user_id {user_id}"))?;
    state
        .db
        .users
        .update_one::<Document>(
            &user_id,
            mungos::Update::Set(doc! { "create_server_permissions": create_server_permissions }),
        )
        .await?;
    let update_type = if create_server_permissions {
        "enabled"
    } else {
        "disabled"
    };
    let ts = monitor_timestamp();
    let mut update = Update {
        target: UpdateTarget::System,
        operation: Operation::ModifyUserCreateServerPermissions,
        logs: vec![Log::simple(
            "modify user create server permissions",
            format!(
                "{update_type} create server permissions for {} (id: {})",
                user.username, user.id
            ),
        )],
        start_ts: ts.clone(),
        end_ts: Some(ts),
        status: UpdateStatus::Complete,
        success: true,
        operator: user.id.clone(),
        ..Default::default()
    };
    update.id = state.add_update(update.clone()).await?;
    Ok(update)
}

async fn modify_user_create_build_permissions(
    Extension(state): StateExtension,
    Extension(user): RequestUserExtension,
    Json(ModifyUserCreateBuildBody {
        user_id,
        create_build_permissions,
    }): Json<ModifyUserCreateBuildBody>,
) -> anyhow::Result<Update> {
    if !user.is_admin {
        return Err(anyhow!(
            "user does not have permissions for this action (not admin)"
        ));
    }
    let user = state
        .db
        .users
        .find_one_by_id(&user_id)
        .await
        .context("failed at mongo query to find target user")?
        .ok_or(anyhow!("did not find any user with user_id {user_id}"))?;
    state
        .db
        .users
        .update_one::<Document>(
            &user_id,
            mungos::Update::Set(doc! { "create_build_permissions": create_build_permissions }),
        )
        .await?;
    let update_type = if create_build_permissions {
        "enabled"
    } else {
        "disabled"
    };
    let ts = monitor_timestamp();
    let mut update = Update {
        target: UpdateTarget::System,
        operation: Operation::ModifyUserCreateBuildPermissions,
        logs: vec![Log::simple(
            "modify user create build permissions",
            format!(
                "{update_type} create build permissions for {} (id: {})",
                user.username, user.id
            ),
        )],
        start_ts: ts.clone(),
        end_ts: Some(ts),
        status: UpdateStatus::Complete,
        success: true,
        operator: user.id.clone(),
        ..Default::default()
    };
    update.id = state.add_update(update.clone()).await?;
    Ok(update)
}
