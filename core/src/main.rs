// #![allow(unused)]

use ::helpers::get_socket_addr;
use auth::JwtClient;
use axum::Router;
use db::DbClient;
use ws::make_update_ws_sender_reciver;

mod api;
mod auth;
mod config;
mod helpers;
mod ws;

#[tokio::main]
async fn main() {
    let config = config::load();

    let (sender, reciever) = make_update_ws_sender_reciver();

    let app = Router::new()
        .nest("/api", api::router())
        .nest("/auth", auth::router(&config))
        .nest("/ws", ws::router(reciever))
        .layer(sender)
        .layer(DbClient::extension(config.mongo.clone()).await)
        .layer(JwtClient::extension(&config));

    axum::Server::bind(&get_socket_addr(config.port))
        .serve(app.into_make_service())
        .await
        .expect("monitor core axum server crashed");
}
