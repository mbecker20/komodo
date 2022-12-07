// #![allow(unused)]

use ::helpers::get_socket_addr;
use auth::JwtClient;
use axum::Router;
use state::State;

mod actions;
mod api;
mod auth;
mod config;
mod helpers;
mod state;
mod ws;

#[tokio::main]
async fn main() {
    let config = config::load();

    let app = Router::new()
        .nest("/api", api::router())
        .nest("/auth", auth::router(&config))
        .nest("/ws", ws::router())
        .layer(JwtClient::extension(&config))
        .layer(State::extension(config.clone()).await);

    axum::Server::bind(&get_socket_addr(config.port))
        .serve(app.into_make_service())
        .await
        .expect("monitor core axum server crashed");
}
