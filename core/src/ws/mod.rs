use anyhow::anyhow;
use axum::{routing::get, Router};
use types::{PermissionLevel, PermissionsMap};

pub mod update;

pub use update::make_update_ws_sender_reciver;

use self::update::UpdateWsRecieverExtension;

pub fn router(reciever: UpdateWsRecieverExtension) -> Router {
    Router::new()
        .route("/update", get(update::ws_handler))
        .layer(reciever)
}
