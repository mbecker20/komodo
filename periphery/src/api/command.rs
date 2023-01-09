use anyhow::Context;
use axum::{routing::post, Json, Router};
use helpers::{handle_anyhow_error, run_monitor_command};
use types::Command;

use crate::HomeDirExtension;

pub fn router() -> Router {
    Router::new().route(
        "/",
        post(
            |home_dir: HomeDirExtension, Json(Command { path, command })| async move {
                tokio::spawn(async move {
                    let command = if path.is_empty() {
                        command
                    } else {
                        let path = path.replace("~", &home_dir);
                        format!("cd {path} && {command}")
                    };
                    let log = run_monitor_command("run command", command).await;
                    Json(log)
                })
                .await
                .context("failure in spawned task")
                .map_err(handle_anyhow_error)
            },
        ),
    )
}
