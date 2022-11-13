#![allow(unused)]

use std::sync::Arc;

use ::helpers::get_socket_addr;
use axum::{extract::Path, http::StatusCode, routing::get, Extension, Json, Router};
use types::PeripheryConfig;

mod api;
mod config;
mod helpers;

type PeripheryConfigExtension = Extension<Arc<PeripheryConfig>>;

#[tokio::main]
async fn main() {
    let (port, config) = config::load();

    let app = api::router().layer(config.clone());

    axum::Server::bind(&get_socket_addr(port))
        .serve(app.into_make_service())
        .await
        .expect("monitor periphery axum server crashed");
}
