#![allow(unused)]

use axum::Router;
use docker::DockerClient;

mod api;
mod config;
mod helpers;
mod oauth;

#[tokio::main]
async fn main() {
    let (socket_addr, mungos) = config::load().await;

    let app = Router::new().nest("/api", api::router());

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .expect("server crashed");
}
