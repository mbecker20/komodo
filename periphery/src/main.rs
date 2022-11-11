#![allow(unused)]

use std::sync::Arc;

use ::helpers::get_socket_addr;
use axum::{extract::Path, http::StatusCode, routing::get, Extension, Json, Router};
use types::PeripherySecrets;

mod api;
mod config;
mod helpers;

use crate::api::*;

type PeripherySecretsExtension = Extension<Arc<PeripherySecrets>>;

#[tokio::main]
async fn main() {
    let (port, secrets) = config::load();

    let app = api::router().layer(secrets);

    println!("starting montior periphery on port {port}");

    axum::Server::bind(&get_socket_addr(port))
        .serve(app.into_make_service())
        .await
        .expect("monitor periphery axum server crashed");
}
