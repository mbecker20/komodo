#[macro_use]
extern crate tracing;

use std::{net::SocketAddr, str::FromStr};

use anyhow::Context;
use axum::Router;
use termination_signal::tokio::immediate_term_handle;
use tower_http::{
  cors::{Any, CorsLayer},
  services::{ServeDir, ServeFile},
};

use crate::config::{core_config, frontend_path};

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
  logger::init(&config.logging)?;
  info!("monitor core version: v{}", env!("CARGO_PKG_VERSION"));
  info!("config: {:?}", config.sanitized());

  // Spawn monitoring loops
  monitor::spawn_monitor_loop();
  prune::spawn_prune_loop();

  // Setup static frontend services
  let frontend_path = frontend_path();
  let frontend_index =
    ServeFile::new(format!("{frontend_path}/index.html"));
  let serve_dir = ServeDir::new(frontend_path)
    .not_found_service(frontend_index.clone());

  let app = Router::new()
    .nest("/auth", api::auth::router())
    .nest("/read", api::read::router())
    .nest("/write", api::write::router())
    .nest("/execute", api::execute::router())
    .nest("/listener", listener::router())
    .nest("/ws", ws::router())
    .nest_service("/", serve_dir)
    .fallback_service(frontend_index)
    .layer(cors()?);

  let socket_addr =
    SocketAddr::from_str(&format!("0.0.0.0:{}", core_config().port))
      .context("failed to parse socket addr")?;

  let listener = tokio::net::TcpListener::bind(&socket_addr)
    .await
    .context("failed to bind to tcp listener")?;

  info!("monitor core listening on {socket_addr}");

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
    .allow_origin(Any)
    .allow_methods(Any)
    .allow_headers(Any);
  Ok(cors)
}
