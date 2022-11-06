#![allow(unused)]

use auth::JwtClient;
use axum::Router;
use db::DbClient;
use docker::DockerClient;
use helpers::get_socket_addr;

mod api;
mod auth;
mod config;
mod helpers;

#[tokio::main]
async fn main() {
    let config = config::load().await;

    let app = Router::new()
        .nest("/api", api::router())
        .nest("/auth", auth::router(&config))
        .layer(DbClient::extension((&config).into()).await)
        .layer(JwtClient::extension(&config));

    axum::Server::bind(&get_socket_addr(&config))
        .serve(app.into_make_service())
        .await
        .expect("server crashed");
}
