#[macro_use]
extern crate tracing;

use std::{net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum::Router;
use termination_signal::tokio::immediate_term_handle;
use tower_http::cors::{Any, CorsLayer};

use crate::config::core_config;

mod api;
mod auth;
mod cloud;
mod config;
mod db;
mod helpers;
mod listener;
mod monitor;
mod prune;
mod state;
mod ws;

async fn app() -> anyhow::Result<()> {
  dotenv::dotenv().ok();
  let config = core_config();
  logger::init(config.log_level);
  info!("monitor core version: v{}", env!("CARGO_PKG_VERSION"));

  monitor::spawn_monitor_loop();
  prune::spawn_prune_loop();

  let socket_addr =
    SocketAddr::from_str(&format!("0.0.0.0:{}", core_config().port))
      .context("failed to parse socket addr")?;

  let app = Router::new()
    .nest("/auth", api::auth::router())
    .nest("/read", api::read::router())
    .nest("/write", api::write::router())
    .nest("/execute", api::execute::router())
    .nest("/listener", listener::router())
    .nest("/ws", ws::router())
    .layer(cors()?);

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

fn cors() -> anyhow::Result<CorsLayer> {
  let cors = CorsLayer::new()
    .allow_origin(
      // core_config()
      //     .host
      //     .parse::<HeaderValue>()
      //     .context("failed to parse host into origin")?,
      Any,
    )
    .allow_methods(Any)
    .allow_headers(Any);
  Ok(cors)
}
