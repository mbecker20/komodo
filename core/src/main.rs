#![allow(unused)]

use axum::Router;
use docker::DockerClient;

mod api;
mod auth;
mod config;
mod helpers;
mod oauth;

#[tokio::main]
async fn main() {
    let (socket_addr, db_extension) = config::load().await;

    let app = Router::new()
        .nest("/api", api::router())
        .layer(db_extension);

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .expect("server crashed");
}
