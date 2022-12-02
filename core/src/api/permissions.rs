use anyhow::{anyhow, Context};
use axum::{routing::post, Extension, Json, Router};
use db::DbExtension;
use helpers::handle_anyhow_error;
use mungos::{doc, Deserialize, Document, Update};
use types::{Build, Deployment, PermissionLevel, PermissionsTarget, Server};

use crate::auth::RequestUserExtension;

#[derive(Deserialize)]
struct PermissionsUpdateBody {
    user_id: String,
    permission: PermissionLevel,
    target_type: PermissionsTarget,
    target_id: String,
}

#[derive(Deserialize)]
struct ModifyUserEnabledBody {
    user_id: String,
    enabled: bool,
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/update",
            post(|db, user, update| async {
                update_permissions(db, user, update)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
        .route(
            "/modify_enabled",
            post(|db, user, body| async {
                modify_user_enabled(db, user, body)
                    .await
                    .map_err(handle_anyhow_error)
            }),
        )
}

async fn update_permissions(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Json(update): Json<PermissionsUpdateBody>,
) -> anyhow::Result<String> {
    if !user.is_admin {
        return Err(anyhow!(
            "user not authorized for this action (is not admin)"
        ));
    }
    let target_user = db
        .users
        .find_one_by_id(&update.user_id)
        .await
        .context("failed at find target user query")?
        .ok_or(anyhow!("failed to find a user with id {}", update.user_id))?;
    if !target_user.enabled {
        return Err(anyhow!("target user not enabled"));
    }
    match update.target_type {
        PermissionsTarget::Server => {
            let server = db
                .servers
                .find_one_by_id(&update.target_id)
                .await
                .context("failed at find server query")?
                .ok_or(anyhow!(
                    "failed to find a server with id {}",
                    update.target_id
                ))?;
            db.servers
                .update_one::<Server>(
                    &update.target_id,
                    Update::Set(doc! {
                        format!("permissions.{}", update.user_id): update.permission.to_string()
                    }),
                )
                .await?;
            Ok(format!(
                "user {} given {} permissions on server {}",
                target_user.username, update.permission, server.name
            ))
        }
        PermissionsTarget::Deployment => {
            let deployment = db
                .deployments
                .find_one_by_id(&update.target_id)
                .await
                .context("failed at find deployment query")?
                .ok_or(anyhow!(
                    "failed to find a deployment with id {}",
                    update.target_id
                ))?;
            db.deployments
                .update_one::<Deployment>(
                    &update.target_id,
                    Update::Set(doc! {
                        format!("permissions.{}", update.user_id): update.permission.to_string()
                    }),
                )
                .await?;
            Ok(format!(
                "user {} given {} permissions on deployment {}",
                target_user.username, update.permission, deployment.name
            ))
        }
        PermissionsTarget::Build => {
            let build = db
                .builds
                .find_one_by_id(&update.target_id)
                .await
                .context("failed at find build query")?
                .ok_or(anyhow!(
                    "failed to find a build with id {}",
                    update.target_id
                ))?;
            db.builds
                .update_one::<Build>(
                    &update.target_id,
                    Update::Set(doc! {
                        format!("permissions.{}", update.user_id): update.permission.to_string()
                    }),
                )
                .await?;
            Ok(format!(
                "user {} given {} permissions on build {}",
                target_user.username, update.permission, build.name
            ))
        }
    }
}

async fn modify_user_enabled(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Json(ModifyUserEnabledBody { user_id, enabled }): Json<ModifyUserEnabledBody>,
) -> anyhow::Result<()> {
    if !user.is_admin {
        return Err(anyhow!(
            "user does not have permissions for this action (not admin)"
        ));
    }
    db.users
        .find_one_by_id(&user_id)
        .await
        .context("failed at mongo query to find target user")?
        .ok_or(anyhow!("did not find any user with user_id {user_id}"))?;
    db.users
        .update_one::<Document>(&user_id, Update::Set(doc! { "enabled": enabled }))
        .await?;
    Ok(())
}
