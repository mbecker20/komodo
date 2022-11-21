use anyhow::anyhow;
use axum::{routing::get, Router};
use types::{PermissionLevel, PermissionsMap};

pub mod update;

pub use update::make_update_ws_sender_reciver;

pub fn router() -> Router {
    Router::new().route("/update", get(update::ws_handler))
}

fn user_permissions(user_id: &str, permissions: &PermissionsMap) -> PermissionLevel {
    *permissions.get(user_id).unwrap_or(&PermissionLevel::None)
}
