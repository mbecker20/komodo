#[macro_use]
extern crate log;

use axum::{Extension, Router};
use termination_signal::tokio::immediate_term_handle;
use tower_http::cors::{Any, CorsLayer};

mod auth;
mod cloud;
mod config;
mod helpers;
mod listener;
mod monitor;
mod requests;
mod state;
mod ws;

async fn app() -> anyhow::Result<()> {
    let state = state::State::load().await?;

    info!("monitor core version: v{}", env!("CARGO_PKG_VERSION"));

    let socket_addr = state.socket_addr()?;

    let app = Router::new()
        .nest("/auth", auth::router(&state))
        .nest("/read", requests::read::router())
        .nest("/write", requests::write::router())
        .nest("/execute", requests::execute::router())
        .nest("/listener", listener::router())
        .nest("/ws", ws::router())
        .layer(Extension(state))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    info!("starting monitor core on {socket_addr}");

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let term_signal = immediate_term_handle()?;

    let app = tokio::spawn(app());

    tokio::select! {
        res = app => return res?,
        _ = term_signal => {},
    }

    Ok(())
}
