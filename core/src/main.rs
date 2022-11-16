#![allow(unused)]

use ::helpers::{docker::DockerClient, get_socket_addr};
use auth::JwtClient;
use axum::{http::StatusCode, Router, routing::get};
use db::DbClient;
use ws::{make_ws_sender_reciver, ws_handler};

mod api;
mod auth;
mod config;
mod helpers;
mod ws;

type ResponseResult<T> = Result<T, (StatusCode, String)>;

#[tokio::main]
async fn main() {
    let config = config::load();

    let (sender, reciever) = make_ws_sender_reciver();

    let app = Router::new()
        .nest("/api", api::router())
        .nest("/auth", auth::router(&config))
        .route("/ws", get(ws_handler))
        .layer(sender)
        .layer(reciever)
        .layer(DbClient::extension(config.mongo.clone()).await)
        .layer(JwtClient::extension(&config));

    axum::Server::bind(&get_socket_addr(config.port))
        .serve(app.into_make_service())
        .await
        .expect("monitor core axum server crashed");
}
