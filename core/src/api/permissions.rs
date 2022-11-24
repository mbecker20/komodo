use anyhow::{anyhow, Context};
use axum::{routing::post, Extension, Json, Router};
use db::DbExtension;
use helpers::handle_anyhow_error;
use mungos::{doc, Deserialize, Update};
use types::{PermissionLevel, PermissionsTarget, Server};

use crate::{auth::RequestUserExtension, helpers::get_user_permissions};

#[derive(Deserialize)]
struct PermissionsUpdate {
    user_id: String,
    permission: PermissionLevel,
    target_type: PermissionsTarget,
    target_id: String,
}

pub fn router() -> Router {
    Router::new().route(
        "/add",
        post(|db, user, update| async {
            add_permissions(db, user, update)
                .await
                .map_err(handle_anyhow_error)
        }),
    )
}

async fn add_permissions(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Json(update): Json<PermissionsUpdate>,
) -> anyhow::Result<()> {
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
            let permissions = get_user_permissions(&user.id, &server.permissions);
            if user.is_admin || permissions == PermissionLevel::Write {
                let target_user = db
                    .users
                    .find_one_by_id(&update.user_id)
                    .await
                    .context("failed at find target user query")?
                    .ok_or(anyhow!("failed to find a user with id {}", update.user_id))?;
                if !target_user.enabled {
                    return Err(anyhow!("target user not enabled"));
                }
                db.servers
                    .update_one::<Server>(&update.target_id, Update::Set(doc! {
                        format!("permissions.{}", update.user_id): update.permission.to_string()
                    }))
                    .await?;
                Ok(())
            } else {
                Err(anyhow!("user is not authorized for this action"))
            }
        }
        PermissionsTarget::Deployment => {
            todo!()
        }
        PermissionsTarget::Build => {
            todo!()
        }
    }
}
