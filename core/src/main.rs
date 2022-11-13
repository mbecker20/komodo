#![allow(unused)]

use ::helpers::{docker::DockerClient, get_socket_addr};
use auth::JwtClient;
use axum::{http::StatusCode, Router};
use db::DbClient;

mod api;
mod auth;
mod config;
mod helpers;

type ResponseResult<T> = Result<T, (StatusCode, String)>;

#[tokio::main]
async fn main() {
    let config = config::load();

    let app = Router::new()
        .nest("/api", api::router())
        .nest("/auth", auth::router(&config))
        .layer(JwtClient::extension(&config))
        .layer(DbClient::extension(config.mongo.clone()).await);

    axum::Server::bind(&get_socket_addr(config.port))
        .serve(app.into_make_service())
        .await
        .expect("monitor core axum server crashed");
}
