// #![allow(unused)]

use ::helpers::get_socket_addr;
use auth::JwtClient;
use axum::{http::StatusCode, Router};
use state::State;
use termination_signal::tokio::immediate_term_handle;
use tower_http::cors::{Any, CorsLayer};

mod actions;
mod api;
mod auth;
mod cloud;
mod config;
mod helpers;
mod monitoring;
mod state;
mod ws;

type ResponseResult<T> = Result<T, (StatusCode, String)>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("version: v{}", env!("CARGO_PKG_VERSION"));

    let term_signal = immediate_term_handle()?;

    let app = tokio::spawn(async move {
        let (config, spa_router) = config::load();

        println!("starting monitor core on port {}...", config.port);

        let app = Router::new()
            .merge(spa_router)
            .nest("/api", api::router())
            .nest("/auth", auth::router(&config))
            .nest("/ws", ws::router())
            .layer(JwtClient::extension(&config))
            .layer(State::extension(config.clone()).await)
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            );

        println!("started monitor core on port {}", config.port);

        axum::Server::bind(&get_socket_addr(config.port))
            .serve(app.into_make_service())
            .await?;

        anyhow::Ok(())
    });

    tokio::select! {
        res = app => return res?,
        _ = term_signal => {},
    }

    Ok(())
}
