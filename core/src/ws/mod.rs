use anyhow::anyhow;
use axum::{routing::get, Router};
use types::{PermissionLevel, PermissionsMap};

pub mod update;

pub use update::make_update_ws_sender_reciver;

pub fn router() -> Router {
    Router::new().route("/update", get(update::ws_handler))
}

fn user_permissions(user_id: &str, permissions: &PermissionsMap) -> anyhow::Result<()> {
    let permission_level = *permissions
        .get(user_id)
        .ok_or(anyhow!("user has no permissions"))?;
    match permission_level {
        PermissionLevel::None => Err(anyhow!("user has None permission level")),
        _ => Ok(()),
    }
}
