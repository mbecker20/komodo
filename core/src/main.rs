#![allow(unused)]

use ::helpers::docker::DockerClient;
use auth::JwtClient;
use axum::{http::StatusCode, Router};
use db::DbClient;

mod api;
mod auth;
mod config;
mod helpers;

use crate::helpers::get_socket_addr;

type ResponseResult<T> = Result<T, (StatusCode, String)>;

#[tokio::main]
async fn main() {
    let config = config::load();

    let app = Router::new()
        .nest("/api", api::router())
        .nest("/auth", auth::router(&config))
        .layer(JwtClient::extension(&config))
        .layer(DbClient::extension(config.mongo.clone()).await);

    println!("starting monitor_core on localhost:{}", config.port);

    axum::Server::bind(&get_socket_addr(&config))
        .serve(app.into_make_service())
        .await
        .expect("monitor core axum server crashed");
}
