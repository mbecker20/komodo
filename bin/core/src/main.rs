#[macro_use]
extern crate log;

use anyhow::Context;
use axum::{Extension, Router};
use termination_signal::tokio::immediate_term_handle;

mod api;
mod auth;
mod cloud;
mod config;
mod helpers;
mod listener;
mod monitor;
mod state;
mod ws;

async fn app() -> anyhow::Result<()> {
  let state = state::State::load().await?;

  info!("monitor core version: v{}", env!("CARGO_PKG_VERSION"));

  let socket_addr = state.socket_addr()?;

  let app = Router::new()
    .nest("/auth", auth::router(&state))
    .nest("/read", api::read::router())
    .nest("/write", api::write::router())
    .nest("/execute", api::execute::router())
    .nest("/listener", listener::router())
    .nest("/ws", ws::router())
    .layer(state.cors()?)
    .layer(Extension(state));

  info!("starting monitor core on {socket_addr}");

  let listener = tokio::net::TcpListener::bind(&socket_addr)
    .await
    .context("failed to bind to tcp listener")?;

  axum::serve(listener, app).await.context("server crashed")?;

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
