use anyhow::Context;
use async_timing_util::unix_timestamp_ms;
use axum::{routing::post, Extension, Json, Router};
use db::DbExtension;
use helpers::handle_anyhow_error;
use types::{Deployment, Operation, PermissionLevel, Update, UpdateTarget};

use crate::{auth::RequestUserExtension, ws::update};

use super::add_update;

pub fn router() -> Router {
    Router::new().route(
        "/create",
        post(|db, user, update_ws, deployment| async {
            create(db, user, update_ws, deployment)
                .await
                .map_err(handle_anyhow_error)
        }),
    )
}

async fn create(
    Extension(db): DbExtension,
    Extension(user): RequestUserExtension,
    Extension(update_ws): update::UpdateWsSenderExtension,
    Json(mut deployment): Json<Deployment>,
) -> anyhow::Result<()> {
    deployment.permissions = [(user.id.clone(), PermissionLevel::Write)]
        .into_iter()
        .collect();
    let start_ts = unix_timestamp_ms() as i64;
    let deployment_id = db
        .deployments
        .create_one(deployment)
        .await
        .context("failed to add server to db")?;
    let update = Update {
        target: UpdateTarget::Deployment(deployment_id),
        operation: Operation::CreateDeployment,
        start_ts,
        end_ts: unix_timestamp_ms() as i64,
        operator: user.id.clone(),
        ..Default::default()
    };
    add_update(update, &db, &update_ws).await
}
