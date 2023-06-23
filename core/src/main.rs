#[macro_use]
extern crate log;

use axum::{Extension, Router};
use termination_signal::tokio::immediate_term_handle;

mod auth;
mod cloud;
mod config;
mod db;
mod helpers;
mod monitoring;
mod requests;
mod state;
mod ws;

async fn app() -> anyhow::Result<()> {
    let state = state::State::load().await?;

    info!("version: v{}", env!("CARGO_PKG_VERSION"));

    let socket_addr = state.socket_addr()?;

    let app = Router::new()
        .nest("/auth", auth::router(&state))
        .nest("/api", requests::api::router())
        .nest("/ws", ws::router())
        .layer(Extension(state));

    info!("starting server on {}", socket_addr);

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
